use anyhow::Result;
use colored::*;
use crate::config::Config;
use crate::api::ApiClient;

pub async fn execute() -> Result<()> {
    let config = Config::load()?;
    let token = config.get_token()?;
    let api_client = ApiClient::from_config(&config);
    
    println!("{}", "📁 All Accessible Projects".cyan().bold());
    println!();
    
    let workspaces = api_client.get_workspaces(&token).await?;
    
    for workspace in workspaces {
        println!("  {} {}", "📦".bright_blue(), workspace.name.bright_white().bold());
        
        let projects = api_client.get_projects(&token, &workspace.id).await?;
        
        if projects.is_empty() {
            println!("    {}", "No projects".bright_black());
        } else {
            for project in projects {
                println!("    {} {}", "•".cyan(), project.name);
                println!("      {}", project.id.bright_black());
            }
        }
        
        println!();
    }
    
    Ok(())
}
