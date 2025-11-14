mod schema;

use anyhow::{Context, Result};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use crate::config::Config;
use crate::master::planner::TaskAnalysis;
use crate::master::orchestrator::{ExecutionPlan, ExecutionResult};
use crate::agents::AgentCapability;
use chrono::{DateTime, Utc};

pub use schema::*;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(_config: &Config) -> Result<Self> {
        let db_path = Config::db_file()?;
        let db_url = format!("sqlite:{}", db_path.display());

        // Create pool
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .context("Failed to connect to database")?;

        // Run migrations
        sqlx::query(schema::CREATE_TABLES)
            .execute(&pool)
            .await
            .context("Failed to create database tables")?;

        Ok(Self { pool })
    }

    /// Save a task execution for learning
    pub async fn save_task_execution(
        &self,
        task: &str,
        analysis: &TaskAnalysis,
        plan: &ExecutionPlan,
        result: &ExecutionResult,
    ) -> Result<()> {
        let task_json = serde_json::to_string(analysis)?;
        let plan_json = serde_json::to_string(plan)?;
        let result_json = serde_json::to_string(result)?;

        sqlx::query(
            "INSERT INTO task_executions (task_description, complexity, estimated_tokens, actual_tokens, success, task_data, plan_data, result_data)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(task)
        .bind(analysis.complexity as i32)
        .bind(analysis.estimated_tokens as i64)
        .bind(result.tokens_used as i64)
        .bind(result.success)
        .bind(task_json)
        .bind(plan_json)
        .bind(result_json)
        .execute(&self.pool)
        .await
        .context("Failed to save task execution")?;

        Ok(())
    }

    /// Get agent statistics
    pub async fn get_agent_stats(&self) -> Result<AgentStats> {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_executions")
            .fetch_one(&self.pool)
            .await?;

        let successful: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_executions WHERE success = 1")
            .fetch_one(&self.pool)
            .await?;

        let total_tokens: i64 = sqlx::query_scalar("SELECT COALESCE(SUM(tokens_used), 0) FROM agent_executions")
            .fetch_one(&self.pool)
            .await?;

        let total_time: f64 = sqlx::query_scalar("SELECT COALESCE(SUM(execution_time_ms), 0) FROM agent_executions")
            .fetch_one(&self.pool)
            .await
            .map(|ms: i64| ms as f64 / 1000.0)?;

        let avg_tokens = if total > 0 { total_tokens / total } else { 0 };
        let avg_time = if total > 0 { total_time / total as f64 } else { 0.0 };

        // Get capability breakdown
        let by_capability = std::collections::HashMap::new();
        // This would query the actual capability distribution
        // For now, using placeholder

        Ok(AgentStats {
            total_executions: total as usize,
            successful_executions: successful as usize,
            total_tokens: total_tokens as usize,
            avg_tokens_per_agent: avg_tokens as usize,
            total_time_secs: total_time,
            avg_time_per_agent: avg_time,
            by_capability,
        })
    }

    /// Get agent execution history
    pub async fn get_agent_history(&self, limit: usize) -> Result<Vec<AgentHistoryEntry>> {
        let rows = sqlx::query_as::<_, (String, String, String, i64, i64, bool, String)>(
            "SELECT agent_id, agent_type, task, tokens_used, execution_time_ms, success, created_at
             FROM agent_executions
             ORDER BY created_at DESC
             LIMIT ?"
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut history = Vec::new();

        for row in rows {
            let timestamp = DateTime::parse_from_rfc3339(&row.6)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc);

            history.push(AgentHistoryEntry {
                agent_id: row.0,
                agent_type: row.1,
                capability: AgentCapability::CodeWriting, // Would parse from DB
                task: row.2,
                tokens_used: row.3 as usize,
                execution_time_secs: row.4 as f64 / 1000.0,
                success: row.5,
                timestamp,
            });
        }

        Ok(history)
    }

    /// Save an agent execution
    pub async fn save_agent_execution(
        &self,
        agent_id: &str,
        agent_type: &str,
        capability: &AgentCapability,
        task: &str,
        tokens_used: usize,
        execution_time_ms: u64,
        success: bool,
    ) -> Result<()> {
        let capability_str = format!("{:?}", capability);

        sqlx::query(
            "INSERT INTO agent_executions (agent_id, agent_type, capability, task, tokens_used, execution_time_ms, success)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(agent_id)
        .bind(agent_type)
        .bind(capability_str)
        .bind(task)
        .bind(tokens_used as i64)
        .bind(execution_time_ms as i64)
        .bind(success)
        .execute(&self.pool)
        .await
        .context("Failed to save agent execution")?;

        Ok(())
    }
}

pub struct AgentStats {
    pub total_executions: usize,
    pub successful_executions: usize,
    pub total_tokens: usize,
    pub avg_tokens_per_agent: usize,
    pub total_time_secs: f64,
    pub avg_time_per_agent: f64,
    pub by_capability: std::collections::HashMap<AgentCapability, usize>,
}

pub struct AgentHistoryEntry {
    pub agent_id: String,
    pub agent_type: String,
    pub capability: AgentCapability,
    pub task: String,
    pub tokens_used: usize,
    pub execution_time_secs: f64,
    pub success: bool,
    pub timestamp: DateTime<Utc>,
}
