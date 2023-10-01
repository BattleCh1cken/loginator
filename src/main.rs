mod bluetooth;
mod config;
mod context;
mod decoder;
mod dispatch;
mod tests;

use context::Context;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List the brains that can be connected to
    ListBrains {},
    /// Display the connection code on a specific brain
    DisplayCode { name: Option<String> },
    /// Connect to a specific name
    Connect {
        name: Option<String>,
        code: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let context = Context::new().await?;

    let mut controller = bluetooth::BrainController::new().await;

    println!("Searching for brains...");
    let brain_info = controller.search().await.unwrap();

    match &cli.command {
        Commands::ListBrains {} => {
            for info in brain_info {
                println!("{info}");
            }
        }
        Commands::DisplayCode { name } => {
            let name: String = match name {
                Some(name) => name.to_string(),
                None => context.config.brain_name.clone(),
            };

            controller.connect(name).await?;
            controller.ping_brain_for_code().await?;
        }
        Commands::Connect { name, code } => {
            let name: String = match name {
                Some(name) => name.to_string(),
                None => context.config.brain_name.clone(),
            };

            let code: String = match code {
                Some(code) => code.to_string(),
                None => context.config.brain_code.clone(),
            };

            println!("Connecting to {}.", name);
            controller.connect(name).await?;
            controller.authenticate(&code).await?;
            controller.poll(dispatch::dispatch, context).await?;
        }
    }

    Ok(())
}
