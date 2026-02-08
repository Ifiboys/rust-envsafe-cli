#![allow(dead_code)]

use crate::api::ApiClient;
use crate::config::Config;
use anyhow::Result;
use colored::Colorize;

/// M2M (Machine-to-Machine) authentication
/// Used for CI/CD pipelines and automated services
pub async fn execute(
    token: Option<String>,
    workspace_id: Option<String>,
    project_id: Option<String>,
) -> Result<()> {
    println!("{}", "🤖 M2M Authentication".cyan().bold());
    println!();

    // Get token from argument or environment variable
    let m2m_token = if let Some(t) = token {
        t
    } else if let Ok(env_token) = std::env::var("ENVSAFE_M2M_TOKEN") {
        env_token
    } else if let Ok(env_token) = std::env::var("ENVSAFE_TOKEN") {
        env_token
    } else {
        anyhow::bail!(
            "M2M token required. Provide via --token or ENVSAFE_M2M_TOKEN environment variable"
        );
    };

    // Validate token with API
    let mut config = Config::load()?;
    let api_client = ApiClient::from_config(&config);

    println!("{}", "Validating M2M token...".dimmed());

    let user = api_client.get_user(&m2m_token).await?;

    println!("{}", format!("✓ Authenticated as: {}", user.name).green());

    // Save token
    config.set_token(m2m_token.clone())?;

    // Optionally set workspace and project
    if let Some(ws_id) = workspace_id.or_else(|| std::env::var("ENVSAFE_WORKSPACE_ID").ok()) {
        config.current_workspace = Some(ws_id.clone());
        println!("{}", format!("✓ Workspace set: {}", ws_id).green());
    }

    if let Some(proj_id) = project_id.or_else(|| std::env::var("ENVSAFE_PROJECT_ID").ok()) {
        config.current_project = Some(proj_id.clone());
        println!("{}", format!("✓ Project set: {}", proj_id).green());
    }

    config.save()?;

    println!();
    println!("{}", "✓ M2M authentication successful".green().bold());
    println!();
    println!(
        "{}",
        "You can now use CLI commands non-interactively:".dimmed()
    );
    println!("{}", "  envsafe pull --prod".dimmed());
    println!("{}", "  envsafe run --prod -- your-command".dimmed());

    Ok(())
}

/// Check if running in M2M mode (non-interactive)
pub fn is_m2m_mode() -> bool {
    std::env::var("ENVSAFE_M2M_MODE").is_ok()
        || std::env::var("CI").is_ok()
        || std::env::var("GITHUB_ACTIONS").is_ok()
        || std::env::var("GITLAB_CI").is_ok()
        || std::env::var("JENKINS_URL").is_ok()
        || std::env::var("BITBUCKET_BUILD_NUMBER").is_ok()
}

/// Get M2M token from environment
pub fn get_m2m_token() -> Option<String> {
    std::env::var("ENVSAFE_M2M_TOKEN")
        .or_else(|_| std::env::var("ENVSAFE_TOKEN"))
        .ok()
}
