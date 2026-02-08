use crate::api::ApiClient;
use crate::config::{Config, ProjectConfig};
use crate::storage::EnvStorage;
use crate::utils::i18n::get_translations;
use anyhow::Result;
use colored::*;
use std::collections::HashMap;
use std::process::Command;

pub async fn execute(
    project: Option<String>,
    environment: Option<String>,
    command_args: Vec<String>,
) -> Result<()> {
    let config = Config::load()?;
    let t = get_translations(&config.language);

    // Determine project (we need both ID for display and slug for API)
    let (project_id, project_slug, env_name) = if let Some(proj) = project {
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

    if command_args.is_empty() {
        anyhow::bail!("No command specified");
    }

    println!("{}", t.run.executing.cyan());
    println!("{}", format!("  Project: {}", project_id).bright_black());
    println!("{}", format!("  Environment: {}", env_name).bright_black());
    println!();

    let token = config.get_token()?;
    let api_client = ApiClient::from_config(&config);

    // Try to get from shared memory first (faster)
    let vars = {
        let storage = EnvStorage::new()?;
        if let Some(data) = storage.read()? {
            if data.project_id == project_id && data.environment == env_name {
                println!("{}", t.run.using_cached.bright_black());
                data.variables
            } else {
                fetch_vars(&api_client, &token, &project_slug, &env_name).await?
            }
        } else {
            fetch_vars(&api_client, &token, &project_slug, &env_name).await?
        }
    };

    println!(
        "{}",
        t.run
            .loaded_vars
            .replace("{}", &vars.len().to_string())
            .green()
    );
    println!();
    println!("{}", "─".repeat(50).bright_black());
    println!();

    // Execute command with environment variables
    let cmd_name = &command_args[0];
    let cmd_args = &command_args[1..];

    let mut cmd = Command::new(cmd_name);
    cmd.args(cmd_args);

    // Inject environment variables
    for (key, value) in vars {
        cmd.env(key, value);
    }

    let status = cmd.status()?;

    println!();
    println!("{}", "─".repeat(50).bright_black());

    if status.success() {
        println!("{}", t.run.success.green());
        Ok(())
    } else {
        anyhow::bail!(t.run.failure.replace("{}", &status.to_string()));
    }
}

async fn fetch_vars(
    api_client: &ApiClient,
    token: &str,
    project_slug: &str,
    env_name: &str,
) -> Result<HashMap<String, String>> {
    let env = api_client
        .get_environment(token, project_slug, env_name)
        .await?;

    let vars: HashMap<String, String> = env
        .variables
        .into_iter()
        .map(|v| (v.key, v.value))
        .collect();

    Ok(vars)
}
