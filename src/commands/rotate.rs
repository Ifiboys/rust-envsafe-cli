use anyhow::Result;
use colored::*;
use crate::config::{Config, ProjectConfig};
use crate::api::ApiClient;
use crate::rotation::SecretRotator;

pub async fn enable(interval: u32, exclude: Vec<String>) -> Result<()> {
    let config = Config::load()?;
    let api_client = ApiClient::from_config(&config);
    let mut rotator = SecretRotator::new(api_client, config.clone());
    
    rotator.enable_rotation(interval, exclude)?;
    
    Ok(())
}

pub async fn disable() -> Result<()> {
    let config = Config::load()?;
    let api_client = ApiClient::from_config(&config);
    let mut rotator = SecretRotator::new(api_client, config.clone());
    
    rotator.disable_rotation()?;
    
    Ok(())
}

pub async fn status() -> Result<()> {
    let config = Config::load()?;
    let api_client = ApiClient::from_config(&config);
    let rotator = SecretRotator::new(api_client, config);
    
    rotator.show_status()?;
    
    Ok(())
}

pub async fn now(vars: Vec<String>) -> Result<()> {
    let config = Config::load()?;
    
    // Get current project
    let project_id = if let Some(local_config) = ProjectConfig::load()? {
        local_config.project_id
    } else if let Some(proj) = &config.current_project {
        proj.clone()
    } else {
        anyhow::bail!("No project configured. Run 'envsafe init' first");
    };
    
    let api_client = ApiClient::from_config(&config);
    let mut rotator = SecretRotator::new(api_client, config);
    
    println!("{}", "🔄 Forcing immediate rotation...".cyan());
    println!();
    
    let specific = if vars.is_empty() {
        None
    } else {
        Some(vars)
    };
    
    let count = rotator.rotate_secrets(&project_id, "production", specific).await?;
    
    if count > 0 {
        println!();
        println!("{}", "⚠️  Important:".yellow().bold());
        println!("{}", "  - Update your services with new secrets".yellow());
        println!("{}", "  - Restart affected containers/services".yellow());
        println!("{}", "  - Run 'envsafe pull' to get updated values".yellow());
    }
    
    Ok(())
}
