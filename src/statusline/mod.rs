mod usage_tracker;

use anyhow::{Context, Result};
use crate::config::Config;
use usage_tracker::UsageTracker;
use colored::Colorize;
use std::io::Write;

pub struct StatusLine {
    config: Config,
    tracker: UsageTracker,
}

impl StatusLine {
    pub async fn new(config: Config) -> Result<Self> {
        let tracker = UsageTracker::new(config.clone()).await?;

        Ok(Self { config, tracker })
    }

    /// Render a single status line (for Claude Code integration)
    pub async fn render_line(&self) -> Result<()> {
        let usage = self.tracker.get_usage().await?;

        // Format: [5h: 14k/20k 70%] [7d: 130k/200k 65%] $0.15/hr
        let line = format!(
            "[5h: {}/{}k {}%] [7d: {}/{}k {}%] ${:.2}/hr",
            usage.five_hour_used / 1000,
            usage.five_hour_limit / 1000,
            usage.five_hour_percent,
            usage.seven_day_used / 1000,
            usage.seven_day_limit / 1000,
            usage.seven_day_percent,
            usage.burn_rate_per_hour,
        );

        print!("{}", line);
        std::io::stdout().flush()?;

        Ok(())
    }

    /// Show detailed status
    pub async fn show_status(&self, detailed: bool) -> Result<()> {
        let usage = self.tracker.get_usage().await?;

        println!("\n{}", "Claude Usage Status".bright_cyan().bold());
        println!("{}", "═".repeat(60).bright_cyan());

        // 5-hour block
        println!("\n{}", "Current 5-Hour Block:".white().bold());
        let five_hour_bar = self.create_progress_bar(usage.five_hour_percent);
        println!("  {}", five_hour_bar);
        println!("  Used: {} / {} tokens ({}%)",
                 usage.five_hour_used,
                 usage.five_hour_limit,
                 usage.five_hour_percent);

        if usage.five_hour_minutes_remaining > 0 {
            println!("  Time remaining: {} minutes", usage.five_hour_minutes_remaining);
        } else {
            println!("  Block resets: soon");
        }

        // 7-day total
        println!("\n{}", "7-Day Total:".white().bold());
        let seven_day_bar = self.create_progress_bar(usage.seven_day_percent);
        println!("  {}", seven_day_bar);
        println!("  Used: {} / {} tokens ({}%)",
                 usage.seven_day_used,
                 usage.seven_day_limit,
                 usage.seven_day_percent);

        // Burn rate and cost
        if self.config.statusline.show_costs {
            println!("\n{}", "Cost Information:".white().bold());
            println!("  Burn rate: ${:.2}/hour", usage.burn_rate_per_hour);
            println!("  Estimated 7-day cost: ${:.2}", usage.estimated_seven_day_cost);

            if usage.five_hour_percent > 80 {
                println!("\n  {} You're using tokens quickly!", "⚠".yellow());
            }
        }

        if detailed {
            self.show_detailed_breakdown(&usage).await?;
        }

        Ok(())
    }

    /// Watch usage in real-time
    pub async fn watch(&self, interval: u64) -> Result<()> {
        use tokio::time::{sleep, Duration};

        println!("{}", "Watching Claude usage (Ctrl+C to exit)...".bright_cyan());
        println!();

        loop {
            // Clear screen and move to top
            print!("\x1B[2J\x1B[1;1H");

            self.show_status(false).await?;

            println!("\n{}", format!("Updating every {} seconds...", interval).italic());

            sleep(Duration::from_secs(interval)).await;
        }
    }

    /// Install status line integration for Claude Code
    pub async fn install_integration() -> Result<()> {
        use std::fs;
        use dialoguer::Confirm;

        let settings_path = crate::config::auth::AuthConfig::claude_settings_path()
            .context("Could not find Claude settings path")?;

        if !settings_path.exists() {
            anyhow::bail!(
                "Claude Code settings file not found at {:?}. \
                Please ensure Claude Code is installed.",
                settings_path
            );
        }

        // Read existing settings
        let settings_content = fs::read_to_string(&settings_path)
            .context("Failed to read Claude settings")?;

        let mut settings: serde_json::Value = serde_json::from_str(&settings_content)
            .context("Failed to parse Claude settings")?;

        // Add statusline command
        let statusline_cmd = "claude-helper statusline";

        if let Some(obj) = settings.as_object_mut() {
            obj.insert("statusLine".to_string(), serde_json::Value::String(statusline_cmd.to_string()));
        }

        // Confirm with user
        let confirmed = Confirm::new()
            .with_prompt(format!(
                "This will update your Claude Code settings at {:?}. Continue?",
                settings_path
            ))
            .default(true)
            .interact()?;

        if !confirmed {
            println!("Installation cancelled.");
            return Ok(());
        }

        // Write updated settings
        let updated_content = serde_json::to_string_pretty(&settings)?;
        fs::write(&settings_path, updated_content)
            .context("Failed to write Claude settings")?;

        println!("{}", "✓ Status line integration installed!".green());
        println!("\nRestart Claude Code to see the status line.");

        Ok(())
    }

    fn create_progress_bar(&self, percent: u8) -> String {
        let filled = (percent as usize * 40) / 100;
        let empty = 40 - filled;

        let bar = format!(
            "[{}{}]",
            "█".repeat(filled),
            "░".repeat(empty)
        );

        if percent >= 90 {
            bar.bright_red().to_string()
        } else if percent >= 70 {
            bar.yellow().to_string()
        } else {
            bar.green().to_string()
        }
    }

    async fn show_detailed_breakdown(&self, usage: &usage_tracker::Usage) -> Result<()> {
        println!("\n{}", "Detailed Breakdown:".white().bold());

        // This would show per-session or per-hour breakdown
        // For now, just show some additional info

        println!("  API endpoint: {}", self.config.statusline.api_endpoint);

        Ok(())
    }
}
