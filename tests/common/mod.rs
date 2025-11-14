// Common test utilities and fixtures
use anyhow::Result;
use sqlx::SqlitePool;
use tempfile::TempDir;

/// Create an in-memory SQLite database for testing
pub async fn setup_test_db() -> Result<SqlitePool> {
    let pool = SqlitePool::connect(":memory:").await?;

    // Run schema initialization
    sqlx::query(claude_helper::db::CREATE_TABLES)
        .execute(&pool)
        .await?;

    Ok(pool)
}

/// Create a temporary directory that will be cleaned up automatically
pub fn temp_dir() -> Result<TempDir> {
    Ok(TempDir::new()?)
}

/// Sample task descriptions for testing
pub mod sample_tasks {
    pub const EMPTY: &str = "";
    pub const SIMPLE: &str = "fix typo in README";
    pub const MEDIUM: &str = "implement user authentication";
    pub const COMPLEX: &str = "refactor migrate security architecture with testing and documentation";
    pub const HIGH_COMPLEXITY: &str = "refactor the authentication system";
    pub const MULTIPLE_REQUIREMENTS: &str = "implement OAuth and add tests";
}

/// Sample agent capabilities for testing
pub mod sample_capabilities {
    use claude_helper::agents::AgentCapability;

    pub fn all_capabilities() -> Vec<AgentCapability> {
        vec![
            AgentCapability::Architecture,
            AgentCapability::CodeWriting,
            AgentCapability::Testing,
            AgentCapability::Security,
            AgentCapability::Documentation,
            AgentCapability::Debugging,
            AgentCapability::Performance,
            AgentCapability::Migration,
            AgentCapability::Review,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_setup_test_db() {
        let pool = setup_test_db().await;
        assert!(pool.is_ok());
    }

    #[test]
    fn test_temp_dir() {
        let dir = temp_dir();
        assert!(dir.is_ok());
    }
}
