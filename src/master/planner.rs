use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::agents::AgentCapability;
use super::orchestrator::{ExecutionPlan, ExecutionPhase, AgentSpec};

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
    pub async fn create_plan(&self, analysis: &TaskAnalysis, max_agents: usize) -> Result<ExecutionPlan> {
        let mut phases = Vec::new();

        // Determine agent team composition based on capabilities
        let agent_specs = self.plan_agents(analysis, max_agents);

        // Group agents into phases
        phases.extend(self.create_phases(analysis, agent_specs));

        Ok(ExecutionPlan { phases })
    }

    fn estimate_complexity(&self, task: &str) -> u8 {
        let mut complexity = 3; // Base complexity

        // Increase complexity based on keywords
        let high_complexity_keywords = [
            "refactor", "migrate", "redesign", "architecture",
            "authentication", "oauth", "security", "encryption",
            "performance", "optimize", "scale", "distributed",
        ];

        let medium_complexity_keywords = [
            "implement", "create", "build", "add feature",
            "integration", "api", "database", "tests",
        ];

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
        let mut capabilities = Vec::new();

        let capability_keywords = vec![
            (AgentCapability::CodeWriting, vec!["implement", "create", "write", "add", "build"]),
            (AgentCapability::Testing, vec!["test", "testing", "coverage", "unit test"]),
            (AgentCapability::Security, vec!["security", "auth", "oauth", "encryption", "vulnerability"]),
            (AgentCapability::Documentation, vec!["document", "docs", "readme", "comments"]),
            (AgentCapability::Debugging, vec!["debug", "fix", "bug", "error", "issue"]),
            (AgentCapability::Performance, vec!["optimize", "performance", "speed", "efficiency"]),
            (AgentCapability::Architecture, vec!["architecture", "design", "refactor", "structure"]),
            (AgentCapability::Migration, vec!["migrate", "migration", "upgrade", "convert"]),
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
                        task: "Design system architecture and create implementation plan".to_string(),
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
                            dependencies: if specs.is_empty() { vec![] } else { vec![specs[0].id.clone()] },
                        });
                    }
                }

                AgentCapability::Security => {
                    specs.push(AgentSpec {
                        id: format!("security-{}", specs.len()),
                        agent_type: "Security Auditor".to_string(),
                        capability: capability.clone(),
                        task: "Review code for security vulnerabilities".to_string(),
                        dependencies: specs.iter()
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
                        dependencies: specs.iter()
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

    fn create_phases(&self, _analysis: &TaskAnalysis, specs: Vec<AgentSpec>) -> Vec<ExecutionPhase> {
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
                tracing::warn!("Breaking dependency cycle and executing remaining agents sequentially");
                // Execute remaining specs sequentially as fallback
                for spec in remaining_specs {
                    phases.push(ExecutionPhase {
                        description: format!("Phase {} (dependency cycle recovery)", phases.len() + 1),
                        agents: vec![spec],
                        parallel: false,
                    });
                }
                break;
            }

            // Find specs with no unmet dependencies
            let (ready, not_ready): (Vec<_>, Vec<_>) = remaining_specs
                .into_iter()
                .partition(|spec| {
                    spec.dependencies.iter().all(|dep_id| completed_ids.contains(dep_id))
                });

            if ready.is_empty() {
                // Circular dependency detected - log detailed warning
                let unmet_deps: Vec<String> = not_ready.iter()
                    .flat_map(|spec| {
                        spec.dependencies.iter()
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
                    description: format!("Phase {} (circular dependency fallback)", phases.len() + 1),
                    agents: not_ready,
                    parallel: false,
                });
                break;
            }

            // Check if these can run in parallel (no dependencies on each other)
            let can_parallel = ready.len() > 1 &&
                ready.iter().all(|spec1| {
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

        phases
    }
}
