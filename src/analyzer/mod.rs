mod optimizer;
mod session_parser;

use crate::config::Config;
use crate::db::Database;
use anyhow::Result;
use colored::Colorize;
use optimizer::Optimizer;
use session_parser::SessionParser;

// Re-export for external use
pub use optimizer::Optimization;

pub struct SessionAnalyzer {
    config: Config,
    parser: SessionParser,
    optimizer: Optimizer,
    db: Database,
}

impl SessionAnalyzer {
    pub async fn new(config: Config) -> Result<Self> {
        let parser = SessionParser::new();
        let optimizer = Optimizer::new(config.analyzer.min_savings_threshold);
        let db = Database::new(&config).await?;

        Ok(Self {
            config,
            parser,
            optimizer,
            db,
        })
    }

    /// Analyze recent sessions
    pub async fn analyze_sessions(&self, count: usize) -> Result<()> {
        println!("\n{}", "Analyzing Sessions".bright_cyan().bold());
        println!("{}", "═".repeat(60).bright_cyan());

        let sessions = self.parser.find_recent_sessions(count)?;

        if sessions.is_empty() {
            println!("\n{}", "No Claude Code sessions found.".yellow());
            println!("Make sure Claude Code is installed and you have session history.");
            return Ok(());
        }

        println!("\nFound {} recent sessions", sessions.len());

        let mut total_optimizations = 0;
        let mut total_potential_savings = 0;

        for session_path in sessions {
            println!(
                "\n{} Analyzing: {:?}",
                "→".bright_cyan(),
                session_path.file_name().unwrap()
            );

            let session_data = self.parser.parse_session(&session_path)?;
            let optimizations = self.optimizer.analyze(&session_data)?;

            if !optimizations.is_empty() {
                total_optimizations += optimizations.len();

                for opt in &optimizations {
                    total_potential_savings += opt.estimated_savings;
                }

                println!("  Found {} optimization opportunities", optimizations.len());
            }
        }

        // Summary
        println!("\n{}", "Analysis Summary".bright_yellow().bold());
        println!("  Total optimizations found: {}", total_optimizations);
        println!("  Potential token savings: ~{}", total_potential_savings);

        if total_optimizations > 0 {
            println!(
                "\n{}",
                "Run 'claude-helper optimize' for detailed recommendations".bright_green()
            );
        }

        Ok(())
    }

    /// Get optimization suggestions
    pub async fn optimize_recent(&self, count: usize) -> Result<()> {
        println!("\n{}", "Optimization Suggestions".bright_cyan().bold());
        println!("{}", "═".repeat(60).bright_cyan());

        let sessions = self.parser.find_recent_sessions(count)?;
        let mut all_optimizations = Vec::new();

        for session_path in sessions {
            let session_data = self.parser.parse_session(&session_path)?;
            let optimizations = self.optimizer.analyze(&session_data)?;
            all_optimizations.extend(optimizations);
        }

        if all_optimizations.is_empty() {
            println!("\n{}", "No optimization opportunities found! 🎉".green());
            println!("Your usage patterns are already efficient.");
            return Ok(());
        }

        // Sort by savings (highest first)
        all_optimizations.sort_by(|a, b| b.estimated_savings.cmp(&a.estimated_savings));

        self.display_optimizations(&all_optimizations)?;

        Ok(())
    }

    /// Optimize a specific session
    pub async fn optimize_session(&self, session_id: &str) -> Result<()> {
        println!(
            "\n{}",
            format!("Optimizing Session: {}", session_id)
                .bright_cyan()
                .bold()
        );
        println!("{}", "═".repeat(60).bright_cyan());

        // Find session by ID
        let session_path = self.parser.find_session_by_id(session_id)?;

        let session_data = self.parser.parse_session(&session_path)?;
        let optimizations = self.optimizer.analyze(&session_data)?;

        if optimizations.is_empty() {
            println!(
                "\n{}",
                "No optimization opportunities found for this session.".green()
            );
            return Ok(());
        }

        self.display_optimizations(&optimizations)?;

        Ok(())
    }

    fn display_optimizations(&self, optimizations: &[Optimization]) -> Result<()> {
        for (i, opt) in optimizations.iter().enumerate() {
            println!(
                "\n{} {}",
                format!("{}.", i + 1).bright_yellow().bold(),
                opt.title.white().bold()
            );
            println!("  Type: {}", self.format_opt_type(&opt.opt_type));
            println!("  Description: {}", opt.description);
            println!(
                "  {} ~{} tokens per occurrence",
                "Savings:".green(),
                opt.estimated_savings
            );

            if !opt.examples.is_empty() {
                println!("  Examples:");
                for example in &opt.examples {
                    println!("    • {}", example);
                }
            }

            if let Some(suggestion) = &opt.suggestion {
                println!("  {} {}", "Suggestion:".bright_green().bold(), suggestion);
            }
        }

        println!("\n{}", "═".repeat(60).bright_cyan());
        let total_savings: usize = optimizations.iter().map(|o| o.estimated_savings).sum();
        println!(
            "{} {} tokens",
            "Total potential savings:".bright_green().bold(),
            total_savings
        );

        Ok(())
    }

    fn format_opt_type(&self, opt_type: &optimizer::OptimizationType) -> String {
        use optimizer::OptimizationType;

        let (label, emoji) = match opt_type {
            OptimizationType::QuickCommand => ("Quick Command", "⚡"),
            OptimizationType::ParameterizedScript => ("Parameterized Script", "📝"),
            OptimizationType::FileMerge => ("File Merge", "🔗"),
            OptimizationType::FileSplit => ("File Split", "✂️"),
            OptimizationType::ContextPruning => ("Context Pruning", "🎯"),
            OptimizationType::ToolCallBatching => ("Tool Call Batching", "📦"),
        };

        format!("{} {}", emoji, label)
    }

    /// Start a new session (called from sessionStart hook)
    pub async fn start_session(&self) -> Result<()> {
        // Initialize session tracking in database
        // This could create a new session record with a unique ID
        Ok(())
    }

    /// Log an interaction (called from afterResponse hook)
    pub async fn log_interaction(&self) -> Result<()> {
        // Log current interaction for real-time optimization analysis
        // This could update session stats and check for optimization opportunities
        Ok(())
    }
}
