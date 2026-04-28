use anyhow::Result;
use colored::*;
use crate::config::Config;
use crate::api::ApiClient;

pub async fn execute(workspace: Option<String>) -> Result<()> {
    let mut config = Config::load()?;
    let token = config.get_token()?;
    let api_client = ApiClient::from_config(&config);
    
    println!("{}", "🔗 Link Workspace".cyan().bold());
    println!();
    
    let (workspace_id, workspace_slug) = if let Some(ws) = workspace {
        (ws.clone(), None)
    } else {
        // Fetch available workspaces
        let workspaces = api_client.get_workspaces(&token).await?;
        
        if workspaces.is_empty() {
            anyhow::bail!("No workspaces found");
        }
        
        let items: Vec<String> = workspaces
            .iter()
            .map(|w| format!("{} ({})", w.name, w.id))
            .collect();
        
        let selection = dialoguer::Select::new()
            .with_prompt("Select a workspace")
            .items(&items)
            .interact()?;
        
        let selected = &workspaces[selection];
        (selected.id.clone(), selected.slug.clone())
    };
    
    config.set_workspace(&workspace_id, workspace_slug.as_deref())?;
    
    println!("{}", format!("✓ Linked to workspace: {}", workspace_id).green());
    
    Ok(())
}
