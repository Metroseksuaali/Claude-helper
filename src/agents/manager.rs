use anyhow::Result;
use crate::config::Config;
use crate::db::Database;
use super::AgentCapability;
use colored::Colorize;

pub struct AgentManager {
    config: Config,
    db: Database,
}

impl AgentManager {
    pub async fn new(config: &Config) -> Result<Self> {
        let db = Database::new(config).await?;

        Ok(Self {
            config: config.clone(),
            db,
        })
    }

    pub async fn show_stats(&self) -> Result<()> {
        let stats = self.db.get_agent_stats().await?;

        println!("\n{}", "Agent Statistics".bright_cyan().bold());
        println!("{}", "═".repeat(50).bright_cyan());

        println!("\n{}", "Total Agents Executed:".white().bold());
        println!("  {}", stats.total_executions);

        println!("\n{}", "By Capability:".white().bold());
        for (capability, count) in stats.by_capability {
            println!("  {} {}: {}", capability.emoji(), capability.description(), count);
        }

        println!("\n{}", "Token Usage:".white().bold());
        println!("  Total: {} tokens", stats.total_tokens);
        println!("  Average per agent: {} tokens", stats.avg_tokens_per_agent);

        println!("\n{}", "Execution Time:".white().bold());
        println!("  Total: {:.2} minutes", stats.total_time_secs / 60.0);
        println!("  Average per agent: {:.2} seconds", stats.avg_time_per_agent);

        println!("\n{}", "Success Rate:".white().bold());
        let success_rate = (stats.successful_executions as f64 / stats.total_executions as f64) * 100.0;
        let rate_str = format!("{:.1}%", success_rate);
        let colored_rate = if success_rate >= 90.0 {
            rate_str.green()
        } else if success_rate >= 70.0 {
            rate_str.yellow()
        } else {
            rate_str.red()
        };
        println!("  {} ({}/{})", colored_rate, stats.successful_executions, stats.total_executions);

        Ok(())
    }

    pub fn list_types(&self) -> Result<()> {
        println!("\n{}", "Available Agent Capabilities".bright_cyan().bold());
        println!("{}", "═".repeat(50).bright_cyan());

        let capabilities = vec![
            AgentCapability::Architecture,
            AgentCapability::CodeWriting,
            AgentCapability::Testing,
            AgentCapability::Security,
            AgentCapability::Documentation,
            AgentCapability::Debugging,
            AgentCapability::Performance,
            AgentCapability::Migration,
            AgentCapability::Review,
        ];

        for cap in capabilities {
            println!("\n{} {}", cap.emoji(), cap.description().bright_white().bold());
        }

        println!("\n{}", "Note: Master Coder dynamically creates specialized agents".italic());
        println!("{}", "based on task requirements and complexity.".italic());

        Ok(())
    }

    pub async fn show_history(&self, limit: usize) -> Result<()> {
        let history = self.db.get_agent_history(limit).await?;

        println!("\n{}", "Recent Agent Executions".bright_cyan().bold());
        println!("{}", "═".repeat(80).bright_cyan());

        for entry in history {
            println!("\n{} {} ({})",
                     entry.capability.emoji(),
                     entry.agent_type.bright_white().bold(),
                     entry.timestamp.format("%Y-%m-%d %H:%M:%S"));
            println!("  Task: {}", entry.task.chars().take(60).collect::<String>());
            println!("  Tokens: {} | Time: {:.2}s | Status: {}",
                     entry.tokens_used,
                     entry.execution_time_secs,
                     if entry.success { "✓".green() } else { "✗".red() });
        }

        Ok(())
    }
}
