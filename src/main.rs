use std::{
    error::Error,
    time::Duration, 
    thread::sleep,
    io::Write};
use anyhow::Result;
use colored::Colorize;
use github_api::{
        github::*, 
        env::*};
use tracing::{info, error};
use tracing_subscriber::{
    layer::SubscriberExt, 
    util::SubscriberInitExt};


#[tokio::main]
async fn main() -> Result<()> {

    let config = Config::from_env()?;

    // Initialize the tracing subscriber
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            config.log_level.to_string()
        ))
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .init();
        
    // Clear the terminal
    std::process::Command::new("clear").status().unwrap();println!("\n");

    info!("Config: Repo Owner {}", config.repo_owner);
    info!("Config: Repo Name {}", config.repo_name);
    info!("Config: Log Level {}", config.log_level);
    
    let github_client = 
        GitHubClientBuilder::new(config.github_token()).await?.client();
        
    info!("GitHub Client is ready!");
    
    for _ in 0..10 {
        print!(".");
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_millis(100));
    }
    println!("\n");
    
    let commits = 
        github_client.repos( config.repo_owner, config.repo_name).list_commits().send().await;
    
    std::process::Command::new("clear").status().unwrap();println!("\n");
    
    match commits {
        Err(e) => {
            // Print the error message
            error!("Error: {}", e);
            
            // Print the source of the error
            if let Some(source) = e.source() {
                error!("Caused by: {}", source);
            }
        },
        Ok(commits) => {
            println!("Commits received");
            for commit in commits {
                println!("{}", commit.sha.green());
            }
        },
    }

    Ok(())
}
