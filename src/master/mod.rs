mod agent_factory;
pub mod orchestrator;
pub mod planner;

use crate::agents::AgentCapability;
use crate::config::Config;
use crate::db::Database;
use agent_factory::AgentFactory;
use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use orchestrator::Orchestrator;
use planner::TaskPlanner;

pub use orchestrator::{ExecutionPhase, ExecutionPlan};
pub use planner::TaskAnalysis;

/// Autonomy mode for Master Coder
#[derive(Debug, Clone, PartialEq)]
pub enum AutonomyMode {
    /// Ask for approval frequently
    Conservative,
    /// Approval gates at important points (default)
    Balanced,
    /// Fully automatic execution
    Trust,
    /// Full control over every step
    Interactive,
}

impl AutonomyMode {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "conservative" => Ok(Self::Conservative),
            "balanced" => Ok(Self::Balanced),
            "trust" => Ok(Self::Trust),
            "interactive" => Ok(Self::Interactive),
            _ => anyhow::bail!("Invalid autonomy mode: {}", s),
        }
    }
}

/// Master Coder - orchestrates specialized agents to complete complex tasks
pub struct MasterCoder {
    config: Config,
    mode: AutonomyMode,
    planner: TaskPlanner,
    factory: AgentFactory,
    orchestrator: Orchestrator,
    db: Database,
    max_agents: usize,
    token_budget: usize,
}

impl MasterCoder {
    /// Create a new Master Coder instance
    pub async fn new(config: Config, mode: String) -> Result<Self> {
        let autonomy_mode = AutonomyMode::from_str(&mode)?;
        let db = Database::new(&config).await?;

        Ok(Self {
            planner: TaskPlanner::new(config.clone()),
            factory: AgentFactory::new(config.clone()),
            orchestrator: Orchestrator::new(config.clone(), autonomy_mode.clone()),
            max_agents: config.master_coder.max_parallel_agents,
            token_budget: config.master_coder.token_budget,
            config,
            mode: autonomy_mode,
            db,
        })
    }

    /// Set maximum number of parallel agents
    pub fn set_max_agents(&mut self, max: usize) {
        self.max_agents = max;
        self.orchestrator.set_max_parallel(max);
    }

    /// Set token budget for this task
    pub fn set_token_budget(&mut self, budget: usize) {
        self.token_budget = budget;
    }

    /// Execute a task with agent orchestration
    pub async fn execute(&mut self, task: &str) -> Result<()> {
        println!(
            "\n{}",
            "═══════════════════════════════════════════════════════".bright_cyan()
        );
        println!(
            "{} {}",
            "Master Coder".bright_cyan().bold(),
            "Analyzing task...".white()
        );
        println!(
            "{}",
            "═══════════════════════════════════════════════════════".bright_cyan()
        );

        // Step 1: Analyze the task
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        spinner.set_message("Analyzing task complexity and requirements...");

        let analysis = self.planner.analyze_task(task).await?;
        spinner.finish_with_message(format!("✓ Analysis complete"));

        self.print_analysis(&analysis);

        // Step 2: Create execution plan
        println!(
            "\n{}",
            "Creating agent team and execution plan...".bright_yellow()
        );

        let plan = self.planner.create_plan(&analysis, self.max_agents).await?;

        self.print_plan(&plan)?;

        // Step 3: Get user approval (if needed based on mode)
        if !self.should_auto_approve() {
            if !self.get_user_approval("Proceed with this plan?")? {
                println!("Task cancelled by user.");
                return Ok(());
            }
        }

        // Step 4: Create agents based on plan
        let agents = self.factory.create_agents(&plan).await?;

        println!("\n{} {} agents created", "✓".green(), agents.len());

        // Step 5: Execute the plan
        println!(
            "\n{}",
            "═══════════════════════════════════════════════════════".bright_cyan()
        );
        println!("{}", "Executing plan...".bright_cyan().bold());
        println!(
            "{}",
            "═══════════════════════════════════════════════════════".bright_cyan()
        );

        let result = self.orchestrator.execute_plan(&plan, agents).await?;

        // Step 6: Save to database for learning
        self.save_execution(&task, &analysis, &plan, &result)
            .await?;

        // Step 7: Show results
        self.print_results(&result)?;

        Ok(())
    }

    fn print_analysis(&self, analysis: &TaskAnalysis) {
        println!("\n{}", "Task Analysis:".bright_yellow().bold());
        println!(
            "  {} {}",
            "Complexity:".white(),
            self.format_complexity(analysis.complexity)
        );
        println!(
            "  {} {}",
            "Estimated files:".white(),
            analysis.estimated_files
        );
        println!(
            "  {} {:?}",
            "Required expertise:".white(),
            analysis.required_capabilities
        );
        println!(
            "  {} ~{}",
            "Estimated tokens:".white(),
            analysis.estimated_tokens
        );
        println!(
            "  {} {}-{} minutes",
            "Estimated time:".white(),
            analysis.estimated_time_min,
            analysis.estimated_time_max
        );
    }

    fn format_complexity(&self, complexity: u8) -> String {
        let (label, color) = match complexity {
            0..=3 => ("Low", "green"),
            4..=6 => ("Medium", "yellow"),
            7..=8 => ("High", "red"),
            _ => ("Very High", "bright_red"),
        };

        match color {
            "green" => label.green().to_string(),
            "yellow" => label.yellow().to_string(),
            "red" => label.red().to_string(),
            "bright_red" => label.bright_red().bold().to_string(),
            _ => label.to_string(),
        }
    }

    fn print_plan(&self, plan: &ExecutionPlan) -> Result<()> {
        println!("\n{}", "Execution Plan:".bright_yellow().bold());
        println!("  {} {} phases", "Total phases:".white(), plan.phases.len());
        println!(
            "  {} {} agents",
            "Total agents:".white(),
            plan.total_agents()
        );
        println!("  {} {}", "Token budget:".white(), self.token_budget);

        for (i, phase) in plan.phases.iter().enumerate() {
            println!(
                "\n  {} Phase {}/{}: {}",
                if phase.parallel { "⚡" } else { "→" },
                i + 1,
                plan.phases.len(),
                phase.description
            );

            for agent_spec in &phase.agents {
                println!(
                    "    {} {} - {}",
                    "•".bright_cyan(),
                    agent_spec.agent_type,
                    agent_spec.task
                );
            }
        }

        Ok(())
    }

    fn should_auto_approve(&self) -> bool {
        matches!(self.mode, AutonomyMode::Trust)
    }

    fn get_user_approval(&self, prompt: &str) -> Result<bool> {
        use dialoguer::Confirm;

        let result = Confirm::new()
            .with_prompt(prompt)
            .default(true)
            .interact()?;

        Ok(result)
    }

    async fn save_execution(
        &self,
        task: &str,
        analysis: &TaskAnalysis,
        plan: &ExecutionPlan,
        result: &orchestrator::ExecutionResult,
    ) -> Result<()> {
        if self.config.master_coder.enable_learning {
            self.db
                .save_task_execution(task, analysis, plan, result)
                .await?;
        }
        Ok(())
    }

    fn print_results(&self, result: &orchestrator::ExecutionResult) -> Result<()> {
        println!(
            "\n{}",
            "═══════════════════════════════════════════════════════".bright_green()
        );
        println!("{}", "Execution Complete!".bright_green().bold());
        println!(
            "{}",
            "═══════════════════════════════════════════════════════".bright_green()
        );

        println!("\n{}", "Summary:".bright_yellow().bold());
        println!(
            "  {} {}",
            "Status:".white(),
            if result.success {
                "Success ✓".green()
            } else {
                "Failed ✗".red()
            }
        );
        println!(
            "  {} {}",
            "Agents executed:".white(),
            result.agents_executed
        );
        println!("  {} {}", "Total tokens used:".white(), result.tokens_used);
        println!(
            "  {} {:.2}s",
            "Total time:".white(),
            result.execution_time_secs
        );

        if !result.errors.is_empty() {
            println!("\n{}", "Errors:".red().bold());
            for error in &result.errors {
                println!("  {} {}", "✗".red(), error);
            }
        }

        if !result.warnings.is_empty() {
            println!("\n{}", "Warnings:".yellow().bold());
            for warning in &result.warnings {
                println!("  {} {}", "⚠".yellow(), warning);
            }
        }

        Ok(())
    }
}
