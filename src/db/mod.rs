mod schema;

use crate::agents::AgentCapability;
use crate::analyzer::Optimization;
use crate::config::Config;
use crate::master::orchestrator::{ExecutionPlan, ExecutionResult};
use crate::master::planner::TaskAnalysis;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub use schema::*;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    // TODO: Add tests for Database::new():
    // - Test successful initialization
    // - Test with invalid path (permission denied)
    // - Test with corrupted database file
    // - Test schema migration on existing database
    // - Test concurrent database initialization

    pub async fn new(_config: &Config) -> Result<Self> {
        let db_path = Config::db_file()?;
        let db_url = format!("sqlite://{}", db_path.display());

        // Create pool with create_if_missing option
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                db_url
                    .parse::<sqlx::sqlite::SqliteConnectOptions>()?
                    .create_if_missing(true),
            )
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
    // TODO: Add tests for save_task_execution():
    // - Test successful save and verify data integrity
    // - Test with extremely long task description (>10000 chars)
    // - Test with special characters and Unicode
    // - Test with invalid JSON in analysis/plan
    // - Test concurrent saves (race conditions)
    // - Test transaction rollback on error
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
    // TODO: Add tests for get_agent_stats():
    // - Test with empty database (division by zero)
    // - Test with single record
    // - Test with large dataset (1000+ records)
    // - Test aggregation accuracy
    // - Test with NULL values in tokens_used
    // - Test with negative values (should never happen but handle gracefully)
    pub async fn get_agent_stats(&self) -> Result<AgentStats> {
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_executions")
            .fetch_one(&self.pool)
            .await?;

        let successful: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM agent_executions WHERE success = 1")
                .fetch_one(&self.pool)
                .await?;

        let total_tokens: i64 =
            sqlx::query_scalar("SELECT COALESCE(SUM(tokens_used), 0) FROM agent_executions")
                .fetch_one(&self.pool)
                .await?;

        let total_time: f64 =
            sqlx::query_scalar("SELECT COALESCE(SUM(execution_time_ms), 0) FROM agent_executions")
                .fetch_one(&self.pool)
                .await
                .map(|ms: i64| ms as f64 / 1000.0)?;

        let avg_tokens = if total > 0 { total_tokens / total } else { 0 };
        let avg_time = if total > 0 {
            total_time / total as f64
        } else {
            0.0
        };

        // Get capability breakdown
        let capability_rows = sqlx::query_as::<_, (String, i64)>(
            "SELECT capability, COUNT(*) as count
             FROM agent_executions
             GROUP BY capability",
        )
        .fetch_all(&self.pool)
        .await?;

        let mut by_capability = std::collections::HashMap::new();
        for (capability_str, count) in capability_rows {
            if let Some(capability) = AgentCapability::from_str(&capability_str) {
                by_capability.insert(capability, count as usize);
            }
        }

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
        let rows = sqlx::query_as::<_, (String, String, String, String, i64, i64, bool, String)>(
            "SELECT agent_id, agent_type, capability, task, tokens_used, execution_time_ms, success, created_at
             FROM agent_executions
             ORDER BY created_at DESC
             LIMIT ?",
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut history = Vec::new();

        for row in rows {
            let timestamp = DateTime::parse_from_rfc3339(&row.7)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc);

            // Parse capability from database, fallback to CodeWriting for unknown values
            let capability =
                AgentCapability::from_str(&row.2).unwrap_or(AgentCapability::CodeWriting);

            history.push(AgentHistoryEntry {
                agent_id: row.0,
                agent_type: row.1,
                capability,
                task: row.3,
                tokens_used: row.4 as usize,
                execution_time_secs: row.5 as f64 / 1000.0,
                success: row.6,
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

    /// Get hourly token usage breakdown
    pub async fn get_hourly_breakdown(&self, hours: usize) -> Result<Vec<HourlyBreakdown>> {
        // Security: Use parameterized query to prevent SQL injection
        // Calculate threshold datetime in Rust instead of using string interpolation
        let hours_i64 = hours as i64;
        let threshold = Utc::now() - chrono::Duration::hours(hours_i64);
        let threshold_str = threshold.format("%Y-%m-%d %H:%M:%S").to_string();

        let query = "SELECT
                strftime('%Y-%m-%d %H:00:00', created_at) as hour,
                COUNT(*) as task_count,
                COALESCE(SUM(tokens_used), 0) as total_tokens
             FROM agent_executions
             WHERE datetime(created_at) >= datetime(?)
             GROUP BY hour
             ORDER BY hour DESC";

        let rows = sqlx::query_as::<_, (String, i64, i64)>(query)
            .bind(&threshold_str)
            .fetch_all(&self.pool)
            .await?;

        let mut breakdown = Vec::new();
        for row in rows {
            let timestamp =
                DateTime::parse_from_str(&format!("{} +0000", row.0), "%Y-%m-%d %H:%M:%S %z")
                    .unwrap_or_else(|_| Utc::now().into())
                    .with_timezone(&Utc);

            breakdown.push(HourlyBreakdown {
                hour: timestamp,
                task_count: row.1 as usize,
                total_tokens: row.2 as usize,
            });
        }

        Ok(breakdown)
    }

    /// Get recent task executions summary
    pub async fn get_recent_tasks(&self, limit: usize) -> Result<Vec<TaskSummary>> {
        let rows = sqlx::query_as::<_, (i64, String, i64, bool, String)>(
            "SELECT id, task_description, actual_tokens, success, created_at
             FROM task_executions
             ORDER BY created_at DESC
             LIMIT ?",
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut tasks = Vec::new();
        for row in rows {
            let timestamp = DateTime::parse_from_rfc3339(&row.4)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc);

            tasks.push(TaskSummary {
                id: row.0 as usize,
                description: row.1,
                tokens_used: row.2 as usize,
                success: row.3,
                timestamp,
            });
        }

        Ok(tasks)
    }

    /// Save an optimization suggestion to the database
    pub async fn save_optimization(&self, opt: &Optimization) -> Result<()> {
        let opt_type = format!("{:?}", opt.opt_type);
        let examples_json = serde_json::to_string(&opt.examples)?;

        sqlx::query(
            "INSERT INTO optimizations (optimization_type, title, description, estimated_savings, examples, applied)
             VALUES (?, ?, ?, ?, ?, 0)"
        )
        .bind(opt_type)
        .bind(&opt.title)
        .bind(&opt.description)
        .bind(opt.estimated_savings as i64)
        .bind(examples_json)
        .execute(&self.pool)
        .await
        .context("Failed to save optimization")?;

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

pub struct HourlyBreakdown {
    pub hour: DateTime<Utc>,
    pub task_count: usize,
    pub total_tokens: usize,
}

pub struct TaskSummary {
    pub id: usize,
    pub description: String,
    pub tokens_used: usize,
    pub success: bool,
    pub timestamp: DateTime<Utc>,
}
