use anyhow::Result;
use colored::*;
use crate::config::Config;

pub async fn execute() -> Result<()> {
    let mut config = Config::load()?;
    config.clear_token()?;
    
    println!("{}", "✓ Successfully logged out".green());
    
    Ok(())
}
