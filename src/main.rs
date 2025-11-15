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
        #[arg(long)]
        max_agents: Option<usize>,

        /// Token budget for this task
        #[arg(short = 'b', long)]
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

    /// Install complete Claude Code integration (status line + hooks + commands)
    InstallClaudeIntegration,

    /// Session start hook (called by Claude Code on session start)
    SessionStart,

    /// Log usage hook (called by Claude Code after each response)
    LogUsage,

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

        Commands::InstallClaudeIntegration => {
            install_claude_integration().await?;
        }

        Commands::SessionStart => {
            handle_session_start(&config).await?;
        }

        Commands::LogUsage => {
            handle_log_usage(&config).await?;
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

async fn install_claude_integration() -> Result<()> {
    use std::fs;
    use std::path::PathBuf;
    use anyhow::Context;

    println!("📦 Installing Claude Code integration...\n");

    // Get home directory
    let home = std::env::var("HOME")
        .context("HOME environment variable not set")?;
    let claude_dir = PathBuf::from(home).join(".claude");
    let commands_dir = claude_dir.join("commands");

    // Create directories
    fs::create_dir_all(&commands_dir)
        .context("Failed to create .claude/commands directory")?;
    println!("✓ Created directory structure");

    // Copy settings.json
    let settings_content = include_str!("../.claude-templates/settings.json");
    fs::write(claude_dir.join("settings.json"), settings_content)
        .context("Failed to write settings.json")?;
    println!("✓ Installed settings.json");

    // Copy command files
    let commands = vec![
        ("master.md", include_str!("../.claude-templates/commands/master.md")),
        ("optimize.md", include_str!("../.claude-templates/commands/optimize.md")),
        ("token-usage.md", include_str!("../.claude-templates/commands/token-usage.md")),
    ];

    for (filename, content) in commands {
        fs::write(commands_dir.join(filename), content)
            .with_context(|| format!("Failed to write {}", filename))?;
        println!("✓ Installed /{}",  filename.trim_end_matches(".md"));
    }

    println!("\n✨ Claude Code integration installed successfully!\n");
    println!("Next time you run 'claude', you'll have:");
    println!("  • Status line showing token usage (updates every 5s)");
    println!("  • /master - Run Master Coder orchestration");
    println!("  • /optimize - Get session optimization suggestions");
    println!("  • /token-usage - View detailed token breakdown");
    println!("\nConfiguration: ~/.claude/settings.json");
    println!("Commands: ~/.claude/commands/\n");

    Ok(())
}

async fn handle_session_start(config: &Config) -> Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;
    use chrono::Utc;

    // Log session start to database
    let db_path = Config::db_file()?;
    let session_log = db_path.parent().unwrap().join("sessions.log");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(session_log)?;

    writeln!(file, "[{}] Session started", Utc::now().to_rfc3339())?;

    // Initialize session tracking
    let analyzer = SessionAnalyzer::new(config.clone()).await?;
    analyzer.start_session().await?;

    Ok(())
}

async fn handle_log_usage(config: &Config) -> Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;
    use chrono::Utc;

    // Get current usage and log it
    let statusline = StatusLine::new(config.clone()).await?;
    let usage = statusline.get_current_usage().await?;

    // Log to file
    let db_path = Config::db_file()?;
    let usage_log = db_path.parent().unwrap().join("usage.log");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(usage_log)?;

    writeln!(
        file,
        "[{}] 5h: {}/{} ({}%), 7d: {}/{} ({}%), Burn: ${:.2}/hr",
        Utc::now().to_rfc3339(),
        usage.five_hour_used,
        usage.five_hour_limit,
        usage.five_hour_percent,
        usage.seven_day_used,
        usage.seven_day_limit,
        usage.seven_day_percent,
        usage.burn_rate_per_hour
    )?;

    // Analyze for optimization opportunities
    let analyzer = SessionAnalyzer::new(config.clone()).await?;
    analyzer.log_interaction().await?;

    Ok(())
}
