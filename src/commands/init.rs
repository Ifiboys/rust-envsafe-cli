use crate::api::ApiClient;
use crate::config::{Config, ProjectConfig};
use anyhow::Result;
use colored::*;

pub async fn execute() -> Result<()> {
    let mut config = Config::load()?;
    let token = config.get_token()?;
    let api_client = ApiClient::from_config(&config);

    println!("{}", "🚀 Initialize Project".cyan().bold());
    println!();

    let (workspace_slug, workspace_id) = if let Some(ws_slug) = &config.current_workspace_slug {
        (ws_slug.clone(), config.current_workspace.clone().unwrap())
    } else if let Some(ws_id) = &config.current_workspace {
        // Fallback for legacy config
        (ws_id.clone(), ws_id.clone())
    } else {
        let workspaces = api_client.get_workspaces(&token).await?;

        if workspaces.is_empty() {
            anyhow::bail!("No workspaces found");
        }

        let items: Vec<String> = workspaces.iter().map(|w| w.name.clone()).collect();

        let selection = dialoguer::Select::new()
            .with_prompt("Select a workspace")
            .items(&items)
            .interact()?;

        let selected = &workspaces[selection];
        let ws_id = selected.id.clone();
        let ws_slug = selected.slug.clone().unwrap_or(ws_id.clone());
        config.set_workspace(&ws_id, Some(&ws_slug))?;
        (ws_slug, ws_id)
    };

    let projects = api_client.get_projects(&token, &workspace_slug).await?;

    if projects.is_empty() {
        println!("{}", "No projects found in this workspace".yellow());
        println!(
            "{}",
            "Run 'envsafe create' to create a new project".bright_black()
        );
        return Ok(());
    }

    let items: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();

    let selection = dialoguer::Select::new()
        .with_prompt("Select a project")
        .items(&items)
        .interact()?;

    let selected_project = &projects[selection];

    let project_config = ProjectConfig {
        workspace_id: workspace_id.clone(),
        project_id: selected_project.id.clone(),
        project_name: selected_project.name.clone(),
        project_slug: selected_project.slug.clone(),
    };

    project_config.save()?;

    println!();
    println!(
        "{}",
        format!("✓ Initialized with project: {}", selected_project.name).green()
    );
    println!("{}", "  Configuration saved to .envsafe".bright_black());

    Ok(())
}
