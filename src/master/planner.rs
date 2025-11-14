use super::orchestrator::{AgentSpec, ExecutionPhase, ExecutionPlan};
use crate::agents::AgentCapability;
use crate::config::Config;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAnalysis {
    pub task_description: String,
    pub complexity: u8, // 0-10 scale
    pub estimated_files: usize,
    pub estimated_tokens: usize,
    pub estimated_time_min: u32, // minutes
    pub estimated_time_max: u32,
    pub required_capabilities: Vec<AgentCapability>,
    pub keywords: Vec<String>,
}

pub struct TaskPlanner {
    config: Config,
}

impl TaskPlanner {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Analyze a task to understand its requirements
    pub async fn analyze_task(&self, task: &str) -> Result<TaskAnalysis> {
        // This is a simplified analysis. In production, you might:
        // 1. Use Claude API to analyze the task
        // 2. Use local ML model
        // 3. Use rule-based heuristics

        let task_lower = task.to_lowercase();

        // Detect complexity based on keywords
        let complexity = self.estimate_complexity(&task_lower);

        // Detect required capabilities
        let capabilities = self.detect_capabilities(&task_lower);

        // Extract keywords
        let keywords = self.extract_keywords(&task_lower);

        // Estimate resources
        let estimated_files = self.estimate_files(&task_lower, complexity);
        let estimated_tokens = self.estimate_tokens(complexity, estimated_files);
        let (time_min, time_max) = self.estimate_time(complexity);

        Ok(TaskAnalysis {
            task_description: task.to_string(),
            complexity,
            estimated_files,
            estimated_tokens,
            estimated_time_min: time_min,
            estimated_time_max: time_max,
            required_capabilities: capabilities,
            keywords,
        })
    }

    /// Create an execution plan based on task analysis
    pub async fn create_plan(
        &self,
        analysis: &TaskAnalysis,
        max_agents: usize,
    ) -> Result<ExecutionPlan> {
        let mut phases = Vec::new();

        // Determine agent team composition based on capabilities
        let agent_specs = self.plan_agents(analysis, max_agents);

        // Group agents into phases
        phases.extend(self.create_phases(analysis, agent_specs));

        Ok(ExecutionPlan { phases })
    }

    fn estimate_complexity(&self, task: &str) -> u8 {
        // TODO: Add input validation - reject empty strings or extremely long inputs (>10000 chars)
        // TODO: Add tests for edge cases: empty string, single char, Unicode, null bytes
        // TODO: Add tests for boundary values: should never exceed 10 or go below 0
        // TODO: Add tests for case insensitivity: "REFACTOR" should match "refactor"
        // TODO: Add tests for keyword repetition: "refactor refactor" should not double-count

        let mut complexity = 3; // Base complexity

        // Increase complexity based on keywords
        let high_complexity_keywords = [
            "refactor",
            "migrate",
            "redesign",
            "architecture",
            "authentication",
            "oauth",
            "security",
            "encryption",
            "performance",
            "optimize",
            "scale",
            "distributed",
        ];

        let medium_complexity_keywords = [
            "implement",
            "create",
            "build",
            "add feature",
            "integration",
            "api",
            "database",
            "tests",
        ];

        // TODO: Convert to lowercase for case-insensitive matching
        for keyword in &high_complexity_keywords {
            if task.contains(keyword) {
                complexity += 2;
            }
        }

        for keyword in &medium_complexity_keywords {
            if task.contains(keyword) {
                complexity += 1;
            }
        }

        // Check for multiple requirements
        if task.contains(" and ") || task.contains(" with ") {
            complexity += 1;
        }

        complexity.min(10)
    }

    fn detect_capabilities(&self, task: &str) -> Vec<AgentCapability> {
        // TODO: Add tests for multiple capability detection
        // TODO: Add tests for default CodeWriting fallback
        // TODO: Add tests for edge cases: empty input should return default
        // TODO: Add tests for case insensitive matching

        let mut capabilities = Vec::new();

        let capability_keywords = vec![
            (
                AgentCapability::CodeWriting,
                vec!["implement", "create", "write", "add", "build"],
            ),
            (
                AgentCapability::Testing,
                vec!["test", "testing", "coverage", "unit test"],
            ),
            (
                AgentCapability::Security,
                vec!["security", "auth", "oauth", "encryption", "vulnerability"],
            ),
            (
                AgentCapability::Documentation,
                vec!["document", "docs", "readme", "comments"],
            ),
            (
                AgentCapability::Debugging,
                vec!["debug", "fix", "bug", "error", "issue"],
            ),
            (
                AgentCapability::Performance,
                vec!["optimize", "performance", "speed", "efficiency"],
            ),
            (
                AgentCapability::Architecture,
                vec!["architecture", "design", "refactor", "structure"],
            ),
            (
                AgentCapability::Migration,
                vec!["migrate", "migration", "upgrade", "convert"],
            ),
        ];

        for (capability, keywords) in capability_keywords {
            for keyword in keywords {
                if task.contains(keyword) {
                    if !capabilities.contains(&capability) {
                        capabilities.push(capability);
                    }
                    break;
                }
            }
        }

        // If no specific capabilities detected, assume code writing
        if capabilities.is_empty() {
            capabilities.push(AgentCapability::CodeWriting);
        }

        capabilities
    }

    fn extract_keywords(&self, task: &str) -> Vec<String> {
        task.split_whitespace()
            .filter(|word| word.len() > 3)
            .take(10)
            .map(|s| s.to_string())
            .collect()
    }

    fn estimate_files(&self, task: &str, complexity: u8) -> usize {
        let base = match complexity {
            0..=3 => 1,
            4..=6 => 3,
            7..=8 => 8,
            _ => 12,
        };

        // Adjust based on task hints
        let multiplier = if task.contains("system") || task.contains("entire") {
            2.0
        } else if task.contains("single") || task.contains("one") {
            0.5
        } else {
            1.0
        };

        (base as f32 * multiplier) as usize
    }

    fn estimate_tokens(&self, complexity: u8, files: usize) -> usize {
        let base_per_file = 2000;
        let complexity_multiplier = 1.0 + (complexity as f32 * 0.2);

        ((files * base_per_file) as f32 * complexity_multiplier) as usize
    }

    fn estimate_time(&self, complexity: u8) -> (u32, u32) {
        match complexity {
            0..=3 => (2, 5),
            4..=6 => (5, 15),
            7..=8 => (15, 30),
            _ => (30, 60),
        }
    }

    fn plan_agents(&self, analysis: &TaskAnalysis, max_agents: usize) -> Vec<AgentSpec> {
        let mut specs = Vec::new();

        // Create agents based on required capabilities
        for capability in &analysis.required_capabilities {
            match capability {
                AgentCapability::Architecture => {
                    specs.push(AgentSpec {
                        id: format!("architect-{}", specs.len()),
                        agent_type: "Architect".to_string(),
                        capability: capability.clone(),
                        task: "Design system architecture and create implementation plan"
                            .to_string(),
                        dependencies: vec![],
                    });
                }

                AgentCapability::CodeWriting => {
                    // Dynamically create multiple code writers for complex tasks
                    let num_writers = if analysis.complexity >= 7 && analysis.estimated_files > 5 {
                        ((analysis.estimated_files / 3).min(max_agents - specs.len())).max(1)
                    } else {
                        1
                    };

                    for i in 0..num_writers {
                        let suffix = if num_writers > 1 {
                            format!(" (Part {})", i + 1)
                        } else {
                            String::new()
                        };

                        let agent_name = if i == 0 {
                            "Code Writer Alpha".to_string()
                        } else if i == 1 {
                            "Code Writer Beta".to_string()
                        } else if i == 2 {
                            "Code Writer Gamma".to_string()
                        } else {
                            format!("Code Writer Delta-{}", i - 2)
                        };

                        specs.push(AgentSpec {
                            id: format!("coder-{}", i),
                            agent_type: agent_name,
                            capability: capability.clone(),
                            task: format!("Implement code changes{}", suffix),
                            dependencies: specs
                                .iter()
                                .filter(|s| s.capability == AgentCapability::Architecture)
                                .map(|s| s.id.clone())
                                .collect(),
                        });
                    }
                }

                AgentCapability::Security => {
                    specs.push(AgentSpec {
                        id: format!("security-{}", specs.len()),
                        agent_type: "Security Auditor".to_string(),
                        capability: capability.clone(),
                        task: "Review code for security vulnerabilities".to_string(),
                        dependencies: specs
                            .iter()
                            .filter(|s| s.capability == AgentCapability::CodeWriting)
                            .map(|s| s.id.clone())
                            .collect(),
                    });
                }

                AgentCapability::Testing => {
                    specs.push(AgentSpec {
                        id: format!("tester-{}", specs.len()),
                        agent_type: "Test Engineer".to_string(),
                        capability: capability.clone(),
                        task: "Write comprehensive tests".to_string(),
                        dependencies: specs
                            .iter()
                            .filter(|s| s.capability == AgentCapability::CodeWriting)
                            .map(|s| s.id.clone())
                            .collect(),
                    });
                }

                AgentCapability::Documentation => {
                    specs.push(AgentSpec {
                        id: format!("docs-{}", specs.len()),
                        agent_type: "Documentation Writer".to_string(),
                        capability: capability.clone(),
                        task: "Create comprehensive documentation".to_string(),
                        dependencies: specs.iter().map(|s| s.id.clone()).collect(),
                    });
                }

                AgentCapability::Migration => {
                    specs.push(AgentSpec {
                        id: format!("migration-{}", specs.len()),
                        agent_type: "Migration Specialist".to_string(),
                        capability: capability.clone(),
                        task: "Plan and execute migration strategy".to_string(),
                        dependencies: vec![],
                    });
                }

                _ => {}
            }
        }

        specs
    }

    fn create_phases(
        &self,
        _analysis: &TaskAnalysis,
        specs: Vec<AgentSpec>,
    ) -> Vec<ExecutionPhase> {
        // Build dependency graph and create phases
        let mut phases = Vec::new();
        let mut remaining_specs = specs;
        let mut completed_ids: Vec<String> = Vec::new();

        // Security: Prevent infinite loops from circular dependencies
        let max_iterations = remaining_specs.len() * 2; // Reasonable upper bound
        let mut iteration_count = 0;

        while !remaining_specs.is_empty() {
            iteration_count += 1;

            // Detect potential infinite loop from circular dependencies
            if iteration_count > max_iterations {
                tracing::error!(
                    "Circular dependency detected! Remaining agents: {:?}",
                    remaining_specs.iter().map(|s| &s.id).collect::<Vec<_>>()
                );
                tracing::warn!(
                    "Breaking dependency cycle and executing remaining agents sequentially"
                );
                // Execute remaining specs sequentially as fallback
                for spec in remaining_specs {
                    phases.push(ExecutionPhase {
                        description: format!(
                            "Phase {} (dependency cycle recovery)",
                            phases.len() + 1
                        ),
                        agents: vec![spec],
                        parallel: false,
                    });
                }
                break;
            }

            // Find specs with no unmet dependencies
            let (ready, not_ready): (Vec<_>, Vec<_>) =
                remaining_specs.into_iter().partition(|spec| {
                    spec.dependencies
                        .iter()
                        .all(|dep_id| completed_ids.contains(dep_id))
                });

            if ready.is_empty() {
                // Circular dependency detected - log detailed warning
                let unmet_deps: Vec<String> = not_ready
                    .iter()
                    .flat_map(|spec| {
                        spec.dependencies
                            .iter()
                            .filter(|dep| !completed_ids.contains(dep))
                            .map(|d| format!("{} -> {}", spec.id, d))
                    })
                    .collect();

                tracing::error!(
                    "Circular dependency detected! Unmet dependencies: {:?}",
                    unmet_deps
                );
                tracing::warn!("Executing remaining agents in arbitrary order as fallback");

                // Add remaining as final phase with warning
                phases.push(ExecutionPhase {
                    description: format!(
                        "Phase {} (circular dependency fallback)",
                        phases.len() + 1
                    ),
                    agents: not_ready,
                    parallel: false,
                });
                break;
            }

            // Check if these can run in parallel (no dependencies on each other)
            let can_parallel = ready.len() > 1
                && ready.iter().all(|spec1| {
                    ready.iter().all(|spec2| {
                        spec1.id == spec2.id || !spec2.dependencies.contains(&spec1.id)
                    })
                });

            // Mark these as completed
            completed_ids.extend(ready.iter().map(|s| s.id.clone()));

            phases.push(ExecutionPhase {
                description: if can_parallel {
                    format!("Phase {} (parallel execution)", phases.len() + 1)
                } else {
                    format!("Phase {}", phases.len() + 1)
                },
                agents: ready,
                parallel: can_parallel,
            });

            remaining_specs = not_ready;
        }

        // TODO: Add comprehensive tests for create_phases():
        // - Empty agents vector
        // - Single agent with no dependencies
        // - Linear chain (A -> B -> C)
        // - Diamond dependency (A -> B,C -> D)
        // - Fully parallel agents (no dependencies)
        // - Circular dependency (A -> B -> A) - should use fallback
        // - Self-dependency (A -> A)
        // - Missing dependency (A depends on non-existent B)
        // - Large graph (100+ agents) - performance test
        // - Complex multi-path dependencies

        phases
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_planner() -> TaskPlanner {
        TaskPlanner::new(Config::default())
    }

    // ============================================================================
    // Complexity Calculation Tests (8 tests)
    // ============================================================================

    #[test]
    fn test_estimate_complexity_empty_string() {
        let planner = create_test_planner();
        let complexity = planner.estimate_complexity("");
        // Empty string should return base complexity
        assert_eq!(complexity, 3);
    }

    #[test]
    fn test_estimate_complexity_base_value() {
        let planner = create_test_planner();
        let complexity = planner.estimate_complexity("fix typo in README");
        // Simple task with no keywords should return base complexity
        assert_eq!(complexity, 3);
    }

    #[test]
    fn test_estimate_complexity_high_keyword_refactor() {
        let planner = create_test_planner();
        let complexity = planner.estimate_complexity("refactor the code");
        // Base (3) + high keyword (2) = 5
        assert_eq!(complexity, 5);
    }

    #[test]
    fn test_estimate_complexity_medium_keyword_implement() {
        let planner = create_test_planner();
        let complexity = planner.estimate_complexity("implement new feature");
        // Base (3) + medium keyword (1) = 4
        assert_eq!(complexity, 4);
    }

    #[test]
    fn test_estimate_complexity_multiple_requirements() {
        let planner = create_test_planner();
        let complexity = planner.estimate_complexity("implement authentication and add tests");
        // Base (3) + implement (1) + authentication (2) + tests (1) + "and" bonus (1) = 8
        assert_eq!(complexity, 8);
    }

    #[test]
    fn test_estimate_complexity_capped_at_10() {
        let planner = create_test_planner();
        let complexity = planner.estimate_complexity(
            "refactor migrate redesign architecture authentication oauth security encryption performance optimize scale distributed"
        );
        // Should have many high keywords but capped at 10
        assert_eq!(complexity, 10);
    }

    #[test]
    fn test_estimate_complexity_case_sensitivity() {
        let planner = create_test_planner();
        // Note: Current implementation is case-sensitive (TODO to fix)
        let lower = planner.estimate_complexity("refactor code");
        let upper = planner.estimate_complexity("REFACTOR code");

        // For now, uppercase won't match (this is a known issue)
        // When case-insensitive matching is implemented, this test should be updated
        assert_eq!(lower, 5); // Base + refactor
        assert_eq!(upper, 3); // Base only (doesn't match uppercase)
    }

    #[test]
    fn test_estimate_complexity_multiple_same_keyword() {
        let planner = create_test_planner();
        let complexity = planner.estimate_complexity("refactor refactor refactor");
        // Current implementation counts keyword match once per keyword check
        // Base (3) + refactor (2) = 5 (doesn't count repetitions)
        // TODO: Consider if we should count multiple occurrences
        assert_eq!(complexity, 5);
    }

    // ============================================================================
    // Capability Detection Tests (6 tests)
    // ============================================================================

    #[test]
    fn test_detect_capabilities_empty_string() {
        let planner = create_test_planner();
        let capabilities = planner.detect_capabilities("");
        // Empty string should default to CodeWriting
        assert!(capabilities.contains(&AgentCapability::CodeWriting));
    }

    #[test]
    fn test_detect_capabilities_single_keyword() {
        let planner = create_test_planner();
        let capabilities = planner.detect_capabilities("write tests for the feature");
        // Should detect Testing capability
        assert!(capabilities.contains(&AgentCapability::Testing));
    }

    #[test]
    fn test_detect_capabilities_multiple_keywords() {
        let planner = create_test_planner();
        let capabilities =
            planner.detect_capabilities("implement authentication with security audit and tests");
        // Should detect CodeWriting, Security, and Testing
        assert!(capabilities.contains(&AgentCapability::CodeWriting));
        assert!(capabilities.contains(&AgentCapability::Security));
        assert!(capabilities.contains(&AgentCapability::Testing));
        assert!(capabilities.len() >= 3);
    }

    #[test]
    fn test_detect_capabilities_default_code_writing() {
        let planner = create_test_planner();
        let capabilities = planner.detect_capabilities("some random task");
        // Should default to CodeWriting when no specific keywords match
        assert!(capabilities.contains(&AgentCapability::CodeWriting));
    }

    #[test]
    fn test_detect_capabilities_security_audit() {
        let planner = create_test_planner();
        let capabilities = planner.detect_capabilities("perform security audit");
        // Should detect Security capability
        assert!(capabilities.contains(&AgentCapability::Security));
    }

    #[test]
    fn test_detect_capabilities_documentation() {
        let planner = create_test_planner();
        let capabilities = planner.detect_capabilities("write documentation for the API");
        // Should detect Documentation capability
        assert!(capabilities.contains(&AgentCapability::Documentation));
    }

    // ============================================================================
    // Dependency Resolution Tests (10 tests) - Phase 2
    // ============================================================================

    #[test]
    fn test_create_phases_empty_agents() {
        let planner = create_test_planner();
        let analysis = TaskAnalysis {
            task_description: "test".to_string(),
            complexity: 3,
            estimated_files: 1,
            estimated_tokens: 1000,
            estimated_time_min: 5,
            estimated_time_max: 10,
            required_capabilities: vec![AgentCapability::CodeWriting],
            keywords: vec![],
        };

        let phases = planner.create_phases(&analysis, vec![]);

        // Empty input should produce empty phases
        assert_eq!(phases.len(), 0);
    }

    #[test]
    fn test_create_phases_single_agent() {
        let planner = create_test_planner();
        let analysis = TaskAnalysis {
            task_description: "test".to_string(),
            complexity: 3,
            estimated_files: 1,
            estimated_tokens: 1000,
            estimated_time_min: 5,
            estimated_time_max: 10,
            required_capabilities: vec![AgentCapability::CodeWriting],
            keywords: vec![],
        };

        let specs = vec![AgentSpec {
            id: "agent-1".to_string(),
            agent_type: "code".to_string(),
            capability: AgentCapability::CodeWriting,
            task: "write code".to_string(),
            dependencies: vec![],
        }];

        let phases = planner.create_phases(&analysis, specs);

        // Single agent with no dependencies should create one phase
        assert_eq!(phases.len(), 1);
        assert_eq!(phases[0].agents.len(), 1);
        assert_eq!(phases[0].agents[0].id, "agent-1");
    }

    #[test]
    fn test_create_phases_linear_chain() {
        let planner = create_test_planner();
        let analysis = TaskAnalysis {
            task_description: "test".to_string(),
            complexity: 5,
            estimated_files: 3,
            estimated_tokens: 3000,
            estimated_time_min: 10,
            estimated_time_max: 20,
            required_capabilities: vec![],
            keywords: vec![],
        };

        // A -> B -> C (linear dependency chain)
        let specs = vec![
            AgentSpec {
                id: "C".to_string(),
                agent_type: "code".to_string(),
                capability: AgentCapability::CodeWriting,
                task: "task C".to_string(),
                dependencies: vec!["B".to_string()],
            },
            AgentSpec {
                id: "A".to_string(),
                agent_type: "arch".to_string(),
                capability: AgentCapability::Architecture,
                task: "task A".to_string(),
                dependencies: vec![],
            },
            AgentSpec {
                id: "B".to_string(),
                agent_type: "test".to_string(),
                capability: AgentCapability::Testing,
                task: "task B".to_string(),
                dependencies: vec!["A".to_string()],
            },
        ];

        let phases = planner.create_phases(&analysis, specs);

        // Should create 3 phases: [A], [B], [C]
        assert_eq!(phases.len(), 3);
        assert_eq!(phases[0].agents.len(), 1);
        assert_eq!(phases[0].agents[0].id, "A");
        assert_eq!(phases[1].agents.len(), 1);
        assert_eq!(phases[1].agents[0].id, "B");
        assert_eq!(phases[2].agents.len(), 1);
        assert_eq!(phases[2].agents[0].id, "C");
    }

    #[test]
    fn test_create_phases_diamond_dependency() {
        let planner = create_test_planner();
        let analysis = TaskAnalysis {
            task_description: "test".to_string(),
            complexity: 7,
            estimated_files: 5,
            estimated_tokens: 5000,
            estimated_time_min: 15,
            estimated_time_max: 30,
            required_capabilities: vec![],
            keywords: vec![],
        };

        // Diamond: A -> B, A -> C, B -> D, C -> D
        let specs = vec![
            AgentSpec {
                id: "A".to_string(),
                agent_type: "arch".to_string(),
                capability: AgentCapability::Architecture,
                task: "design".to_string(),
                dependencies: vec![],
            },
            AgentSpec {
                id: "B".to_string(),
                agent_type: "code".to_string(),
                capability: AgentCapability::CodeWriting,
                task: "implement feature 1".to_string(),
                dependencies: vec!["A".to_string()],
            },
            AgentSpec {
                id: "C".to_string(),
                agent_type: "code".to_string(),
                capability: AgentCapability::CodeWriting,
                task: "implement feature 2".to_string(),
                dependencies: vec!["A".to_string()],
            },
            AgentSpec {
                id: "D".to_string(),
                agent_type: "test".to_string(),
                capability: AgentCapability::Testing,
                task: "test both features".to_string(),
                dependencies: vec!["B".to_string(), "C".to_string()],
            },
        ];

        let phases = planner.create_phases(&analysis, specs);

        // Should create 3 phases: [A], [B, C] (parallel), [D]
        assert_eq!(phases.len(), 3);
        assert_eq!(phases[0].agents.len(), 1);
        assert_eq!(phases[0].agents[0].id, "A");

        // Phase 2 should have B and C in parallel
        assert_eq!(phases[1].agents.len(), 2);
        assert!(phases[1].parallel); // B and C can run in parallel

        assert_eq!(phases[2].agents.len(), 1);
        assert_eq!(phases[2].agents[0].id, "D");
    }

    #[test]
    fn test_create_phases_fully_parallel() {
        let planner = create_test_planner();
        let analysis = TaskAnalysis {
            task_description: "test".to_string(),
            complexity: 5,
            estimated_files: 4,
            estimated_tokens: 4000,
            estimated_time_min: 10,
            estimated_time_max: 20,
            required_capabilities: vec![],
            keywords: vec![],
        };

        // Four independent agents with no dependencies
        let specs = vec![
            AgentSpec {
                id: "A".to_string(),
                agent_type: "code".to_string(),
                capability: AgentCapability::CodeWriting,
                task: "task A".to_string(),
                dependencies: vec![],
            },
            AgentSpec {
                id: "B".to_string(),
                agent_type: "test".to_string(),
                capability: AgentCapability::Testing,
                task: "task B".to_string(),
                dependencies: vec![],
            },
            AgentSpec {
                id: "C".to_string(),
                agent_type: "doc".to_string(),
                capability: AgentCapability::Documentation,
                task: "task C".to_string(),
                dependencies: vec![],
            },
            AgentSpec {
                id: "D".to_string(),
                agent_type: "security".to_string(),
                capability: AgentCapability::Security,
                task: "task D".to_string(),
                dependencies: vec![],
            },
        ];

        let phases = planner.create_phases(&analysis, specs);

        // All agents should be in a single parallel phase
        assert_eq!(phases.len(), 1);
        assert_eq!(phases[0].agents.len(), 4);
        assert!(phases[0].parallel); // All can run in parallel
    }

    #[test]
    fn test_create_phases_circular_dependency() {
        // This tests the security fix we implemented!
        let planner = create_test_planner();
        let analysis = TaskAnalysis {
            task_description: "test".to_string(),
            complexity: 5,
            estimated_files: 2,
            estimated_tokens: 2000,
            estimated_time_min: 10,
            estimated_time_max: 20,
            required_capabilities: vec![],
            keywords: vec![],
        };

        // Circular dependency: A depends on B, B depends on A
        let specs = vec![
            AgentSpec {
                id: "A".to_string(),
                agent_type: "code".to_string(),
                capability: AgentCapability::CodeWriting,
                task: "task A".to_string(),
                dependencies: vec!["B".to_string()],
            },
            AgentSpec {
                id: "B".to_string(),
                agent_type: "test".to_string(),
                capability: AgentCapability::Testing,
                task: "task B".to_string(),
                dependencies: vec!["A".to_string()],
            },
        ];

        let phases = planner.create_phases(&analysis, specs);

        // Should detect circular dependency and handle gracefully
        // The fallback creates a phase with all remaining agents
        assert!(!phases.is_empty());

        // All agents should still be included (no infinite loop!)
        let total_agents: usize = phases.iter().map(|p| p.agents.len()).sum();
        assert_eq!(total_agents, 2);
    }

    #[test]
    fn test_create_phases_self_dependency() {
        let planner = create_test_planner();
        let analysis = TaskAnalysis {
            task_description: "test".to_string(),
            complexity: 3,
            estimated_files: 1,
            estimated_tokens: 1000,
            estimated_time_min: 5,
            estimated_time_max: 10,
            required_capabilities: vec![],
            keywords: vec![],
        };

        // Agent that depends on itself
        let specs = vec![AgentSpec {
            id: "A".to_string(),
            agent_type: "code".to_string(),
            capability: AgentCapability::CodeWriting,
            task: "task A".to_string(),
            dependencies: vec!["A".to_string()],
        }];

        let phases = planner.create_phases(&analysis, specs);

        // Should handle self-dependency (treated as circular)
        assert!(!phases.is_empty());
        let total_agents: usize = phases.iter().map(|p| p.agents.len()).sum();
        assert_eq!(total_agents, 1);
    }

    #[test]
    fn test_create_phases_missing_dependency() {
        let planner = create_test_planner();
        let analysis = TaskAnalysis {
            task_description: "test".to_string(),
            complexity: 4,
            estimated_files: 2,
            estimated_tokens: 2000,
            estimated_time_min: 10,
            estimated_time_max: 15,
            required_capabilities: vec![],
            keywords: vec![],
        };

        // Agent A depends on non-existent agent "X"
        let specs = vec![AgentSpec {
            id: "A".to_string(),
            agent_type: "code".to_string(),
            capability: AgentCapability::CodeWriting,
            task: "task A".to_string(),
            dependencies: vec!["X".to_string()], // X doesn't exist
        }];

        let phases = planner.create_phases(&analysis, specs);

        // Should handle missing dependency gracefully (fallback)
        assert!(!phases.is_empty());
        let total_agents: usize = phases.iter().map(|p| p.agents.len()).sum();
        assert_eq!(total_agents, 1);
    }

    #[test]
    fn test_create_phases_parallel_flag_correct() {
        let planner = create_test_planner();
        let analysis = TaskAnalysis {
            task_description: "test".to_string(),
            complexity: 5,
            estimated_files: 3,
            estimated_tokens: 3000,
            estimated_time_min: 10,
            estimated_time_max: 20,
            required_capabilities: vec![],
            keywords: vec![],
        };

        // Sequential chain should have parallel=false
        let specs = vec![
            AgentSpec {
                id: "A".to_string(),
                agent_type: "arch".to_string(),
                capability: AgentCapability::Architecture,
                task: "design".to_string(),
                dependencies: vec![],
            },
            AgentSpec {
                id: "B".to_string(),
                agent_type: "code".to_string(),
                capability: AgentCapability::CodeWriting,
                task: "implement".to_string(),
                dependencies: vec!["A".to_string()],
            },
        ];

        let phases = planner.create_phases(&analysis, specs);

        assert_eq!(phases.len(), 2);
        // First phase with single agent should not be marked parallel
        assert!(!phases[0].parallel);
        // Second phase with single agent should not be marked parallel
        assert!(!phases[1].parallel);
    }

    #[test]
    fn test_create_phases_no_agent_lost() {
        let planner = create_test_planner();
        let analysis = TaskAnalysis {
            task_description: "test".to_string(),
            complexity: 6,
            estimated_files: 5,
            estimated_tokens: 5000,
            estimated_time_min: 15,
            estimated_time_max: 30,
            required_capabilities: vec![],
            keywords: vec![],
        };

        // Complex graph with multiple paths
        let specs = vec![
            AgentSpec {
                id: "A".to_string(),
                agent_type: "a".to_string(),
                capability: AgentCapability::Architecture,
                task: "a".to_string(),
                dependencies: vec![],
            },
            AgentSpec {
                id: "B".to_string(),
                agent_type: "b".to_string(),
                capability: AgentCapability::CodeWriting,
                task: "b".to_string(),
                dependencies: vec!["A".to_string()],
            },
            AgentSpec {
                id: "C".to_string(),
                agent_type: "c".to_string(),
                capability: AgentCapability::CodeWriting,
                task: "c".to_string(),
                dependencies: vec!["A".to_string()],
            },
            AgentSpec {
                id: "D".to_string(),
                agent_type: "d".to_string(),
                capability: AgentCapability::Testing,
                task: "d".to_string(),
                dependencies: vec!["B".to_string()],
            },
            AgentSpec {
                id: "E".to_string(),
                agent_type: "e".to_string(),
                capability: AgentCapability::Documentation,
                task: "e".to_string(),
                dependencies: vec!["C".to_string()],
            },
        ];

        let original_count = specs.len();
        let phases = planner.create_phases(&analysis, specs);

        // Verify all agents appear exactly once
        let total_agents: usize = phases.iter().map(|p| p.agents.len()).sum();
        assert_eq!(total_agents, original_count);

        // Verify no duplicates
        let mut all_ids: Vec<String> = phases
            .iter()
            .flat_map(|p| p.agents.iter().map(|a| a.id.clone()))
            .collect();
        all_ids.sort();
        all_ids.dedup();
        assert_eq!(all_ids.len(), original_count);
    }
}
