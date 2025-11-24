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
        // Try to read OAuth token from .credentials.json (new Claude Code format)
        let home = dirs::home_dir()
            .context("Could not find home directory")?;
        let credentials_path = home.join(".claude").join(".credentials.json");

        if credentials_path.exists() {
            // Parse credentials.json
            let credentials_content = fs::read_to_string(&credentials_path)
                .context("Failed to read .credentials.json")?;

            let credentials: serde_json::Value = serde_json::from_str(&credentials_content)
                .context("Failed to parse .credentials.json")?;

            // Extract accessToken from claudeAiOauth
            if let Some(token) = credentials
                .get("claudeAiOauth")
                .and_then(|oauth| oauth.get("accessToken"))
                .and_then(|token| token.as_str())
            {
                return Ok(token.to_string());
            }
        }

        // Fallback: Try old session format for backwards compatibility
        let session_path = self
            .claude_code_session_path
            .clone()
            .or_else(Self::default_session_path)
            .context("Could not determine Claude Code session path")?;

        let session_key_file = session_path.join("sessionKey");
        if session_key_file.exists() {
            let token = fs::read_to_string(&session_key_file)
                .context("Failed to read session key")?
                .trim()
                .to_string();

            return Ok(token);
        }

        anyhow::bail!(
            "No active Claude Code session found. \
            Please log in to Claude Code or use API key authentication."
        )
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
