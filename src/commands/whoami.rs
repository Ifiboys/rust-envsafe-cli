use crate::api::ApiClient;
use crate::config::Config;
use crate::utils::i18n::get_translations;
use anyhow::Result;
use colored::*;

pub async fn execute() -> Result<()> {
    let config = Config::load()?;
    let t = get_translations(&config.language);

    let token = config.get_token()?;
    let api_client = ApiClient::from_config(&config);

    let user = api_client.get_user(&token).await?;

    println!("{}", t.whoami.title.cyan().bold());
    println!();

    // Use replace to inject values into translated strings containing "{}"
    println!(
        "{}",
        t.whoami
            .name
            .replace("{}", &user.name.bright_white().to_string())
    );
    println!(
        "{}",
        t.whoami
            .email
            .replace("{}", &user.email.bright_black().to_string())
    );

    if let Some(last_login) = user.last_login {
        println!(
            "{}",
            t.whoami
                .last_login
                .replace("{}", &last_login.bright_black().to_string())
        );
    }

    Ok(())
}
