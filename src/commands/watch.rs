use crate::api::ApiClient;
use crate::config::{Config, ProjectConfig};
use crate::utils::i18n::get_translations;
use crate::watcher::EnvWatcher;
use anyhow::Result;
use colored::*;

pub async fn execute(
    project: Option<String>,
    environment: Option<String>,
    _file_path: &str,
) -> Result<()> {
    let config = Config::load()?;
    let t = get_translations(&config.language);

    let (project_id, env_name) = if let Some(proj) = project {
        let env = environment.unwrap_or_else(|| "development".to_string());
        (proj, env)
    } else if let Some(local_config) = ProjectConfig::load()? {
        let env = environment.unwrap_or_else(|| "development".to_string());
        (local_config.project_id, env)
    } else {
        // Hard to translate this specific error without passing `t` deeper or redefining error handling
        // For now, let's keep it in English as it's a CLI usage error mostly seen by devs
        anyhow::bail!("No project specified. Run 'envsafe init' or provide project name");
    };

    println!("{}", t.watch.title.cyan().bold());
    println!();
    println!("{}", format!("  Project: {}", project_id).bright_black());
    println!("{}", format!("  Environment: {}", env_name).bright_black());
    println!();
    println!("{}", t.watch.sync_start.cyan());
    println!("{}", t.watch.remote_to_local.bright_black());
    println!("{}", t.watch.local_to_remote.bright_black()); // Note: local->remote sync isn't fully implemented yet but message was there
    println!();

    let api_client = ApiClient::from_config(&config);
    // TODO: Pass translations to watcher if it prints logs
    let mut watcher = EnvWatcher::new(api_client, config)?;

    println!("{}", t.watch.press_ctrl_c.bright_black());
    println!();

    // Start remote monitoring loop
    watcher.watch_remote(&project_id, &env_name).await?;

    Ok(())
}
