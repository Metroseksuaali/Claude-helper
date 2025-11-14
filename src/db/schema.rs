pub const CREATE_TABLES: &str = "
CREATE TABLE IF NOT EXISTS task_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_description TEXT NOT NULL,
    complexity INTEGER NOT NULL,
    estimated_tokens INTEGER NOT NULL,
    actual_tokens INTEGER NOT NULL,
    success BOOLEAN NOT NULL,
    task_data TEXT NOT NULL,
    plan_data TEXT NOT NULL,
    result_data TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agent_executions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id TEXT NOT NULL,
    agent_type TEXT NOT NULL,
    capability TEXT NOT NULL,
    task TEXT NOT NULL,
    tokens_used INTEGER NOT NULL,
    execution_time_ms INTEGER NOT NULL,
    success BOOLEAN NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS optimizations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    optimization_type TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    estimated_savings INTEGER NOT NULL,
    applied BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_task_executions_created ON task_executions(created_at);
CREATE INDEX IF NOT EXISTS idx_agent_executions_created ON agent_executions(created_at);
CREATE INDEX IF NOT EXISTS idx_agent_executions_type ON agent_executions(agent_type);
";
