use crate::api::ApiClient;
use crate::config::{Config, ProjectConfig};
use crate::utils::i18n::get_translations;
use anyhow::Result;
use colored::*;

pub async fn execute() -> Result<()> {
    let mut config = Config::load()?;
    let t = get_translations(&config.language);

    let token = config.get_token()?;
    let api_client = ApiClient::from_config(&config);

    println!("{}", t.init.title.cyan().bold());
    println!();

    let (workspace_slug, workspace_id) = if let Some(ws_slug) = &config.current_workspace_slug {
        (ws_slug.clone(), config.current_workspace.clone().unwrap())
    } else if let Some(ws_id) = &config.current_workspace {
        // Fallback for legacy config
        (ws_id.clone(), ws_id.clone())
    } else {
        let workspaces = api_client.get_workspaces(&token).await?;

        if workspaces.is_empty() {
            anyhow::bail!("{}", t.common.no_workspaces);
        }

        let items: Vec<String> = workspaces.iter().map(|w| w.name.clone()).collect();

        let selection = dialoguer::Select::new()
            .with_prompt(t.common.select_workspace)
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
        println!("{}", t.common.no_projects.yellow());
        // Note: Creating a project message isn't translated yet in i18n struct, using generic fallback or keeping English for now on this specific hint line
        // Or better, let's just stick to translated "no projects" message.
        return Ok(());
    }

    let items: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();

    let selection = dialoguer::Select::new()
        .with_prompt(t.common.select_project)
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
        "{} {}",
        "✓".green(),
        format!("{} : {}", t.init.success, selected_project.name).green()
    );
    println!("{}", t.init.creating_config.bright_black());

    Ok(())
}
