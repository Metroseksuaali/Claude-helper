use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthMethod {
    /// Use Claude Code's authentication (Pro/Max license)
    ClaudeCode,
    /// Use direct API key
    ApiKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Authentication method
    pub method: AuthMethod,

    /// API key (if using ApiKey method)
    pub api_key: Option<String>,

    /// Path to Claude Code session file
    pub claude_code_session_path: Option<PathBuf>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            method: AuthMethod::ClaudeCode,
            api_key: None,
            claude_code_session_path: Self::default_session_path(),
        }
    }
}

impl AuthConfig {
    /// Get default Claude Code session path
    fn default_session_path() -> Option<PathBuf> {
        dirs::home_dir().map(|home| home.join(".claude").join("sessions"))
    }

    /// Get the Claude Code settings file path
    pub fn claude_settings_path() -> Option<PathBuf> {
        dirs::home_dir().map(|home| home.join(".claude").join("settings.json"))
    }

    /// Get authentication token
    pub async fn get_token(&self) -> Result<String> {
        match &self.method {
            AuthMethod::ClaudeCode => self.get_claude_code_token().await,
            AuthMethod::ApiKey => self.api_key.clone().context("API key not configured"),
        }
    }

    /// Get token from Claude Code session
    async fn get_claude_code_token(&self) -> Result<String> {
        // Try to read session cookie from Claude Code
        let session_path = self
            .claude_code_session_path
            .clone()
            .or_else(Self::default_session_path)
            .context("Could not determine Claude Code session path")?;

        // Check if session directory exists
        if !session_path.exists() {
            anyhow::bail!(
                "Claude Code session directory not found at {:?}. \
                Please ensure Claude Code is installed and you're logged in, \
                or use API key authentication instead.",
                session_path
            );
        }

        // Read session cookie/token
        // Note: This is a simplified version. In production, you'd need to:
        // 1. Parse the session files properly
        // 2. Handle session refresh
        // 3. Validate session is still active

        // For now, we'll look for a sessionKey file
        let session_key_file = session_path.join("sessionKey");
        if session_key_file.exists() {
            let token = fs::read_to_string(&session_key_file)
                .context("Failed to read session key")?
                .trim()
                .to_string();

            Ok(token)
        } else {
            anyhow::bail!(
                "No active Claude Code session found. \
                Please log in to Claude Code or use API key authentication."
            )
        }
    }

    /// Validate authentication configuration
    pub async fn validate(&self) -> Result<()> {
        match &self.method {
            AuthMethod::ClaudeCode => {
                // Try to get token to validate
                self.get_claude_code_token().await?;
                Ok(())
            }
            AuthMethod::ApiKey => {
                if self.api_key.is_none() {
                    anyhow::bail!("API key not configured");
                }
                Ok(())
            }
        }
    }
}
