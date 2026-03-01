use anyhow::{Context, Result};
use api::apis::server_api;

use crate::client::TeamCityClient;
use crate::output;

pub struct ServerInfoArgs {}

pub async fn handle_server_command(
    client: &TeamCityClient,
    command: crate::cli::ServerCommands,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    match command {
        crate::cli::ServerCommands::Info => {
            handle_server_info(client, ServerInfoArgs {}, output_format).await
        }
    }
}

async fn handle_server_info(
    client: &TeamCityClient,
    _args: ServerInfoArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let server = server_api::get_server_info(&client.config, None)
        .await
        .context("Failed to get server info")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("{}", output::format_server_info(&server));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&server)?);
        }
    }

    Ok(())
}
