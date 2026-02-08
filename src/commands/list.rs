use anyhow::Result;
use colored::*;
use crate::config::Config;
use crate::api::ApiClient;

pub async fn execute() -> Result<()> {
    let config = Config::load()?;
    let token = config.get_token()?;
    
    let workspace_id = config.current_workspace
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No workspace linked. Run 'envsafe link' first"))?;
    
    let api_client = ApiClient::from_config(&config);
    let projects = api_client.get_projects(&token, workspace_id).await?;
    
    println!("{}", "📋 Projects".cyan().bold());
    println!();
    
    if projects.is_empty() {
        println!("{}", "  No projects found".bright_black());
        return Ok(());
    }
    
    for project in projects {
        println!("  {} {}", "•".cyan(), project.name.bright_white());
        println!("    {}", project.id.bright_black());
    }
    
    Ok(())
}
