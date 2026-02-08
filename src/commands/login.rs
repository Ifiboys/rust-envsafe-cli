use crate::api::ApiClient;
use crate::config::Config;
use anyhow::Result;
use colored::*;
use dialoguer::Input;

pub async fn execute(token: Option<String>) -> Result<()> {
    let mut config = Config::load()?;

    println!("{}", "🔐 EnvSafe Login".cyan().bold());
    println!();

    let token = if let Some(t) = token {
        t
    } else {
        println!(
            "{}",
            "Opening browser to generate API token...".bright_black()
        );

        let dashboard_url = format!("{}/dashboard/settings/tokens", config.dashboard_url);

        if open::that(&dashboard_url).is_ok() {
            println!("{}", format!("✓ Browser opened: {}", dashboard_url).green());
        } else {
            println!("{}", format!("Please visit: {}", dashboard_url).yellow());
        }

        println!();

        Input::<String>::new()
            .with_prompt("Enter your API token")
            .interact()?
    };

    let api_client = ApiClient::from_config(&config);

    print!("{}", "Verifying token... ".bright_black());

    match api_client.get_user(&token).await {
        Ok(user) => {
            println!("{}", "✓".green());
            println!();
            println!("{}", format!("Welcome, {}!", user.name).green().bold());
            println!("{}", format!("Email: {}", user.email).bright_black());

            config.set_token(token)?;

            println!();
            let lang = dialoguer::Select::new()
                .with_prompt("Preferred language / Langue préférée")
                .items(&["English (en)", "Français (fr)"])
                .default(0)
                .interact()?;

            let lang_code = if lang == 0 { "en" } else { "fr" };
            config.set_language(lang_code)?;

            println!();
            println!("{}", "✓ Successfully logged in!".green());
        }
        Err(e) => {
            println!("{}", "✗".red());
            println!();
            eprintln!("{}", format!("Login failed: {}", e).red());
            anyhow::bail!("Invalid token");
        }
    }

    Ok(())
}
