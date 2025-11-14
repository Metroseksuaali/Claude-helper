use super::AgentCapability;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    pub success: bool,
    pub output: String,
    pub tokens_used: usize,
    pub execution_time_ms: u64,
}

/// Base trait for all agents
#[async_trait]
pub trait Agent: Send + Sync {
    /// Get agent ID
    fn id(&self) -> &str;

    /// Get agent type (e.g., "Code Writer Alpha", "Security Auditor")
    fn agent_type(&self) -> &str;

    /// Get agent capability
    fn capability(&self) -> &AgentCapability;

    /// Execute a task
    async fn execute(&mut self, task: &str) -> Result<AgentResult>;

    /// Get conversation history (if applicable)
    fn conversation_history(&self) -> Vec<String> {
        Vec::new()
    }

    /// Reset agent state
    fn reset(&mut self) {}
}
