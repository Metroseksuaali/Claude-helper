use anyhow::Result;
use clap::{Parser, Subcommand};
use claude_helper::{
    config::Config,
    master::MasterCoder,
    statusline::StatusLine,
    analyzer::SessionAnalyzer,
    tui::App,
};
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "claude-helper")]
#[command(about = "Advanced orchestration and optimization toolkit for Claude Code", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a task with Master Coder orchestration
    Run {
        /// The task description
        task: String,

        /// Autonomy mode: conservative, balanced, trust, interactive
        #[arg(short, long, default_value = "balanced")]
        mode: String,

        /// Maximum number of parallel agents
        #[arg(short, long)]
        max_agents: Option<usize>,

        /// Token budget for this task
        #[arg(short, long)]
        token_budget: Option<usize>,
    },

    /// Show current token usage status
    Status {
        /// Show detailed breakdown
        #[arg(short, long)]
        detailed: bool,
    },

    /// Watch token usage in real-time
    Watch {
        /// Update interval in seconds
        #[arg(short, long, default_value = "30")]
        interval: u64,
    },

    /// Display status line (for Claude Code integration)
    Statusline,

    /// Analyze sessions for optimization opportunities
    Analyze {
        /// Number of recent sessions to analyze
        #[arg(short, long, default_value = "10")]
        last: usize,
    },

    /// Get optimization suggestions
    Optimize {
        /// Session ID to optimize
        #[arg(short, long)]
        session: Option<String>,

        /// Analyze last N sessions
        #[arg(short, long)]
        last: Option<usize>,
    },

    /// Open interactive TUI
    Tui,

    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Install status line integration for Claude Code
    InstallStatusline,

    /// Agent management and statistics
    Agents {
        #[command(subcommand)]
        action: AgentAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show current configuration
    Show,

    /// Set API key
    SetApiKey,

    /// Edit configuration file
    Edit,

    /// Reset to defaults
    Reset,
}

#[derive(Subcommand)]
enum AgentAction {
    /// Show agent statistics
    Stats,

    /// List available agent types
    List,

    /// Show agent history
    History {
        /// Number of recent agents to show
        #[arg(short, long, default_value = "20")]
        last: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .init();

    info!("Claude Helper starting...");

    // Load configuration
    let config = Config::load().await?;

    // Execute command
    match cli.command {
        Commands::Run { task, mode, max_agents, token_budget } => {
            let mut master = MasterCoder::new(config, mode).await?;
            if let Some(max) = max_agents {
                master.set_max_agents(max);
            }
            if let Some(budget) = token_budget {
                master.set_token_budget(budget);
            }
            master.execute(&task).await?;
        }

        Commands::Status { detailed } => {
            let statusline = StatusLine::new(config).await?;
            statusline.show_status(detailed).await?;
        }

        Commands::Watch { interval } => {
            let statusline = StatusLine::new(config).await?;
            statusline.watch(interval).await?;
        }

        Commands::Statusline => {
            let statusline = StatusLine::new(config).await?;
            statusline.render_line().await?;
        }

        Commands::Analyze { last } => {
            let analyzer = SessionAnalyzer::new(config).await?;
            analyzer.analyze_sessions(last).await?;
        }

        Commands::Optimize { session, last } => {
            let analyzer = SessionAnalyzer::new(config).await?;
            if let Some(sid) = session {
                analyzer.optimize_session(&sid).await?;
            } else {
                analyzer.optimize_recent(last.unwrap_or(10)).await?;
            }
        }

        Commands::Tui => {
            let app = App::new(config).await?;
            app.run().await?;
        }

        Commands::Config { action } => {
            handle_config_action(action, &config).await?;
        }

        Commands::InstallStatusline => {
            StatusLine::install_integration().await?;
        }

        Commands::Agents { action } => {
            handle_agent_action(action, &config).await?;
        }
    }

    Ok(())
}

async fn handle_config_action(action: ConfigAction, config: &Config) -> Result<()> {
    match action {
        ConfigAction::Show => {
            config.show()?;
        }
        ConfigAction::SetApiKey => {
            Config::set_api_key().await?;
        }
        ConfigAction::Edit => {
            Config::edit()?;
        }
        ConfigAction::Reset => {
            Config::reset().await?;
        }
    }
    Ok(())
}

async fn handle_agent_action(action: AgentAction, config: &Config) -> Result<()> {
    use claude_helper::agents::AgentManager;

    let manager = AgentManager::new(config).await?;

    match action {
        AgentAction::Stats => {
            manager.show_stats().await?;
        }
        AgentAction::List => {
            manager.list_types()?;
        }
        AgentAction::History { last } => {
            manager.show_history(last).await?;
        }
    }
    Ok(())
}
