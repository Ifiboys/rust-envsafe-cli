use anyhow::Result;
use colored::*;
use crate::config::Config;

pub async fn execute(api_url: Option<String>, show: bool) -> Result<()> {
    let mut config = Config::load()?;
    
    if let Some(url) = api_url {
        config.api_url = url.clone();
        config.dashboard_url = url.clone();
        config.save()?;
        println!("{}", format!("✓ API URL set to: {}", url).green());
    }
    
    if show {
        println!("{}", "⚙️  Configuration".cyan().bold());
        println!();
        println!("  API URL: {}", config.api_url);
        println!("  Dashboard URL: {}", config.dashboard_url);
        println!("  Language: {}", config.language);
        
        let token_display = if config.token.is_some() {
            "••••••••"
        } else {
            "(not set)"
        };
        println!("  Token: {}", token_display.bright_black());
    }
    
    Ok(())
}
