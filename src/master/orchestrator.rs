use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use crate::config::Config;
use crate::agents::{Agent, AgentCapability};
use super::AutonomyMode;
use colored::Colorize;
use indicatif::{ProgressBar, MultiProgress, ProgressStyle};
use std::sync::Arc;
use tokio::sync::Semaphore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    pub id: String,
    pub agent_type: String,
    pub capability: AgentCapability,
    pub task: String,
    pub dependencies: Vec<String>, // IDs of agents that must complete first
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPhase {
    pub description: String,
    pub agents: Vec<AgentSpec>,
    pub parallel: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub phases: Vec<ExecutionPhase>,
}

impl ExecutionPlan {
    pub fn total_agents(&self) -> usize {
        self.phases.iter().map(|p| p.agents.len()).sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub agents_executed: usize,
    pub tokens_used: usize,
    pub execution_time_secs: f64,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub struct Orchestrator {
    config: Config,
    mode: AutonomyMode,
    max_parallel: usize,
}

impl Orchestrator {
    pub fn new(config: Config, mode: AutonomyMode) -> Self {
        Self {
            max_parallel: config.master_coder.max_parallel_agents,
            config,
            mode,
        }
    }

    pub fn set_max_parallel(&mut self, max: usize) {
        self.max_parallel = max;
    }

    /// Execute the plan with agents
    pub async fn execute_plan(
        &self,
        plan: &ExecutionPlan,
        mut agents: Vec<Box<dyn Agent>>,
    ) -> Result<ExecutionResult> {
        let start_time = Instant::now();
        let mut total_tokens = 0;
        let mut agents_executed = 0;
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        let multi_progress = MultiProgress::new();

        for (phase_num, phase) in plan.phases.iter().enumerate() {
            println!("\n{} Phase {}/{}: {}",
                     if phase.parallel { "⚡".bright_yellow() } else { "→".bright_cyan() },
                     phase_num + 1,
                     plan.phases.len(),
                     phase.description.bright_white().bold());

            // Get user approval if needed
            if self.needs_approval_for_phase(phase_num, plan.phases.len()) {
                if !self.get_phase_approval(phase)? {
                    warnings.push(format!("Phase {} skipped by user", phase_num + 1));
                    continue;
                }
            }

            // Execute agents in this phase
            let phase_result = if phase.parallel {
                self.execute_parallel(phase, &mut agents, &multi_progress).await?
            } else {
                self.execute_sequential(phase, &mut agents, &multi_progress).await?
            };

            total_tokens += phase_result.tokens_used;
            agents_executed += phase_result.agents_completed;
            errors.extend(phase_result.errors);
            warnings.extend(phase_result.warnings);

            if !phase_result.success && phase_result.critical {
                // Critical failure, stop execution
                errors.push(format!("Critical failure in phase {}, stopping execution", phase_num + 1));
                break;
            }
        }

        let execution_time = start_time.elapsed().as_secs_f64();
        let success = errors.is_empty();

        Ok(ExecutionResult {
            success,
            agents_executed,
            tokens_used: total_tokens,
            execution_time_secs: execution_time,
            errors,
            warnings,
        })
    }

    async fn execute_parallel(
        &self,
        phase: &ExecutionPhase,
        agents: &mut Vec<Box<dyn Agent>>,
        multi_progress: &MultiProgress,
    ) -> Result<PhaseResult> {
        let semaphore = Arc::new(Semaphore::new(self.max_parallel));
        let mut handles: Vec<tokio::task::JoinHandle<Result<(usize, Option<String>)>>> = Vec::new();
        let mut tokens_used = 0;
        let mut completed = 0;
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        for spec in &phase.agents {
            // Find matching agent
            let agent_idx = agents.iter().position(|a| a.id() == spec.id);

            if let Some(idx) = agent_idx {
                let mut agent = agents.remove(idx);
                let permit = semaphore.clone().acquire_owned().await?;

                let pb = multi_progress.add(ProgressBar::new(100));
                pb.set_style(
                    ProgressStyle::default_bar()
                        .template("  {spinner:.cyan} [{bar:40.cyan/blue}] {msg}")
                        .unwrap()
                        .progress_chars("=>-")
                );
                pb.set_message(format!("{}: Starting...", spec.agent_type));

                let spec_clone = spec.clone();

                let handle = tokio::spawn(async move {
                    pb.set_position(10);
                    pb.set_message(format!("{}: Executing...", spec_clone.agent_type));

                    let result = agent.execute(&spec_clone.task).await;

                    pb.set_position(90);

                    drop(permit);

                    match result {
                        Ok(agent_result) => {
                            pb.set_position(100);
                            pb.finish_with_message(format!("{}: ✓ Complete ({} tokens)",
                                                           spec_clone.agent_type,
                                                           agent_result.tokens_used));
                            Ok((agent_result.tokens_used, None))
                        }
                        Err(e) => {
                            pb.finish_with_message(format!("{}: ✗ Failed",
                                                           spec_clone.agent_type));
                            Ok((0, Some(format!("{} failed: {}", spec_clone.agent_type, e))))
                        }
                    }
                });

                handles.push(handle);
            } else {
                warnings.push(format!("Agent {} not found", spec.id));
            }
        }

        // Wait for all agents to complete
        for handle in handles {
            match handle.await? {
                Ok((tokens, error)) => {
                    tokens_used += tokens;
                    completed += 1;
                    if let Some(err) = error {
                        errors.push(err);
                    }
                }
                Err(e) => {
                    errors.push(format!("Agent execution error: {}", e));
                }
            }
        }

        Ok(PhaseResult {
            success: errors.is_empty(),
            critical: false,
            agents_completed: completed,
            tokens_used,
            errors,
            warnings,
        })
    }

    async fn execute_sequential(
        &self,
        phase: &ExecutionPhase,
        agents: &mut Vec<Box<dyn Agent>>,
        multi_progress: &MultiProgress,
    ) -> Result<PhaseResult> {
        let mut tokens_used = 0;
        let mut completed = 0;
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        for spec in &phase.agents {
            // Find matching agent
            let agent_idx = agents.iter().position(|a| a.id() == spec.id);

            if let Some(idx) = agent_idx {
                let mut agent = agents.remove(idx);

                let pb = multi_progress.add(ProgressBar::new(100));
                pb.set_style(
                    ProgressStyle::default_bar()
                        .template("  {spinner:.cyan} [{bar:40.cyan/blue}] {msg}")
                        .unwrap()
                        .progress_chars("=>-")
                );
                pb.set_message(format!("{}: Starting...", spec.agent_type));

                pb.set_position(10);
                pb.set_message(format!("{}: Executing...", spec.agent_type));

                match agent.execute(&spec.task).await {
                    Ok(result) => {
                        tokens_used += result.tokens_used;
                        completed += 1;
                        pb.set_position(100);
                        pb.finish_with_message(format!("{}: ✓ Complete ({} tokens)",
                                                       spec.agent_type,
                                                       result.tokens_used));
                    }
                    Err(e) => {
                        pb.finish_with_message(format!("{}: ✗ Failed", spec.agent_type));
                        errors.push(format!("{} failed: {}", spec.agent_type, e));

                        // In sequential mode, a failure might be critical
                        if spec.capability == AgentCapability::Architecture {
                            return Ok(PhaseResult {
                                success: false,
                                critical: true,
                                agents_completed: completed,
                                tokens_used,
                                errors,
                                warnings,
                            });
                        }
                    }
                }
            } else {
                warnings.push(format!("Agent {} not found", spec.id));
            }
        }

        Ok(PhaseResult {
            success: errors.is_empty(),
            critical: false,
            agents_completed: completed,
            tokens_used,
            errors,
            warnings,
        })
    }

    fn needs_approval_for_phase(&self, phase_num: usize, total_phases: usize) -> bool {
        match self.mode {
            AutonomyMode::Trust => false,
            AutonomyMode::Interactive => true,
            AutonomyMode::Balanced => {
                // Ask at the beginning and before final phase
                phase_num == 0 || phase_num == total_phases - 1
            }
            AutonomyMode::Conservative => true,
        }
    }

    fn get_phase_approval(&self, phase: &ExecutionPhase) -> Result<bool> {
        use dialoguer::Confirm;

        let prompt = format!("Execute phase: {}?", phase.description);

        let result = Confirm::new()
            .with_prompt(prompt)
            .default(true)
            .interact()?;

        Ok(result)
    }
}

struct PhaseResult {
    success: bool,
    critical: bool, // If true, should stop execution
    agents_completed: usize,
    tokens_used: usize,
    errors: Vec<String>,
    warnings: Vec<String>,
}
