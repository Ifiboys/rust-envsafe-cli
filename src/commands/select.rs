use anyhow::Result;
use colored::*;
use crate::config::{Config, ProjectConfig};
use crate::api::ApiClient;

pub async fn execute(project_name: &str) -> Result<()> {
    let config = Config::load()?;
    let token = config.get_token()?;
    
    let workspace_id = config.current_workspace
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No workspace linked. Run 'envsafe link' first"))?;
    
    let api_client = ApiClient::from_config(&config);
    let workspace_slug = config.get_workspace_slug()?;
    let projects = api_client.get_projects(&token, &workspace_slug).await?;
    
    let project = projects
        .iter()
        .find(|p| p.name == project_name || p.id == project_name)
        .ok_or_else(|| anyhow::anyhow!("Project '{}' not found", project_name))?;
    
    // Save to local config
    let project_config = ProjectConfig {
        workspace_id: workspace_id.clone(),
        project_id: project.id.clone(),
        project_name: project.name.clone(),
        project_slug: project.slug.clone(),
    };
    
    project_config.save()?;
    
    println!("{}", format!("✓ Selected project: {}", project.name).green());
    
    Ok(())
}
