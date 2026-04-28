#![allow(deprecated)]

use crate::api::{ApiClient, EnvVariable};
use crate::config::{Config, ProjectConfig};
use anyhow::Result;
use chrono::Utc;
use colored::Colorize;

pub async fn execute(
    project: Option<String>,
    environment: Option<String>,
    file_path: &str,
) -> Result<()> {
    let config = Config::load()?;

    // Determine project
    // Determine project
    let (identifier, env_name) = if let Some(proj) = project {
        let env = environment.unwrap_or_else(|| "development".to_string());
        (proj, env)
    } else if let Some(local_config) = ProjectConfig::load()? {
        let env = environment.unwrap_or_else(|| "development".to_string());
        let id = local_config
            .project_slug
            .clone()
            .unwrap_or(local_config.project_id);
        (id, env)
    } else {
        anyhow::bail!("No project specified. Run 'envsafe init' or provide project name");
    };

    println!("{}", "📤 Pushing environment variables...".cyan());
    println!("{}", format!("  Project: {}", identifier).bright_black());
    println!("{}", format!("  Environment: {}", env_name).bright_black());
    println!("{}", format!("  File: {}", file_path).bright_black());

    // Read .env file
    let variables: Vec<EnvVariable> = dotenv::from_path_iter(file_path)?
        .filter_map(|item| item.ok())
        .map(|(key, value)| EnvVariable {
            key,
            value,
            updated_at: Some(Utc::now().to_rfc3339()),
        })
        .collect();

    if variables.is_empty() {
        println!("{}", "⚠️  No variables found in file".yellow());
        return Ok(());
    }

    let token = config.get_token()?;
    let api_client = ApiClient::from_config(&config);

    // Push to server
    api_client
        .update_variables(&token, &identifier, &env_name, variables.clone())
        .await?;

    println!(
        "{}",
        format!("✓ Pushed {} variables", variables.len()).green()
    );

    Ok(())
}
