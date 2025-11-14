use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::time::Instant;
use crate::config::Config;
use super::{Agent, AgentResult, AgentCapability};

#[derive(Debug, Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: usize,
    messages: Vec<Message>,
    system: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ClaudeResponse {
    content: Vec<ContentBlock>,
    usage: Usage,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Usage {
    input_tokens: usize,
    output_tokens: usize,
}

pub struct ClaudeAgent {
    id: String,
    agent_type: String,
    capability: AgentCapability,
    system_prompt: String,
    config: Config,
    client: Client,
    conversation: Vec<Message>,
}

impl ClaudeAgent {
    pub async fn new(
        id: String,
        agent_type: String,
        capability: AgentCapability,
        system_prompt: String,
        config: Config,
    ) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            id,
            agent_type,
            capability,
            system_prompt,
            config,
            client,
            conversation: Vec::new(),
        })
    }

    async fn call_claude_api(&self, messages: &[Message]) -> Result<ClaudeResponse> {
        let token = self.config.auth.get_token().await?;

        let request = ClaudeRequest {
            model: "claude-sonnet-4-5-20250929".to_string(),
            max_tokens: 8192,
            messages: messages.to_vec(),
            system: Some(self.system_prompt.clone()),
        };

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", token)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Claude API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Claude API error ({}): {}", status, error_text);
        }

        let claude_response: ClaudeResponse = response.json().await
            .context("Failed to parse Claude API response")?;

        Ok(claude_response)
    }
}

#[async_trait]
impl Agent for ClaudeAgent {
    fn id(&self) -> &str {
        &self.id
    }

    fn agent_type(&self) -> &str {
        &self.agent_type
    }

    fn capability(&self) -> &AgentCapability {
        &self.capability
    }

    async fn execute(&mut self, task: &str) -> Result<AgentResult> {
        let start_time = Instant::now();

        // Add user message to conversation
        self.conversation.push(Message {
            role: "user".to_string(),
            content: task.to_string(),
        });

        // Call Claude API
        let response = self.call_claude_api(&self.conversation).await?;

        // Extract text from response
        let output = response.content
            .iter()
            .filter_map(|block| block.text.as_ref())
            .cloned()
            .collect::<Vec<_>>()
            .join("\n");

        // Add assistant response to conversation
        self.conversation.push(Message {
            role: "assistant".to_string(),
            content: output.clone(),
        });

        let execution_time = start_time.elapsed().as_millis() as u64;
        let tokens_used = response.usage.input_tokens + response.usage.output_tokens;

        Ok(AgentResult {
            success: true,
            output,
            tokens_used,
            execution_time_ms: execution_time,
        })
    }

    fn conversation_history(&self) -> Vec<String> {
        self.conversation
            .iter()
            .map(|msg| format!("{}: {}", msg.role, msg.content))
            .collect()
    }

    fn reset(&mut self) {
        self.conversation.clear();
    }
}
