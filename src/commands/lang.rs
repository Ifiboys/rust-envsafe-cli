use anyhow::Result;
use colored::*;
use crate::config::Config;

pub async fn execute(language: Option<String>) -> Result<()> {
    let mut config = Config::load()?;
    
    let lang = if let Some(l) = language {
        l
    } else {
        let selection = dialoguer::Select::new()
            .with_prompt("Select language / Sélectionner la langue")
            .items(&["English (en)", "Français (fr)"])
            .default(0)
            .interact()?;
        
        if selection == 0 { "en".to_string() } else { "fr".to_string() }
    };
    
    config.set_language(&lang)?;
    
    let message = match lang.as_str() {
        "fr" => "✓ Langue changée en français",
        _ => "✓ Language changed to English",
    };
    
    println!("{}", message.green());
    
    Ok(())
}
