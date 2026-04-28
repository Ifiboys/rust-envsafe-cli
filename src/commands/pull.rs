use crate::api::ApiClient;
use crate::config::{Config, ProjectConfig};
use crate::storage::{EnvStorage, SharedEnvData};
use anyhow::Result;
use chrono::Utc;
use colored::*;
use std::collections::HashMap;
use std::fs;

pub async fn execute(
    project: Option<String>,
    environment: Option<String>,
    output_file: &str,
) -> Result<()> {
    let config = Config::load()?;

    // Determine project (we need both ID for display and slug for API)
    let (project_id, project_slug, env_name) = if let Some(proj) = project {
        // When provided as argument, assume it's a slug
        let env = environment.unwrap_or_else(|| "development".to_string());
        (proj.clone(), proj, env)
    } else if let Some(local_config) = ProjectConfig::load()? {
        let env = environment.unwrap_or_else(|| "development".to_string());
        let slug = local_config
            .project_slug
            .clone()
            .unwrap_or(local_config.project_id.clone());
        (local_config.project_id, slug, env)
    } else {
        anyhow::bail!("No project specified. Run 'envsafe init' or provide project name");
    };

    println!("{}", "📥 Pulling environment variables...".cyan());
    println!("{}", format!("  Project: {}", project_id).bright_black());
    println!("{}", format!("  Environment: {}", env_name).bright_black());

    let token = config.get_token()?;
    let api_client = ApiClient::from_config(&config);

    // Fetch environment variables (use slug for API)
    let env = api_client
        .get_environment(&token, &project_slug, &env_name)
        .await?;

    // Read existing .env file if it exists
    let mut existing_vars: HashMap<String, String> = HashMap::new();
    let mut local_only_vars: Vec<(String, String)> = Vec::new();

    if std::path::Path::new(output_file).exists() {
        let existing_content = fs::read_to_string(output_file)?;
        for line in existing_content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                existing_vars.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }

    // Merge: API variables override, but keep local-only variables
    let mut merged_vars: HashMap<String, String> = HashMap::new();
    let api_keys: std::collections::HashSet<String> =
        env.variables.iter().map(|v| v.key.clone()).collect();

    // Add API variables (they take precedence)
    for var in &env.variables {
        merged_vars.insert(var.key.clone(), var.value.clone());
    }

    // Keep local variables that are not in API
    for (key, value) in existing_vars {
        if !api_keys.contains(&key) {
            local_only_vars.push((key.clone(), value.clone()));
            merged_vars.insert(key, value);
        }
    }

    // Write merged content to file
    let mut content = String::new();
    content.push_str(&format!("# EnvSafe - {}\n", project_id));
    content.push_str(&format!("# Environment: {}\n", env_name));
    content.push_str(&format!("# Updated: {}\n", Utc::now().to_rfc3339()));
    content.push_str("# Variables from EnvSafe (managed remotely)\n\n");

    // Write API variables first
    for var in &env.variables {
        content.push_str(&format!("{}={}\n", var.key, var.value));
    }

    // Write local-only variables separately
    let local_vars_count = local_only_vars.len();
    if !local_only_vars.is_empty() {
        content.push_str("\n# Local variables (not managed by EnvSafe)\n");
        for (key, value) in local_only_vars {
            content.push_str(&format!("{}={}\n", key, value));
        }
    }

    fs::write(output_file, &content)?;

    // Update shared memory
    let mut storage = EnvStorage::new()?;
    let vars_map: HashMap<String, String> = env
        .variables
        .iter()
        .map(|v| (v.key.clone(), v.value.clone()))
        .collect();

    let current_version = storage.get_version()?;
    let data = SharedEnvData {
        version: current_version + 1,
        project_id: project_id.clone(),
        environment: env_name.clone(),
        variables: vars_map,
        last_updated: Utc::now().to_rfc3339(),
    };

    storage.write(&data)?;

    println!(
        "{}",
        format!(
            "✓ Pulled {} variables from EnvSafe to {}",
            env.variables.len(),
            output_file
        )
        .green()
    );

    if local_vars_count > 0 {
        println!(
            "{}",
            format!("✓ Preserved {} local variables", local_vars_count).green()
        );
    }

    println!("{}", "✓ Updated shared memory".green());

    Ok(())
}
