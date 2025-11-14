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
**Estimated Time**: 36 hours (4 weeks at 9 hours/week)  
**Skill Level**: Intermediate Rust developer  
**Next Step**: Start Phase 1, Task 1.1
