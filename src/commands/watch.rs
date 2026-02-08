use crate::api::ApiClient;
use crate::config::{Config, ProjectConfig};
use crate::watcher::EnvWatcher;
use anyhow::Result;
use colored::*;

pub async fn execute(
    project: Option<String>,
    environment: Option<String>,
    _file_path: &str,
) -> Result<()> {
    let config = Config::load()?;

    let (project_id, env_name) = if let Some(proj) = project {
        let env = environment.unwrap_or_else(|| "development".to_string());
        (proj, env)
    } else if let Some(local_config) = ProjectConfig::load()? {
        let env = environment.unwrap_or_else(|| "development".to_string());
        (local_config.project_id, env)
    } else {
        anyhow::bail!("No project specified. Run 'envsafe init' or provide project name");
    };

    println!("{}", "👁️  EnvSafe Watch Mode".cyan().bold());
    println!();
    println!("{}", format!("  Project: {}", project_id).bright_black());
    println!("{}", format!("  Environment: {}", env_name).bright_black());
    println!();
    println!("{}", "🔄 Starting bidirectional sync...".cyan());
    println!("{}", "  - Remote changes → Local file".bright_black());
    println!("{}", "  - Local file → Remote".bright_black());
    println!();

    let api_client = ApiClient::from_config(&config);
    let mut watcher = EnvWatcher::new(api_client, config)?;

    println!("{}", "Press Ctrl+C to stop watching".bright_black());
    println!();

    // Start remote monitoring loop
    watcher.watch_remote(&project_id, &env_name).await?;

    Ok(())
}
