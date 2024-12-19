use anyhow::*;
use colored::Colorize;
use std::env::var;

/// A Config represents the configuration of the GitHub API client.
///
/// The Config should be generated from the user's environment, and should
/// contain the following fields:
///
/// * `github_token`: A valid GitHub API token.
///
pub struct Config {
    github_token: String,
    pub repo_owner: String,
    pub repo_name: String,
    pub log_level: String,
}

impl Config {
    /// Loads configuration from environment variables
    ///
    /// # Returns
    /// A Result containing the Config if successful, or an error if any required
    /// environment variables are missing
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        let config = Config {
            github_token: var("GITHUB_TOKEN")?,
            repo_owner: var("REPO_OWNER").context("REPO_OWNER must be set")?,
            repo_name: var("REPO_NAME").context("REPO_NAME must be set")?,
            log_level: var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        };
        
        config.validate().context(format!("{}","\nConfig is invalid".red().bold()))?;
        
        Ok(config)
    }
    
    fn validate(&self) -> anyhow::Result<()> {

        match self.github_token.is_empty() {
            true => Err(anyhow!("GITHUB_TOKEN must be set")),
            false => Ok(()),
        }?;

        match self.repo_owner.is_empty() {
            true => Err(anyhow!("REPO_OWNER must be set")),
            false => Ok(()),
        }?;

        match self.repo_name.is_empty() {
            true => Err(anyhow!("REPO_NAME must be set")),
            false => Ok(()),
        }?;

        match self.log_level.is_empty() {
            true => Err(anyhow!("LOG_LEVEL must be set")),
            false => Ok(()),
        }?;

        Ok(())
    }

    pub fn github_token(&self) -> String {
        self.github_token.clone()
    }

}