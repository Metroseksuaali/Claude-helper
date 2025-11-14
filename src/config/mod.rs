pub mod auth;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

pub use auth::{AuthMethod, AuthConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub auth: AuthConfig,
    pub master_coder: MasterCoderConfig,
    pub statusline: StatusLineConfig,
    pub analyzer: AnalyzerConfig,
}

// TODO: Add validation for config values:
// - default_mode must be one of: conservative, balanced, trust, interactive
// - max_parallel_agents must be >= 1 and <= 100
// - token_budget must be >= 1000 and reasonable (<= 1_000_000)
// TODO: Add tests for config validation
// TODO: Add tests for config serialization/deserialization roundtrip
// TODO: Add tests for invalid TOML parsing

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterCoderConfig {
    /// Default autonomy mode: conservative, balanced, trust, interactive
    pub default_mode: String,

    /// Maximum number of parallel agents
    pub max_parallel_agents: usize,

    /// Token budget per task (prevents runaway costs)
    pub token_budget: usize,

    /// Enable learning from past sessions
    pub enable_learning: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusLineConfig {
    /// Update interval in seconds
    pub update_interval: u64,

    /// Show cost estimates
    pub show_costs: bool,

    /// Claude API endpoint for usage data
    pub api_endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerConfig {
    /// Number of sessions to analyze
    pub history_depth: usize,

    /// Minimum token savings to suggest optimization
    pub min_savings_threshold: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auth: AuthConfig::default(),
            master_coder: MasterCoderConfig {
                default_mode: "balanced".to_string(),
                max_parallel_agents: 5,
                token_budget: 50000,
                enable_learning: true,
            },
            statusline: StatusLineConfig {
                update_interval: 30,
                show_costs: true,
                api_endpoint: "https://claude.ai/api".to_string(),
            },
            analyzer: AnalyzerConfig {
                history_depth: 50,
                min_savings_threshold: 500,
            },
        }
    }
}

impl Config {
    /// Get the configuration directory path
    pub fn config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Failed to find config directory")?
            .join("claude-helper");

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .context("Failed to create config directory")?;
        }

        Ok(config_dir)
    }

    /// Get the configuration file path
    pub fn config_file() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    /// Get the database directory path
    pub fn db_dir() -> Result<PathBuf> {
        let db_dir = Self::config_dir()?.join("db");

        if !db_dir.exists() {
            fs::create_dir_all(&db_dir)
                .context("Failed to create database directory")?;
        }

        Ok(db_dir)
    }

    /// Get the database file path
    pub fn db_file() -> Result<PathBuf> {
        Ok(Self::db_dir()?.join("claude-helper.db"))
    }

    /// Load configuration from file or create default
    pub async fn load() -> Result<Self> {
        let config_file = Self::config_file()?;

        if config_file.exists() {
            let contents = fs::read_to_string(&config_file)
                .context("Failed to read config file")?;

            let config: Config = toml::from_str(&contents)
                .context("Failed to parse config file")?;

            Ok(config)
        } else {
            // Create default config
            let config = Config::default();
            config.save().await?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub async fn save(&self) -> Result<()> {
        let config_file = Self::config_file()?;
        let contents = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;

        fs::write(&config_file, contents)
            .context("Failed to write config file")?;

        Ok(())
    }

    /// Show current configuration
    pub fn show(&self) -> Result<()> {
        println!("{}", toml::to_string_pretty(self)?);
        Ok(())
    }

    /// Set API key interactively
    pub async fn set_api_key() -> Result<()> {
        use dialoguer::{Input, Select};

        let methods = vec!["Claude Code (Pro/Max)", "API Key"];
        let selection = Select::new()
            .with_prompt("Select authentication method")
            .items(&methods)
            .default(0)
            .interact()?;

        let mut config = Self::load().await?;

        match selection {
            0 => {
                config.auth.method = AuthMethod::ClaudeCode;
                println!("✓ Using Claude Code authentication");
            }
            1 => {
                let api_key: String = Input::new()
                    .with_prompt("Enter your Anthropic API key")
                    .interact_text()?;

                config.auth.method = AuthMethod::ApiKey;
                config.auth.api_key = Some(api_key);
                println!("✓ API key saved");
            }
            _ => unreachable!(),
        }

        config.save().await?;
        Ok(())
    }

    /// Edit configuration file
    pub fn edit() -> Result<()> {
        let config_file = Self::config_file()?;
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

        std::process::Command::new(editor)
            .arg(&config_file)
            .status()
            .context("Failed to open editor")?;

        Ok(())
    }

    /// Reset configuration to defaults
    pub async fn reset() -> Result<()> {
        use dialoguer::Confirm;

        let confirmed = Confirm::new()
            .with_prompt("Are you sure you want to reset configuration to defaults?")
            .default(false)
            .interact()?;

        if confirmed {
            let config = Config::default();
            config.save().await?;
            println!("✓ Configuration reset to defaults");
        }

        Ok(())
    }
}
