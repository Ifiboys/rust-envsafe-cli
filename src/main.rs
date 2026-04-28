mod api;
mod commands;
mod config;
mod rotation;
mod storage;
mod utils;
mod watcher;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "envsafe")]
#[command(about = "🔐 EnvSafe CLI - Secure environment variable manager", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with EnvSafe
    Login {
        /// API Token (optional, interactive if not provided)
        #[arg(short, long)]
        token: Option<String>,
    },

    /// Show current user
    Whoami,

    /// Link current directory to a workspace
    Link {
        /// Workspace ID
        workspace: Option<String>,
    },

    /// Initialize a project
    Init,

    /// List projects in the current workspace
    #[command(alias = "ls")]
    List,

    /// Select a project
    Select {
        /// Project ID or name
        project: String,
    },

    /// Create a new project
    #[command(alias = "new")]
    Create {
        /// Project name
        name: Option<String>,
    },

    /// List all accessible projects
    Projects,

    /// Download environment variables
    Pull {
        /// Project ID or name
        project: Option<String>,

        /// Environment name
        #[arg(short, long)]
        env: Option<String>,

        /// Development environment (shortcut)
        #[arg(short, long)]
        dev: bool,

        /// Staging environment (shortcut)
        #[arg(short, long)]
        staging: bool,

        /// Production environment (shortcut)
        #[arg(short, long)]
        prod: bool,

        /// Output file path
        #[arg(short, long, default_value = ".env")]
        output: String,
    },

    /// Upload environment variables
    Push {
        /// Project ID or name
        project: Option<String>,

        /// Environment name
        #[arg(short, long)]
        env: Option<String>,

        /// Development environment (shortcut)
        #[arg(short, long)]
        dev: bool,

        /// Staging environment (shortcut)
        #[arg(short, long)]
        staging: bool,

        /// Production environment (shortcut)
        #[arg(short, long)]
        prod: bool,

        /// Source file path
        #[arg(short, long, default_value = ".env")]
        file: String,
    },

    /// Run a command with injected variables
    Run {
        /// Project ID or name
        #[arg(long)]
        project: Option<String>,

        /// Command to execute
        #[arg(required = true, trailing_var_arg = true)]
        command: Vec<String>,

        /// Environment name
        #[arg(short, long)]
        env: Option<String>,

        /// Development environment (shortcut)
        #[arg(short, long)]
        dev: bool,

        /// Staging environment (shortcut)
        #[arg(short, long)]
        staging: bool,

        /// Production environment (shortcut)
        #[arg(short, long)]
        prod: bool,
    },

    /// Start real-time variable monitoring
    Watch {
        /// Project ID or name
        project: Option<String>,

        /// Environment name
        #[arg(short, long)]
        env: Option<String>,

        /// Development environment (shortcut)
        #[arg(short, long)]
        dev: bool,

        /// Staging environment (shortcut)
        #[arg(short, long)]
        staging: bool,

        /// Production environment (shortcut)
        #[arg(short, long)]
        prod: bool,

        /// File to watch
        #[arg(short, long, default_value = ".env")]
        file: String,
    },

    /// Manage secret rotation
    Rotate {
        #[command(subcommand)]
        action: RotateAction,
    },

    /// Manage configuration
    Config {
        /// Set EnvSafe API URL
        #[arg(long)]
        api_url: Option<String>,

        /// Show current configuration
        #[arg(long)]
        show: bool,
    },

    /// Change CLI language
    #[command(alias = "language")]
    Lang {
        /// Language code (en, fr)
        language: Option<String>,
    },

    /// Log out
    Logout,

    /// Machine-to-Machine authentication (CI/CD)
    #[command(alias = "machine")]
    M2m {
        /// M2M Token
        #[arg(short, long)]
        token: Option<String>,

        /// Workspace ID
        #[arg(short, long)]
        workspace: Option<String>,

        /// Project ID
        #[arg(short, long)]
        project: Option<String>,
    },
}

#[derive(Subcommand)]
enum RotateAction {
    /// Configure automatic rotation
    Enable {
        /// Rotation interval in days
        #[arg(short, long, default_value = "30")]
        interval: u32,

        /// Variables to exclude
        #[arg(short, long)]
        exclude: Vec<String>,
    },

    /// Disable automatic rotation
    Disable,

    /// Show rotation status
    Status,

    /// Force immediate rotation
    Now {
        /// Specific variables to rotate
        #[arg(short, long)]
        vars: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Login { token } => commands::login::execute(token).await?,
        Commands::Whoami => commands::whoami::execute().await?,
        Commands::Link { workspace } => commands::link::execute(workspace).await?,
        Commands::Init => commands::init::execute().await?,
        Commands::List => commands::list::execute().await?,
        Commands::Select { project } => commands::select::execute(&project).await?,
        Commands::Create { name } => commands::create::execute(name).await?,
        Commands::Projects => commands::projects::execute().await?,
        Commands::Pull {
            project,
            env,
            dev,
            staging,
            prod,
            output,
        } => {
            let environment = determine_environment(env, dev, staging, prod);
            commands::pull::execute(project, environment, &output).await?
        }
        Commands::Push {
            project,
            env,
            dev,
            staging,
            prod,
            file,
        } => {
            let environment = determine_environment(env, dev, staging, prod);
            commands::push::execute(project, environment, &file).await?
        }
        Commands::Run {
            project,
            command,
            env,
            dev,
            staging,
            prod,
        } => {
            let environment = determine_environment(env, dev, staging, prod);
            commands::run::execute(project, environment, command).await?
        }
        Commands::Watch {
            project,
            env,
            dev,
            staging,
            prod,
            file,
        } => {
            let environment = determine_environment(env, dev, staging, prod);
            commands::watch::execute(project, environment, &file).await?
        }
        Commands::Rotate { action } => match action {
            RotateAction::Enable { interval, exclude } => {
                commands::rotate::enable(interval, exclude).await?
            }
            RotateAction::Disable => commands::rotate::disable().await?,
            RotateAction::Status => commands::rotate::status().await?,
            RotateAction::Now { vars } => commands::rotate::now(vars).await?,
        },
        Commands::Config { api_url, show } => commands::config::execute(api_url, show).await?,
        Commands::Lang { language } => commands::lang::execute(language).await?,
        Commands::Logout => commands::logout::execute().await?,
        Commands::M2m {
            token,
            workspace,
            project,
        } => commands::m2m::execute(token, workspace, project).await?,
    }

    Ok(())
}

fn determine_environment(
    env: Option<String>,
    dev: bool,
    staging: bool,
    prod: bool,
) -> Option<String> {
    if dev {
        Some("development".to_string())
    } else if staging {
        Some("staging".to_string())
    } else if prod {
        Some("production".to_string())
    } else {
        env
    }
}
