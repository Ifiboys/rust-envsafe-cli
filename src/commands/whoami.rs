use anyhow::Result;
use colored::*;
use crate::config::Config;
use crate::api::ApiClient;

pub async fn execute() -> Result<()> {
    let config = Config::load()?;
    let token = config.get_token()?;
    let api_client = ApiClient::from_config(&config);
    
    let user = api_client.get_user(&token).await?;
    
    println!("{}", "👤 Current User".cyan().bold());
    println!();
    println!("  Name: {}", user.name.bright_white());
    println!("  Email: {}", user.email.bright_black());
    
    if let Some(last_login) = user.last_login {
        println!("  Last login: {}", last_login.bright_black());
    }
    
    Ok(())
}
