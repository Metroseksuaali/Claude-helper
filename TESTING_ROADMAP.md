# Claude Helper - Testing Implementation Roadmap

**Status**: Analysis Complete - Ready for Implementation  
**Date**: 2025-11-14  
**Severity**: HIGH - No automated tests exist  

---

## Summary for Developers

### Current Situation
- **0 automated tests** in the codebase
- **Manual testing only** on CLI and configuration
- **Critical business logic untested**: task analysis, orchestration, database queries
- **Test infrastructure ready**: dependencies already in Cargo.toml (mockito, tempfile)

### Why This Matters
Three categories of code are currently untested:
1. **Algorithms**: Complexity calculation, dependency resolution, optimization detection
2. **Database**: All CRUD operations and aggregation queries
3. **Orchestration**: Parallel execution, autonomy modes, agent coordination

A single bug in any of these could cost users significant money (token overspend) or incorrect results.

---

## Implementation Path (36 Hours Total)

### Week 1: Foundation (12 Hours)

**Objective**: Establish test infrastructure and test 3 critical functions

```
Monday-Tuesday (6 hours):
├─ Create tests/common/mod.rs with shared fixtures
├─ Create tests/integration/database_tests.rs
└─ Add #[cfg(test)] modules to planner.rs, optimizer.rs

Wednesday-Thursday (4 hours):
├─ Implement unit tests for estimate_complexity() (8 tests)
├─ Implement unit tests for detect_capabilities() (6 tests)
└─ Run: cargo test

Friday (2 hours):
├─ Code review and fixes
├─ Document test patterns
└─ Update TESTING.md
```

**Success Criteria**:
- ✅ 14 unit tests passing
- ✅ Test infrastructure documented
- ✅ No compilation errors

### Week 2: Core Algorithms (10 Hours)

**Objective**: Test complex algorithms used in planning and analysis

```
Monday-Tuesday (5 hours):
├─ Implement tests for create_phases() (10 tests)
│  ├─ Single agent, linear chains, diamond dependencies
│  ├─ Circular dependency detection
│  └─ Parallel flag detection
└─ Run: cargo test test_create_phases

Wednesday-Thursday (3 hours):
├─ Implement tests for optimizer algorithms
│  ├─ detect_bash_chains() (5 tests)
│  ├─ detect_file_patterns() (5 tests)
│  └─ detect_tool_repetition() (4 tests)
└─ Verify savings calculations

Friday (2 hours):
├─ Implement tests for token calculations (8 tests)
└─ Code review
```

**Success Criteria**:
- ✅ 32 algorithm tests passing
- ✅ Edge cases covered
- ✅ No panics or infinite loops

### Week 3: Integration (8 Hours)

**Objective**: Test database operations and configuration

```
Monday-Wednesday (5 hours):
├─ Implement database integration tests (8 tests)
│  ├─ save_task_execution()
│  ├─ save_agent_execution()
│  ├─ get_agent_stats() with aggregation
│  ├─ get_agent_history() with pagination
│  ├─ get_hourly_breakdown() - FIX SQL INJECTION!
│  └─ Concurrent access patterns
└─ Fix any database issues found

Thursday (2 hours):
├─ Implement configuration tests (5 tests)
├─ Implement session parsing tests (6 tests)
└─ Run full integration suite

Friday (1 hour):
├─ Code review
└─ Performance analysis
```

**Success Criteria**:
- ✅ 19 integration tests passing
- ✅ SQL injection vulnerability fixed
- ✅ 70% code coverage achieved

### Week 4: Orchestration & Polish (6 Hours)

**Objective**: System tests and edge cases

```
Monday-Tuesday (3 hours):
├─ Implement orchestrator tests (10 tests)
│  ├─ Single phase execution
│  ├─ Parallel execution with semaphore
│  ├─ Sequential execution with dependencies
│  ├─ Autonomy modes (conservative, balanced, trust)
│  └─ Error handling and recovery
└─ Run with mock agents

Wednesday-Friday (3 hours):
├─ Property-based tests for edge cases (optional)
├─ Performance testing
├─ Coverage report (target: 80%)
├─ Security audit for remaining issues
└─ Documentation finalization
```

**Success Criteria**:
- ✅ 10 orchestration tests passing
- ✅ 80%+ line coverage
- ✅ All critical paths tested
- ✅ Test documentation complete

---

## Task-by-Task Breakdown

### ⏱️ PHASE 1 (Week 1)

#### Task 1.1: Create Test Infrastructure (2 hours)

**Files to create:**
- `tests/common/mod.rs` - Shared utilities
- `tests/integration/database_tests.rs` - DB tests stub

**Files to modify:**
- `Cargo.toml` - Add tokio test feature (already present)

**Steps:**
```bash
mkdir -p tests/common tests/integration tests/fixtures/sessions

cat > tests/common/mod.rs << 'RUST'
use tempfile::TempDir;
use sqlx::SqlitePool;
use crate::master::planner::TaskAnalysis;
use crate::agents::AgentCapability;

pub fn sample_task_analysis() -> TaskAnalysis {
    TaskAnalysis {
        task_description: "Write tests".to_string(),
        complexity: 5,
        estimated_files: 3,
        estimated_tokens: 6000,
        estimated_time_min: 5,
        estimated_time_max: 15,
        required_capabilities: vec![AgentCapability::Testing],
        keywords: vec!["write".to_string(), "tests".to_string()],
    }
}

pub async fn setup_test_db() -> (TempDir, Database) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    // Create test database in temp directory
    // ...
}
RUST
```

#### Task 1.2: Implement Complexity Tests (2.5 hours)

**File**: `src/master/planner.rs` (append to file)

**Test cases** (8 tests):
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    
    fn setup() -> TaskPlanner {
        TaskPlanner::new(Config::default())
    }
    
    #[test]
    fn test_estimate_complexity_empty_string() { }
    
    #[test]
    fn test_estimate_complexity_base_value() { }
    
    #[test]
    fn test_estimate_complexity_high_keyword_refactor() { }
    
    #[test]
    fn test_estimate_complexity_medium_keyword_implement() { }
    
    #[test]
    fn test_estimate_complexity_multiple_requirements_bonus() { }
    
    #[test]
    fn test_estimate_complexity_capped_at_10() { }
    
    #[test]
    fn test_estimate_complexity_case_insensitive() { }
    
    #[test]
    fn test_estimate_complexity_keyword_appears_twice() { }
}
```

**Expected behavior**:
- Empty string → 3 (base)
- "write a function" → 4 (base + medium keyword)
- "refactor authentication" → 5 (base + high keyword)
- "refactor AND migrate AND security" → 10 (capped, not 9)

#### Task 1.3: Implement Capability Detection Tests (2.5 hours)

**File**: `src/master/planner.rs` (same module)

**Test cases** (6 tests):
```rust
#[test]
fn test_detect_capabilities_empty() { }

#[test]
fn test_detect_capabilities_single_keyword() { }

#[test]
fn test_detect_capabilities_multiple_keywords() { }

#[test]
fn test_detect_capabilities_default_code_writing() { }

#[test]
fn test_detect_capabilities_security_audit() { }

#[test]
fn test_detect_capabilities_testing_and_security() { }
```

#### Task 1.4: Integration Test Setup (1.5 hours)

**File**: `tests/integration/database_tests.rs`

```rust
#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_database_initialization() {
        // Test that database can be created
        // and schema is correct
    }
}
```

---

### ⏱️ PHASE 2 (Week 2)

#### Task 2.1: Dependency Resolution Tests (4 hours)

**File**: `src/master/planner.rs` (add to tests module)

**Test cases** (10 tests):

```rust
#[test]
fn test_create_phases_empty_agents() { }

#[test]
fn test_create_phases_single_agent() { }

#[test]
fn test_create_phases_linear_chain_a_b_c() { }

#[test]
fn test_create_phases_diamond_a_to_bc_to_d() { }

#[test]
fn test_create_phases_fully_parallel() { }

#[test]
fn test_create_phases_circular_dependency() { }

#[test]
fn test_create_phases_parallel_flag_correct() { }

#[test]
fn test_create_phases_agent_ordering_preserved() { }

#[test]
fn test_create_phases_large_100_agents() { }

#[test]
fn test_create_phases_complex_multipath() { }
```

#### Task 2.2: Optimizer Tests (3 hours)

**File**: `src/analyzer/optimizer.rs` (add to file)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_bash_chains_empty_session() { }
    
    #[test]
    fn test_detect_bash_chains_git_operations() { }
    
    // ... 13 more tests
}
```

#### Task 2.3: Token/Cost Calculation Tests (2 hours)

**File**: `src/statusline/usage_tracker.rs` (add to file)

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_percentage_zero_usage() { }
    
    #[test]
    fn test_percentage_at_limit() { }
    
    // ... 6 more tests
}
```

---

### ⏱️ PHASE 3 (Week 3)

#### Task 3.1: Database Integration Tests (5 hours)

**File**: `tests/integration/database_tests.rs`

```rust
#[tokio::test]
async fn test_save_task_execution() { }

#[tokio::test]
async fn test_save_agent_execution() { }

#[tokio::test]
async fn test_get_agent_stats_empty_db() { }

#[tokio::test]
async fn test_get_agent_stats_with_data() { }

#[tokio::test]
async fn test_get_agent_history_pagination() { }

#[tokio::test]
async fn test_get_hourly_breakdown_time_range() { }

#[tokio::test]
async fn test_get_recent_tasks_limit() { }

#[tokio::test]
async fn test_concurrent_insertions() { }
```

**⚠️ SECURITY FIX NEEDED**:
```rust
// BEFORE (vulnerable):
let query = format!(
    "SELECT ... WHERE datetime(created_at) >= datetime('now', '-{} hours')",
    hours
);

// AFTER (safe):
let query = "SELECT ... WHERE datetime(created_at) >= datetime('now', ?)";
sqlx::query(&query)
    .bind(format!("-{} hours", hours))
    // ...
```

#### Task 3.2: Config Tests (2 hours)

**File**: `tests/integration/config_tests.rs`

```rust
#[tokio::test]
async fn test_config_load_creates_default() { }

#[tokio::test]
async fn test_config_save_roundtrip() { }

#[test]
fn test_config_dir_creation() { }

#[test]
fn test_default_values_valid() { }

#[test]
fn test_config_validation() { }
```

#### Task 3.3: Session Parser Tests (1.5 hours)

**File**: `tests/integration/session_parser_tests.rs`

```rust
#[test]
fn test_parse_session_empty_file() { }

#[test]
fn test_parse_session_valid_jsonl() { }

#[test]
fn test_parse_session_extracts_tool_calls() { }

// ... more tests
```

---

### ⏱️ PHASE 4 (Week 4)

#### Task 4.1: Orchestrator Tests (3 hours)

**File**: `tests/integration/orchestration_tests.rs`

```rust
#[tokio::test]
async fn test_execute_plan_sequential() { }

#[tokio::test]
async fn test_execute_plan_parallel() { }

#[tokio::test]
async fn test_autonomy_mode_conservative() { }

// ... more tests
```

#### Task 4.2: Code Review & Coverage (2 hours)

```bash
cargo test              # Run all tests
cargo tarpaulin         # Generate coverage report
cargo clippy            # Check code quality
cargo fmt               # Format code
```

#### Task 4.3: Documentation (1 hour)

- Update TESTING.md with test results
- Document test patterns used
- Create examples for future test additions

---

## Testing Checklist Template

Print and use this while implementing:

```
PHASE 1 - FOUNDATION
[  ] Create tests/common/mod.rs
[  ] Create tests/integration/database_tests.rs
[  ] Add #[cfg(test)] to planner.rs
[  ] Implement 8 complexity tests
[  ] Implement 6 capability tests
[  ] All Phase 1 tests passing
[  ] Code review completed

PHASE 2 - CORE ALGORITHMS
[  ] Implement 10 dependency resolution tests
[  ] Implement 14 optimizer tests
[  ] Implement 8 token calculation tests
[  ] All Phase 2 tests passing (32 total)
[  ] Edge cases identified and covered
[  ] Code review completed

PHASE 3 - INTEGRATION
[  ] Create database tests (8 tests)
[  ] Fix SQL injection vulnerability
[  ] Create config tests (5 tests)
[  ] Create session parser tests (6 tests)
[  ] All Phase 3 tests passing (19 total)
[  ] Coverage report shows 70%+
[  ] Code review completed

PHASE 4 - POLISH
[  ] Create orchestrator tests (10 tests)
[  ] Coverage report shows 80%+
[  ] All security issues resolved
[  ] TESTING.md updated
[  ] Test documentation complete
[  ] Final code review
[  ] CI/CD ready
```

---

## CI/CD Integration (Future)

Once tests are in place, add GitHub Actions:

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
      - run: cargo test
      - run: cargo tarpaulin --out Xml
      - uses: codecov/codecov-action@v2
```

---

## Success Metrics

| Metric | Phase 1 | Phase 2 | Phase 3 | Phase 4 |
|--------|---------|---------|---------|---------|
| Unit Tests | 14 | 46 | 46 | 56 |
| Integration Tests | 0 | 0 | 19 | 29 |
| Line Coverage | 15% | 45% | 70% | 80%+ |
| Critical Path Tested | 30% | 70% | 85% | 100% |
| Known Issues | 0 | 0 | -1* | 0 |

*SQL injection vulnerability fixed in Phase 3

---

## Reference Files

All supporting documents are in the repository:
- `TEST_COVERAGE_ANALYSIS.md` - Detailed technical analysis
- `TESTING_QUICK_REFERENCE.md` - Quick lookup guide
- `CLAUDE.md` - Architecture and code structure
- `TESTING.md` - Manual test results

---

**Created**: 2025-11-14
**Updated**: 2025-11-14 (Expanded with advanced testing scenarios)
**Estimated Time**: 60 hours (6 weeks at 10 hours/week)
**Skill Level**: Intermediate to Advanced Rust developer
**Next Step**: Start Phase 1, Task 1.1

---

## PHASE 5: Performance & Benchmarking (8 Hours)

### Week 5: Performance Testing

**Objective**: Ensure the system performs within acceptable parameters

#### Task 5.1: Benchmark Critical Paths (4 hours)

**File**: `benches/critical_paths.rs` (create new)

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use claude_helper::master::planner::TaskPlanner;

fn bench_complexity_calculation(c: &mut Criterion) {
    c.bench_function("estimate_complexity simple", |b| {
        b.iter(|| {
            black_box(estimate_complexity("fix typo"))
        })
    });

    c.bench_function("estimate_complexity complex", |b| {
        b.iter(|| {
            black_box(estimate_complexity(
                "refactor migrate security architecture with testing and documentation"
            ))
        })
    });
}

fn bench_dependency_resolution(c: &mut Criterion) {
    let mut group = c.benchmark_group("dependency_resolution");

    // Benchmark with different agent counts
    for agent_count in [5, 10, 50, 100, 500].iter() {
        group.bench_with_input(
            format!("{}_agents", agent_count),
            agent_count,
            |b, &size| {
                let specs = generate_agent_specs(size);
                b.iter(|| create_phases(&specs))
            },
        );
    }
    group.finish();
}

fn bench_database_queries(c: &mut Criterion) {
    c.bench_function("insert_agent_execution", |b| {
        // Test database write performance
    });

    c.bench_function("get_agent_stats_1000_records", |b| {
        // Test aggregation query performance
    });
}

criterion_group!(
    benches,
    bench_complexity_calculation,
    bench_dependency_resolution,
    bench_database_queries
);
criterion_main!(benches);
```

**Add to Cargo.toml**:
```toml
[[bench]]
name = "critical_paths"
harness = false

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
```

**Performance Targets**:
- Complexity calculation: < 1μs
- Dependency resolution (100 agents): < 10ms
- Database insert: < 5ms
- Status line generation: < 8ms (already achieved)

#### Task 5.2: Load Testing (2 hours)

**File**: `tests/load/stress_tests.rs`

```rust
#[tokio::test]
async fn test_concurrent_1000_agent_executions() {
    // Simulate 1000 concurrent agent executions
    let db = setup_test_db().await;
    let mut handles = vec![];

    for i in 0..1000 {
        let db_clone = db.clone();
        handles.push(tokio::spawn(async move {
            db_clone.save_agent_execution(/* ... */).await
        }));
    }

    // All should complete without deadlocks or errors
    for handle in handles {
        assert!(handle.await.is_ok());
    }
}

#[tokio::test]
async fn test_large_session_parsing() {
    // Test with 10MB JSONL session file
    let large_session = generate_large_session(10_000); // 10k messages
    let result = SessionParser::parse(&large_session).await;
    assert!(result.is_ok());
}

#[test]
fn test_memory_usage_100_agents() {
    // Monitor memory usage during agent creation
    let initial_memory = get_current_memory_usage();

    let agents = create_100_agents();

    let final_memory = get_current_memory_usage();
    let memory_per_agent = (final_memory - initial_memory) / 100;

    // Should be under 100KB per agent
    assert!(memory_per_agent < 100_000);
}
```

#### Task 5.3: Memory Profiling (2 hours)

**Setup memory profiling**:
```bash
# Install valgrind/massif
sudo apt-get install valgrind

# Profile memory usage
valgrind --tool=massif --massif-out-file=massif.out ./target/release/claude-helper run "complex task"

# Analyze results
ms_print massif.out
```

**Create memory leak tests**:
```rust
#[test]
fn test_no_memory_leaks_in_orchestrator() {
    for _ in 0..1000 {
        let orchestrator = Orchestrator::new();
        orchestrator.execute_plan(/* ... */);
        // orchestrator should be dropped and memory freed
    }
    // Memory usage should stabilize
}
```

---

## PHASE 6: Error Handling & Edge Cases (10 Hours)

### Week 6: Comprehensive Error Testing

**Objective**: Test all error paths and edge cases

#### Task 6.1: Database Error Scenarios (3 hours)

**File**: `tests/integration/database_error_tests.rs`

```rust
#[tokio::test]
async fn test_database_connection_failure() {
    // Test with invalid connection string
    let result = Database::new("invalid://path").await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), DatabaseError::ConnectionFailed));
}

#[tokio::test]
async fn test_database_disk_full() {
    // Simulate disk full scenario (if possible in testing)
}

#[tokio::test]
async fn test_database_corrupted_data() {
    // Insert invalid data and test recovery
}

#[tokio::test]
async fn test_database_concurrent_write_conflict() {
    // Test SQLite BUSY handling
}

#[tokio::test]
async fn test_transaction_rollback() {
    // Test that failed transactions roll back properly
}

#[tokio::test]
async fn test_schema_migration_failure() {
    // Test handling of migration errors
}
```

#### Task 6.2: API Error Scenarios (3 hours)

**File**: `tests/integration/api_error_tests.rs`

```rust
#[tokio::test]
async fn test_api_rate_limit_429() {
    // Mock 429 response from Claude API
    let mock_server = mockito::Server::new();
    let mock = mock_server.mock("POST", "/messages")
        .with_status(429)
        .with_header("retry-after", "60")
        .create();

    let agent = ClaudeAgent::new_with_endpoint(mock_server.url());
    let result = agent.execute("task").await;

    assert!(matches!(result.unwrap_err(), AgentError::RateLimitExceeded));
}

#[tokio::test]
async fn test_api_timeout() {
    // Test timeout handling
}

#[tokio::test]
async fn test_api_invalid_response() {
    // Test malformed JSON response
}

#[tokio::test]
async fn test_api_authentication_failure() {
    // Test 401/403 responses
}

#[tokio::test]
async fn test_api_network_failure() {
    // Test network disconnect
}
```

#### Task 6.3: Input Validation Tests (2 hours)

**File**: Throughout codebase with TODO comments

```rust
// In src/master/planner.rs
#[test]
fn test_validate_task_description_empty() {
    // TODO: Add validation - empty task should be rejected
}

#[test]
fn test_validate_task_description_too_long() {
    // TODO: Add validation - task > 10,000 chars should be rejected
}

#[test]
fn test_validate_token_budget_negative() {
    // TODO: Add validation - negative budget should be rejected
}

#[test]
fn test_validate_max_agents_zero() {
    // TODO: Add validation - max_agents must be >= 1
}

// In src/config/mod.rs
#[test]
fn test_config_invalid_toml() {
    let invalid_toml = "this is [ not valid toml";
    let result = Config::from_str(&invalid_toml);
    assert!(result.is_err());
}

#[test]
fn test_config_missing_required_fields() {
    // TODO: Determine which fields are truly required
}
```

#### Task 6.4: Edge Case Matrix (2 hours)

Create comprehensive edge case test suite:

**File**: `tests/edge_cases/mod.rs`

```rust
mod boundary_values {
    #[test] fn test_complexity_min_0() { }
    #[test] fn test_complexity_max_10() { }
    #[test] fn test_complexity_overflow_255() { }
    #[test] fn test_tokens_zero() { }
    #[test] fn test_tokens_max_u64() { }
    #[test] fn test_agents_empty_vec() { }
    #[test] fn test_agents_single() { }
    #[test] fn test_agents_1000_parallel() { }
}

mod unicode_handling {
    #[test] fn test_task_with_emoji() { }
    #[test] fn test_task_with_chinese_chars() { }
    #[test] fn test_task_with_rtl_text() { }
    #[test] fn test_task_with_null_bytes() { }
    #[test] fn test_task_with_control_chars() { }
}

mod malformed_data {
    #[test] fn test_session_file_invalid_json() { }
    #[test] fn test_session_file_truncated() { }
    #[test] fn test_session_file_binary_data() { }
    #[test] fn test_config_partial_corruption() { }
}

mod resource_exhaustion {
    #[test] fn test_deeply_nested_dependencies_stack_overflow() { }
    #[test] fn test_infinite_loop_protection() { } // Already fixed!
    #[test] fn test_large_file_parsing_oom() { }
}
```

---

## PHASE 7: Security Testing (6 Hours)

### Week 7: Security Audit & Fuzzing

**Objective**: Identify and fix security vulnerabilities

#### Task 7.1: Security Test Suite (3 hours)

**File**: `tests/security/vulnerability_tests.rs`

```rust
#[tokio::test]
async fn test_sql_injection_prevention() {
    // Attempt various SQL injection patterns
    let db = setup_test_db().await;

    let malicious_inputs = vec![
        "'; DROP TABLE agent_executions; --",
        "1 OR 1=1",
        "1'; UPDATE agent_executions SET tokens_used=999999; --",
        "1 UNION SELECT * FROM sqlite_master",
    ];

    for input in malicious_inputs {
        // Should be safely handled by parameterized queries
        let result = db.get_hourly_breakdown(input.parse().unwrap_or(24)).await;
        // Should either succeed safely or return error, never execute injection
        assert!(result.is_ok() || result.is_err());

        // Verify database integrity
        let count = db.count_executions().await.unwrap();
        assert_eq!(count, 0); // No records should exist yet
    }
}

#[test]
fn test_path_traversal_prevention() {
    // Test that config/session paths can't escape directories
    let malicious_paths = vec![
        "../../../etc/passwd",
        "..\\..\\..\\windows\\system32",
        "/etc/shadow",
        "~/../../root/.ssh/id_rsa",
    ];

    for path in malicious_paths {
        let result = load_session_from_path(path);
        assert!(result.is_err());
    }
}

#[test]
fn test_command_injection_prevention() {
    // If we ever shell out, test command injection protection
    // Currently not applicable, but add as safeguard
}

#[tokio::test]
async fn test_dos_large_input() {
    // Test that large inputs don't cause DoS
    let huge_task = "x".repeat(10_000_000); // 10MB string
    let result = analyze_task(&huge_task).await;

    // Should either handle gracefully or return error quickly
    // Should NOT consume all memory or hang
}

#[test]
fn test_sensitive_data_not_logged() {
    // Ensure API keys don't appear in logs
    let config = Config::default();
    config.auth.api_key = Some("sk-super-secret-key".to_string());

    let log_output = capture_logs(|| {
        // Perform operations that might log
        config.validate();
    });

    assert!(!log_output.contains("sk-super-secret-key"));
    assert!(log_output.contains("sk-***") || !log_output.contains("sk-"));
}
```

#### Task 7.2: Fuzzing Setup (3 hours)

**File**: `fuzz/fuzz_targets/complexity.rs`

```rust
#![no_main]
use libfuzzer_sys::fuzz_target;
use claude_helper::master::planner::estimate_complexity;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // Should never panic, only return valid 0-10 range
        let result = estimate_complexity(s);
        assert!(result <= 10);
    }
});
```

**File**: `fuzz/fuzz_targets/session_parser.rs`

```rust
#![no_main]
use libfuzzer_sys::fuzz_target;
use claude_helper::analyzer::session_parser::SessionParser;

fuzz_target!(|data: &[u8]| {
    // Should handle any binary data without panicking
    let _ = SessionParser::parse_bytes(data);
});
```

**Add to Cargo.toml**:
```toml
[dependencies]
cargo-fuzz = "0.12"
```

**Run fuzzing**:
```bash
cargo install cargo-fuzz
cargo fuzz run complexity -- -max_total_time=300  # 5 minutes
cargo fuzz run session_parser -- -max_total_time=300
```

---

## PHASE 8: Property-Based Testing (6 Hours)

### Week 8: QuickCheck / Proptest Integration

**Objective**: Generate thousands of random test cases automatically

#### Task 8.1: Property Tests for Planner (3 hours)

**Add to Cargo.toml**:
```toml
[dev-dependencies]
proptest = "1.5"
```

**File**: `src/master/planner.rs` (add to tests module)

```rust
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn complexity_always_in_range(task in "\\PC*") {
            let complexity = estimate_complexity(&task);
            prop_assert!(complexity <= 10);
            prop_assert!(complexity >= 0);
        }

        #[test]
        fn complexity_deterministic(task in "\\PC{1,1000}") {
            let result1 = estimate_complexity(&task);
            let result2 = estimate_complexity(&task);
            prop_assert_eq!(result1, result2);
        }

        #[test]
        fn dependency_resolution_terminates(
            agents in prop::collection::vec(arbitrary_agent_spec(), 1..100)
        ) {
            // Should always terminate, even with random dependencies
            let phases = create_phases(&agents);
            prop_assert!(!phases.is_empty());
        }

        #[test]
        fn no_agent_lost_in_phases(
            agents in prop::collection::vec(arbitrary_agent_spec(), 1..50)
        ) {
            let original_count = agents.len();
            let phases = create_phases(&agents);
            let phase_count: usize = phases.iter().map(|p| p.agents.len()).sum();

            // All agents should appear exactly once
            prop_assert_eq!(original_count, phase_count);
        }
    }

    fn arbitrary_agent_spec() -> impl Strategy<Value = AgentSpec> {
        (any::<String>(), prop::collection::vec(any::<String>(), 0..5))
            .prop_map(|(id, deps)| AgentSpec {
                id,
                dependencies: deps,
                // ...
            })
    }
}
```

#### Task 8.2: Property Tests for Database (2 hours)

```rust
proptest! {
    #[test]
    fn save_and_retrieve_roundtrip(
        tokens in 0u64..1_000_000,
        complexity in 0u8..10,
    ) {
        let db = setup_test_db().await;

        // Save
        db.save_agent_execution(/* ... */, tokens, complexity).await.unwrap();

        // Retrieve
        let stats = db.get_agent_stats().await.unwrap();

        prop_assert_eq!(stats.total_tokens, tokens);
    }
}
```

#### Task 8.3: Property Tests for Optimizer (1 hour)

```rust
proptest! {
    #[test]
    fn optimization_savings_non_negative(
        session_data in arbitrary_session_data()
    ) {
        let optimizations = analyze_session(&session_data);

        for opt in optimizations {
            prop_assert!(opt.estimated_savings >= 0);
        }
    }
}
```

---

## PHASE 9: Integration & System Tests (8 Hours)

### Week 9: End-to-End Testing

**Objective**: Test complete workflows with mocked external dependencies

#### Task 9.1: Full Workflow Tests (4 hours)

**File**: `tests/e2e/full_workflow_tests.rs`

```rust
#[tokio::test]
async fn test_full_workflow_simple_task() {
    // Setup
    let config = test_config();
    let db = setup_test_db().await;
    let mock_api = setup_mock_claude_api().await;

    // Execute full workflow
    let master = MasterCoder::new(config, db);
    let result = master.run("Fix typo in README", AutonomyMode::Trust).await;

    // Verify
    assert!(result.is_ok());
    assert_eq!(result.unwrap().agents_used, 1);

    // Verify database updated
    let stats = db.get_agent_stats().await.unwrap();
    assert_eq!(stats.total_executions, 1);

    // Verify API called correctly
    mock_api.assert();
}

#[tokio::test]
async fn test_full_workflow_complex_multiagent() {
    let master = MasterCoder::new(test_config(), setup_test_db().await);
    let result = master.run(
        "Implement OAuth2 with security audit, tests, and documentation",
        AutonomyMode::Balanced
    ).await;

    assert!(result.is_ok());
    assert!(result.unwrap().agents_used >= 4); // Auth, Security, Testing, Docs
}
```

#### Task 9.2: Regression Tests (2 hours)

**File**: `tests/regression/known_issues.rs`

```rust
// Document and test previously fixed bugs to prevent regression

#[tokio::test]
async fn test_issue_001_sql_injection_fixed() {
    // Test that SQL injection vulnerability stays fixed
    let db = setup_test_db().await;
    let result = db.get_hourly_breakdown(24).await;
    assert!(result.is_ok());
}

#[test]
fn test_issue_002_circular_dependency_detection() {
    // Test that circular dependencies are detected
    let specs = vec![
        AgentSpec { id: "A", dependencies: vec!["B"] },
        AgentSpec { id: "B", dependencies: vec!["A"] },
    ];

    let phases = create_phases(&specs);

    // Should handle gracefully with fallback
    assert!(!phases.is_empty());
}

#[test]
fn test_issue_003_nan_in_success_rate() {
    // Test division by zero when no agents executed
    let stats = AgentStats { total_executions: 0, successful: 0, /* ... */ };
    let success_rate = stats.success_rate();

    // Should not be NaN
    assert!(!success_rate.is_nan());
}
```

#### Task 9.3: Smoke Tests (2 hours)

**File**: `tests/smoke/critical_paths.rs`

```rust
// Quick tests that can run in CI on every commit

#[tokio::test]
async fn smoke_config_loads() {
    let config = Config::load().await;
    assert!(config.is_ok());
}

#[tokio::test]
async fn smoke_database_initializes() {
    let db = Database::new_in_memory().await;
    assert!(db.is_ok());
}

#[test]
fn smoke_all_commands_have_help() {
    let cli = Cli::parse_from(&["claude-helper", "--help"]);
    // Should not panic
}

#[tokio::test]
async fn smoke_status_line_renders() {
    let statusline = StatusLine::new();
    let output = statusline.render();
    assert!(!output.is_empty());
}
```

---

## Testing Best Practices

### Mock Strategy

**Use mocks for external dependencies:**

```rust
// Good: Mock HTTP responses
#[tokio::test]
async fn test_with_mock_api() {
    let mut server = mockito::Server::new();
    let mock = server.mock("POST", "/messages")
        .with_status(200)
        .with_body(r#"{"content":[{"text":"response"}]}"#)
        .create();

    // Test with mocked endpoint
}

// Good: In-memory database for speed
#[tokio::test]
async fn test_with_memory_db() {
    let db = SqlitePool::connect(":memory:").await?;
    // Fast tests without disk I/O
}

// Bad: Real API calls in tests
#[tokio::test]
async fn bad_test() {
    let api_key = env::var("ANTHROPIC_API_KEY").unwrap(); // Don't do this!
    let agent = ClaudeAgent::new(api_key);
    agent.execute("task").await // Slow, expensive, flaky
}
```

### Test Data Management

**File**: `tests/fixtures/README.md`

```markdown
# Test Fixtures

## Sessions
- `simple_session.jsonl` - Single message session
- `complex_session.jsonl` - Multi-agent with tool calls
- `malformed_session.jsonl` - Invalid JSON for error testing

## Configs
- `valid_config.toml` - Standard configuration
- `minimal_config.toml` - Only required fields
- `invalid_config.toml` - Malformed TOML
```

### Coverage Goals

| Component | Target Coverage | Priority |
|-----------|----------------|----------|
| Planner | 95%+ | CRITICAL |
| Orchestrator | 90%+ | CRITICAL |
| Database | 85%+ | HIGH |
| Agents | 80%+ | HIGH |
| Optimizer | 80%+ | MEDIUM |
| Config | 75%+ | MEDIUM |
| TUI | 60%+ | LOW |
| CLI | 70%+ | MEDIUM |

---

## Updated Timeline

| Phase | Hours | Cumulative | Coverage | Tests |
|-------|-------|------------|----------|-------|
| Phase 1: Foundation | 12 | 12 | 15% | 14 |
| Phase 2: Core Algorithms | 10 | 22 | 45% | 46 |
| Phase 3: Integration | 8 | 30 | 70% | 65 |
| Phase 4: Orchestration | 6 | 36 | 80% | 75 |
| Phase 5: Performance | 8 | 44 | 80% | 85 |
| Phase 6: Error Handling | 10 | 54 | 85% | 125 |
| Phase 7: Security | 6 | 60 | 85% | 140 |
| Phase 8: Property-Based | 6 | 66 | 90% | 200+ |
| Phase 9: E2E & Regression | 8 | 74 | 92% | 220+ |

**Total: 74 hours over 9 weeks at ~8 hours/week**

---

## Tools & Commands Reference

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_complexity

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel
cargo test -- --test-threads=8

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage/

# Run benchmarks
cargo bench

# Run fuzzing (5 minutes each target)
cargo fuzz run complexity -- -max_total_time=300

# Check for memory leaks
valgrind --leak-check=full ./target/debug/claude-helper

# Profile performance
cargo flamegraph --bin claude-helper

# Property-based testing
cargo test --features proptest

# Run only fast tests (unit + smoke)
cargo test --lib

# Run only slow tests (integration)
cargo test --test '*'

# Watch mode for TDD
cargo watch -x test
```

---

## Continuous Integration Configuration

**File**: `.github/workflows/tests.yml`

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run tests
        run: cargo test --all-features

      - name: Run clippy
        run: cargo clippy -- -D warnings

      - name: Check formatting
        run: cargo fmt -- --check

      - name: Generate coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --out Xml

      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./cobertura.xml

  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Run benchmarks
        run: cargo bench --no-fail-fast

      - name: Store benchmark results
        uses: benchmark-action/github-action-benchmark@v1

  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Security audit
        run: |
          cargo install cargo-audit
          cargo audit

      - name: Dependency check
        run: |
          cargo install cargo-deny
          cargo deny check
```

---

**Created**: 2025-11-14
**Updated**: 2025-11-14 (Extended to 9 phases with 220+ tests)
**Estimated Time**: 74 hours (9 weeks at 8 hours/week)
**Skill Level**: Intermediate to Advanced Rust developer
**Next Step**: Start Phase 1, Task 1.1
**Target**: 92% code coverage, 220+ comprehensive tests
