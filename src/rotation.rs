#![allow(dead_code)]

use crate::api::{ApiClient, EnvVariable};
use crate::config::Config;
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationPolicy {
    pub enabled: bool,
    pub interval_days: u32,
    pub exclude_vars: HashSet<String>,
    pub last_rotation: Option<DateTime<Utc>>,
}

pub struct SecretRotator {
    api_client: ApiClient,
    config: Config,
}

impl SecretRotator {
    pub fn new(api_client: ApiClient, config: Config) -> Self {
        Self { api_client, config }
    }

    /// Check if rotation is needed
    pub fn needs_rotation(&self) -> bool {
        if !self.config.rotation.enabled {
            return false;
        }

        if let Some(last) = &self.config.rotation.last_rotation {
            if let Ok(last_date) = DateTime::parse_from_rfc3339(last) {
                let now = Utc::now();
                let elapsed = now.signed_duration_since(last_date.with_timezone(&Utc));
                let interval = Duration::days(self.config.rotation.interval_days as i64);

                return elapsed >= interval;
            }
        }

        true
    }

    /// Generate a new secret value
    pub fn generate_secret(&self, old_value: &str, key: &str) -> String {
        // Create a deterministic but unique new value
        let timestamp = Utc::now().timestamp();
        let input = format!("{}:{}:{}", key, old_value, timestamp);

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();

        // Convert to hex string
        hex::encode(result)
    }

    /// Rotate secrets for a project/environment
    pub async fn rotate_secrets(
        &mut self,
        project_id: &str,
        environment: &str,
        specific_vars: Option<Vec<String>>,
    ) -> Result<usize> {
        println!("{}", "🔄 Starting secret rotation...".cyan());

        let token = self.config.get_token()?;
        let env = self
            .api_client
            .get_environment(&token, project_id, environment)
            .await?;

        let exclude_set: HashSet<String> =
            self.config.rotation.exclude_vars.iter().cloned().collect();

        let mut rotated_count = 0;
        let mut new_variables = Vec::new();

        for var in env.variables {
            let should_rotate = if let Some(ref specific) = specific_vars {
                // If specific vars provided, only rotate those
                specific.contains(&var.key)
            } else {
                // Otherwise, rotate all except excluded
                !exclude_set.contains(&var.key) && !self.is_excluded_by_pattern(&var.key)
            };

            if should_rotate {
                let new_value = self.generate_secret(&var.value, &var.key);

                println!(
                    "  {} {} → {}",
                    "↻".yellow(),
                    var.key.bright_white(),
                    "••••••••".bright_black()
                );

                new_variables.push(EnvVariable {
                    key: var.key,
                    value: new_value,
                    updated_at: Some(Utc::now().to_rfc3339()),
                });

                rotated_count += 1;
            } else {
                new_variables.push(var);
            }
        }

        if rotated_count > 0 {
            // Update on server
            self.api_client
                .update_variables(&token, project_id, environment, new_variables)
                .await?;

            // Update last rotation time
            self.config.rotation.last_rotation = Some(Utc::now().to_rfc3339());
            self.config.save()?;

            println!(
                "{}",
                format!("✓ Rotated {} secret(s)", rotated_count).green()
            );
        } else {
            println!("{}", "ℹ No secrets to rotate".bright_black());
        }

        Ok(rotated_count)
    }

    /// Check if a variable should be excluded by pattern
    fn is_excluded_by_pattern(&self, key: &str) -> bool {
        // Don't rotate certain common non-secret env vars
        let patterns = [
            "NODE_ENV",
            "PORT",
            "HOST",
            "ENVIRONMENT",
            "DEBUG",
            "LOG_LEVEL",
            "_URL",  // URLs typically don't need rotation
            "_PATH", // Paths typically don't need rotation
        ];

        patterns.iter().any(|pattern| {
            if pattern.starts_with('_') {
                key.ends_with(pattern)
            } else {
                key == *pattern
            }
        })
    }

    /// Enable rotation
    pub fn enable_rotation(&mut self, interval_days: u32, exclude_vars: Vec<String>) -> Result<()> {
        self.config.rotation.enabled = true;
        self.config.rotation.interval_days = interval_days;
        self.config.rotation.exclude_vars = exclude_vars;
        self.config.save()?;

        println!("{}", "✓ Secret rotation enabled".green());
        println!(
            "{}",
            format!("  Interval: {} days", interval_days).bright_black()
        );

        Ok(())
    }

    /// Disable rotation
    pub fn disable_rotation(&mut self) -> Result<()> {
        self.config.rotation.enabled = false;
        self.config.save()?;

        println!("{}", "✓ Secret rotation disabled".yellow());

        Ok(())
    }

    /// Show rotation status
    pub fn show_status(&self) -> Result<()> {
        println!("{}", "🔐 Secret Rotation Status".cyan().bold());
        println!();

        if self.config.rotation.enabled {
            println!("  Status: {}", "Enabled".green());
            println!(
                "  Interval: {} days",
                format!("{}", self.config.rotation.interval_days).bright_black()
            );

            if let Some(last) = &self.config.rotation.last_rotation {
                println!("  Last rotation: {}", last);

                if let Ok(last_date) = DateTime::parse_from_rfc3339(last) {
                    let now = Utc::now();
                    let elapsed = now.signed_duration_since(last_date.with_timezone(&Utc));
                    let remaining =
                        Duration::days(self.config.rotation.interval_days as i64) - elapsed;

                    if remaining.num_days() > 0 {
                        println!("  Next rotation: in {} days", remaining.num_days());
                    } else {
                        println!("  Next rotation: {}", "Now (overdue)".yellow());
                    }
                }
            } else {
                println!("  Last rotation: {}", "Never".bright_black());
            }

            if !self.config.rotation.exclude_vars.is_empty() {
                println!("  Excluded variables:");
                for var in &self.config.rotation.exclude_vars {
                    println!("    - {}", var.bright_black());
                }
            }
        } else {
            println!("  Status: {}", "Disabled".bright_black());
        }

        Ok(())
    }
}

/// Docker integration helpers
pub mod docker {
    use super::*;

    /// Generate Docker secrets file for rotation
    pub fn generate_docker_secrets(variables: &[EnvVariable], output_path: &str) -> Result<()> {
        use std::fs;

        let mut content = String::new();
        content.push_str("# Auto-generated by EnvSafe\n");
        content.push_str(&format!("# Generated at: {}\n", Utc::now().to_rfc3339()));
        content.push_str("# WARNING: These secrets will be rotated periodically\n\n");

        for var in variables {
            content.push_str(&format!("{}={}\n", var.key, var.value));
        }

        fs::write(output_path, content)?;

        println!(
            "{}",
            format!("✓ Docker secrets file generated: {}", output_path).green()
        );

        Ok(())
    }

    /// Create Docker Compose override with secrets
    pub fn generate_docker_compose_override(
        service_name: &str,
        env_file: &str,
        output_path: &str,
    ) -> Result<()> {
        use std::fs;

        let content = format!(
            r#"# Auto-generated by EnvSafe
# Generated at: {}

version: '3.8'

services:
  {}:
    env_file:
      - {}
"#,
            Utc::now().to_rfc3339(),
            service_name,
            env_file
        );

        fs::write(output_path, content)?;

        println!(
            "{}",
            format!("✓ Docker Compose override generated: {}", output_path).green()
        );

        Ok(())
    }
}
