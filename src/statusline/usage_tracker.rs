use crate::config::Config;
use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Usage {
    pub five_hour_used: usize,
    pub five_hour_limit: usize,
    pub five_hour_percent: u8,
    pub five_hour_minutes_remaining: u32,

    pub seven_day_used: usize,
    pub seven_day_limit: usize,
    pub seven_day_percent: u8,

    pub burn_rate_per_hour: f64,
    pub estimated_seven_day_cost: f64,
}

#[derive(Debug, Deserialize)]
struct ClaudeUsageResponse {
    // This structure would match the actual Claude API response
    // For now, using placeholder structure
    usage: UsageData,
}

#[derive(Debug, Deserialize)]
struct UsageData {
    five_hour: BlockUsage,
    seven_day: BlockUsage,
}

#[derive(Debug, Deserialize)]
struct BlockUsage {
    used: usize,
    limit: usize,
    reset_at: Option<String>,
}

pub struct UsageTracker {
    config: Config,
    client: Client,
}

impl UsageTracker {
    pub async fn new(config: Config) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { config, client })
    }

    pub async fn get_usage(&self) -> Result<Usage> {
        // Try to fetch from Claude API
        match self.fetch_from_api().await {
            Ok(usage) => Ok(usage),
            Err(_) => {
                // Fallback to mock data for testing
                Ok(self.mock_usage())
            }
        }
    }

    async fn fetch_from_api(&self) -> Result<Usage> {
        let token = self.config.auth.get_token().await?;

        // Note: This endpoint might not be the correct one
        // You'd need to find the actual Claude usage API endpoint
        let response = self
            .client
            .get(format!("{}/usage", self.config.statusline.api_endpoint))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to fetch usage data")?;

        if !response.status().is_success() {
            anyhow::bail!("API returned error: {}", response.status());
        }

        let usage_response: ClaudeUsageResponse = response
            .json()
            .await
            .context("Failed to parse usage response")?;

        Ok(self.convert_response(usage_response))
    }

    // TODO: Add tests for convert_response():
    // - Test percentage calculation (0%, 50%, 100%)
    // - Test division by zero when limit is 0
    // - Test burn rate calculation accuracy
    // - Test cost calculation with known values
    // - Test with zero usage
    // - Test with usage exceeding limit (should it cap at 100%?)
    // - Test percentage overflow (255+)
    // - Test floating point precision issues
    fn convert_response(&self, response: ClaudeUsageResponse) -> Usage {
        // Calculate percentages with division by zero protection and clamping to 0-100 range
        let five_hour_percent = if response.usage.five_hour.limit == 0 {
            0
        } else {
            let percent = (response.usage.five_hour.used as f64
                / response.usage.five_hour.limit as f64)
                * 100.0;
            percent.clamp(0.0, 100.0) as u8
        };

        let seven_day_percent = if response.usage.seven_day.limit == 0 {
            0
        } else {
            let percent = (response.usage.seven_day.used as f64
                / response.usage.seven_day.limit as f64)
                * 100.0;
            percent.clamp(0.0, 100.0) as u8
        };

        // Calculate burn rate (tokens per hour)
        let burn_rate_tokens = response.usage.five_hour.used as f64 / 5.0;

        // Estimate cost (this would use actual pricing from LiteLLM or similar)
        // Using rough estimate: $3 per million input tokens, $15 per million output tokens
        // Assuming 50/50 split for simplicity
        // TODO: Use actual input/output token split for accurate cost calculation
        let avg_cost_per_million = 9.0; // Average of input and output
        let burn_rate_cost = (burn_rate_tokens / 1_000_000.0) * avg_cost_per_million;

        let estimated_seven_day_cost =
            (response.usage.seven_day.used as f64 / 1_000_000.0) * avg_cost_per_million;

        Usage {
            five_hour_used: response.usage.five_hour.used,
            five_hour_limit: response.usage.five_hour.limit,
            five_hour_percent,
            five_hour_minutes_remaining: 60, // Would calculate from reset_at

            seven_day_used: response.usage.seven_day.used,
            seven_day_limit: response.usage.seven_day.limit,
            seven_day_percent,

            burn_rate_per_hour: burn_rate_cost,
            estimated_seven_day_cost,
        }
    }

    fn mock_usage(&self) -> Usage {
        // Mock data for testing when API is not available
        Usage {
            five_hour_used: 14_000,
            five_hour_limit: 20_000,
            five_hour_percent: 70,
            five_hour_minutes_remaining: 120,

            seven_day_used: 130_000,
            seven_day_limit: 200_000,
            seven_day_percent: 65,

            burn_rate_per_hour: 0.15,
            estimated_seven_day_cost: 1.17,
        }
    }
}
