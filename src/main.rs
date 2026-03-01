mod cli;
mod client;
mod commands;
mod config;
mod interactive;
mod output;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use cli::Commands;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Cli::parse();

    let config = config::Config::load()?;
    let (server_url, token) = config.resolve(
        args.server.as_deref(),
        args.token.as_deref(),
        args.profile.as_deref(),
    );

    let client = client::TeamCityClient::new(&server_url, &token)?;

    match args.command {
        Commands::Build { command } => {
            commands::handle_build_command(&client, command, args.output).await?;
        }
        Commands::Project { command } => {
            commands::handle_project_command(&client, command, args.output).await?;
        }
        Commands::Buildtype { command } => {
            commands::handle_buildtype_command(&client, command, args.output).await?;
        }
        Commands::Server { command } => {
            commands::handle_server_command(&client, command, args.output).await?;
        }
        Commands::Queue { command } => {
            commands::handle_queue_command(&client, command, args.output).await?;
        }
        Commands::Agent { command } => {
            commands::handle_agent_command(&client, command, args.output).await?;
        }
        Commands::Interactive => {
            interactive::run_interactive_mode(&client).await?;
        }
        Commands::Completion { shell } => {
            generate(
                shell,
                &mut cli::Cli::command(),
                "citizen",
                &mut std::io::stdout(),
            );
        }
    }

    Ok(())
}
