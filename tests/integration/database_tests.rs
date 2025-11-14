// Database integration tests
#[path = "../common/mod.rs"]
mod common;

use anyhow::Result;

#[tokio::test]
async fn test_database_initialization() -> Result<()> {
    // Test that database can be created and schema is correct
    let pool = common::setup_test_db().await?;

    // Verify tables exist by querying them
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM task_executions")
        .fetch_one(&pool)
        .await?;

    assert_eq!(count, 0);

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_executions")
        .fetch_one(&pool)
        .await?;

    assert_eq!(count, 0);

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM optimizations")
        .fetch_one(&pool)
        .await?;

    assert_eq!(count, 0);

    Ok(())
}

// TODO: Add more database tests in Phase 3
// - test_save_task_execution
// - test_save_agent_execution
// - test_get_agent_stats_empty_db
// - test_get_agent_stats_with_data
// - test_get_hourly_breakdown
// - test_concurrent_operations
