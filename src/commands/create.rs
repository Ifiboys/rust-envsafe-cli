use anyhow::Result;
use colored::*;
use dialoguer::Input;
use crate::config::{Config, ProjectConfig};
use crate::api::ApiClient;

pub async fn execute(name: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let token = config.get_token()?;
    
    let workspace_id = config.current_workspace
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No workspace linked. Run 'envsafe link' first"))?;
    
    let project_name = if let Some(n) = name {
        n
    } else {
        Input::<String>::new()
            .with_prompt("Project name")
            .interact()?
    };
    
    let api_client = ApiClient::from_config(&config);
    let workspace_slug = config.get_workspace_slug()?;
    let project = api_client.create_project(&token, &workspace_slug, &project_name).await?;
    
    println!("{}", format!("✓ Created project: {}", project.name).green());
    println!("{}", format!("  ID: {}", project.id).bright_black());
    
    // Auto-select the new project
    let project_config = ProjectConfig {
        workspace_id: workspace_id.clone(),
        project_id: project.id,
        project_name: project.name,
        project_slug: project.slug,
    };
    
    project_config.save()?;
    
    Ok(())
}
