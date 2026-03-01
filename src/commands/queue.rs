use anyhow::{Context, Result};
use api::apis::build_queue_api;
use api::models::{Build, BuildCancelRequest};

use crate::client::TeamCityClient;
use crate::output;

pub struct QueueListArgs {
    pub build_type: Option<String>,
    pub limit: u32,
}

pub struct QueueGetArgs {
    pub build_id: String,
}

pub struct QueueCancelArgs {
    pub build_id: String,
    pub comment: Option<String>,
}

pub struct QueueReorderArgs {
    pub build_id: String,
    pub position: Option<i32>,
    pub top: bool,
}

pub async fn handle_queue_command(
    client: &TeamCityClient,
    command: crate::cli::QueueCommands,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    match command {
        crate::cli::QueueCommands::List { build_type, limit } => {
            handle_queue_list(client, QueueListArgs { build_type, limit }, output_format).await
        }
        crate::cli::QueueCommands::Get { build_id } => {
            handle_queue_get(client, QueueGetArgs { build_id }, output_format).await
        }
        crate::cli::QueueCommands::Cancel { build_id, comment } => {
            handle_queue_cancel(client, QueueCancelArgs { build_id, comment }, output_format).await
        }
        crate::cli::QueueCommands::Reorder {
            build_id,
            position,
            top,
        } => {
            handle_queue_reorder(
                client,
                QueueReorderArgs {
                    build_id,
                    position,
                    top,
                },
                output_format,
            )
            .await
        }
    }
}

async fn handle_queue_list(
    client: &TeamCityClient,
    args: QueueListArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let mut locator_parts = Vec::new();

    if let Some(bt) = &args.build_type {
        locator_parts.push(format!("buildType:({})", bt));
    }

    locator_parts.push(format!("count:{}", args.limit));

    let locator = if locator_parts.is_empty() {
        None
    } else {
        Some(locator_parts.join(","))
    };

    let builds = build_queue_api::get_all_queued_builds(&client.config, locator.as_deref(), None)
        .await
        .context("Failed to fetch queued builds")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("{}", output::format_queue_table(&builds));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&builds)?);
        }
    }

    Ok(())
}

async fn handle_queue_get(
    client: &TeamCityClient,
    args: QueueGetArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let build = build_queue_api::get_queued_build(&client.config, &args.build_id, None)
        .await
        .context("Failed to fetch queued build")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("{}", output::format_queued_build_details(&build));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&build)?);
        }
    }

    Ok(())
}

async fn handle_queue_cancel(
    client: &TeamCityClient,
    args: QueueCancelArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let mut cancel_request = BuildCancelRequest::default();

    if let Some(comment) = &args.comment {
        cancel_request.comment = Some(comment.clone());
    }

    let build =
        build_queue_api::cancel_queued_build(&client.config, &args.build_id, Some(cancel_request))
            .await
            .context("Failed to cancel queued build")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("Queued build canceled successfully!");
            println!(
                "Build ID: {}",
                build.id.map_or("N/A".to_string(), |id| id.to_string())
            );
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&build)?);
        }
    }

    Ok(())
}

async fn handle_queue_reorder(
    client: &TeamCityClient,
    args: QueueReorderArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let build = if args.top {
        let mut build = Build::default();
        build.id = Some(args.build_id.parse().context("Invalid build ID")?);

        build_queue_api::add_build_to_queue(&client.config, Some(true), Some(build))
            .await
            .context("Failed to move build to top of queue")?
    } else if let Some(pos) = args.position {
        let mut build = Build::default();
        build.id = Some(args.build_id.parse().context("Invalid build ID")?);

        build_queue_api::set_queued_build_position(
            &client.config,
            &pos.to_string(),
            None,
            Some(build),
        )
        .await
        .context("Failed to reorder build in queue")?
    } else {
        anyhow::bail!("Either --position or --top must be specified");
    };

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("Build reordered in queue successfully!");
            println!(
                "Build ID: {}",
                build.id.map_or("N/A".to_string(), |id| id.to_string())
            );
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&build)?);
        }
    }

    Ok(())
}
