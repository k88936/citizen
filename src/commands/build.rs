use anyhow::{Context, Result};
use api::apis::build_api;
use api::apis::build_queue_api;
use api::models::{Build, BuildCancelRequest, PinInfo, Properties, Property, Tag, Tags};

use crate::client::TeamCityClient;
use crate::output;

pub struct BuildListArgs {
    pub build_type: Option<String>,
    pub status: Option<String>,
    pub branch: Option<String>,
    pub limit: u32,
    pub running: bool,
    pub queued: bool,
}

pub struct BuildGetArgs {
    pub build_id: String,
}

pub struct BuildTriggerArgs {
    pub build_type: String,
    pub branch: Option<String>,
    pub parameter: Vec<String>,
    pub wait: bool,
    pub watch: bool,
}

pub struct BuildCancelArgs {
    pub build_id: String,
    pub comment: Option<String>,
}

pub struct BuildStatusArgs {
    pub build_id: String,
}

pub struct BuildLogArgs {
    pub build_id: String,
    pub follow: bool,
    pub tail: u32,
    pub download: bool,
}

pub struct BuildArtifactsArgs {
    pub build_id: String,
    pub command: crate::cli::ArtifactCommands,
}

pub struct BuildTagsArgs {
    pub build_id: String,
    pub command: crate::cli::TagCommands,
}

pub struct BuildPinArgs {
    pub build_id: String,
    pub unpin: bool,
    pub comment: Option<String>,
}

pub async fn handle_build_command(
    client: &TeamCityClient,
    command: crate::cli::BuildCommands,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    match command {
        crate::cli::BuildCommands::List {
            build_type,
            status,
            branch,
            limit,
            running,
            queued,
        } => {
            handle_build_list(
                client,
                BuildListArgs {
                    build_type,
                    status,
                    branch,
                    limit,
                    running,
                    queued,
                },
                output_format,
            )
            .await
        }
        crate::cli::BuildCommands::Get { build_id } => {
            handle_build_get(client, BuildGetArgs { build_id }, output_format).await
        }
        crate::cli::BuildCommands::Trigger {
            build_type,
            branch,
            comment: _,
            parameter,
            wait,
            watch,
        } => {
            handle_build_trigger(
                client,
                BuildTriggerArgs {
                    build_type,
                    branch,
                    parameter,
                    wait,
                    watch,
                },
                output_format,
            )
            .await
        }
        crate::cli::BuildCommands::Cancel { build_id, comment } => {
            handle_build_cancel(client, BuildCancelArgs { build_id, comment }, output_format).await
        }
        crate::cli::BuildCommands::Status { build_id } => {
            handle_build_status(client, BuildStatusArgs { build_id }, output_format).await
        }
        crate::cli::BuildCommands::Log {
            build_id,
            follow,
            tail,
            download,
        } => {
            handle_build_log(
                client,
                BuildLogArgs {
                    build_id,
                    follow,
                    tail,
                    download,
                },
            )
            .await
        }
        crate::cli::BuildCommands::Artifacts { build_id, command } => {
            handle_build_artifacts(
                client,
                BuildArtifactsArgs { build_id, command },
                output_format,
            )
            .await
        }
        crate::cli::BuildCommands::Tags { build_id, command } => {
            handle_build_tags(client, BuildTagsArgs { build_id, command }, output_format).await
        }
        crate::cli::BuildCommands::Pin {
            build_id,
            unpin,
            comment,
        } => {
            handle_build_pin(
                client,
                BuildPinArgs {
                    build_id,
                    unpin,
                    comment,
                },
                output_format,
            )
            .await
        }
    }
}

async fn handle_build_list(
    client: &TeamCityClient,
    args: BuildListArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let mut locator_parts = Vec::new();

    if let Some(bt) = &args.build_type {
        locator_parts.push(format!("buildType:({})", bt));
    }

    if let Some(status) = &args.status {
        locator_parts.push(format!("status:{}", status));
    }

    if let Some(branch) = &args.branch {
        locator_parts.push(format!("branch:{}", branch));
    }

    if args.running {
        locator_parts.push("running:true".to_string());
    }

    if args.queued {
        locator_parts.push("state:queued".to_string());
    }

    locator_parts.push(format!("count:{}", args.limit));

    let locator = if locator_parts.is_empty() {
        None
    } else {
        Some(locator_parts.join(","))
    };

    let builds = build_api::get_all_builds(&client.config, locator.as_deref(), None)
        .await
        .context("Failed to fetch builds")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("{}", output::format_builds_table(&builds));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&builds)?);
        }
    }

    Ok(())
}

async fn handle_build_get(
    client: &TeamCityClient,
    args: BuildGetArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let build = build_api::get_build(&client.config, &args.build_id, None)
        .await
        .context("Failed to fetch build")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("{}", output::format_build_details(&build));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&build)?);
        }
    }

    Ok(())
}

async fn handle_build_trigger(
    client: &TeamCityClient,
    args: BuildTriggerArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let mut build = Build::default();
    build.build_type_id = Some(args.build_type.clone());

    if let Some(branch) = &args.branch {
        build.branch_name = Some(branch.clone());
    }

    if !args.parameter.is_empty() {
        let properties = parse_parameters(&args.parameter)?;
        build.properties = Some(Box::new(properties));
    }

    let queued_build = build_queue_api::add_build_to_queue(&client.config, None, Some(build))
        .await
        .context("Failed to trigger build")?;

    let build_id = queued_build.id;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("Build triggered successfully!");
            println!(
                "Build ID: {}",
                build_id.map_or("N/A".to_string(), |id| id.to_string())
            );
            if let Some(number) = &queued_build.number {
                println!("Build Number: {}", number);
            }
            if let Some(web_url) = &queued_build.web_url {
                println!("Web URL: {}", web_url);
            }
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&queued_build)?);
        }
    }

    if args.wait || args.watch {
        if let Some(id) = build_id {
            wait_for_build_completion(client, id, args.watch, output_format).await?;
        }
    }

    Ok(())
}

async fn wait_for_build_completion(
    client: &TeamCityClient,
    build_id: i64,
    watch: bool,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    use std::time::Duration;

    loop {
        let build = build_api::get_build(&client.config, &build_id.to_string(), None)
            .await
            .context("Failed to fetch build status")?;

        let state = build
            .state
            .as_ref()
            .map(|s| format!("{:?}", s))
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let status = build.status.as_deref().unwrap_or("UNKNOWN");

        if watch {
            print!("\r\x1b[2K");
            print!("Build {}: State: {}, Status: {}", build_id, state, status);
            use std::io::Write;
            std::io::stdout().flush().ok();
        }

        let is_finished = matches!(
            build.state.as_ref().map(|s| format!("{:?}", s)).as_deref(),
            Some("finished")
        );

        if is_finished {
            if watch {
                println!();
            }

            match output_format {
                crate::cli::OutputFormat::Table => {
                    println!("\nBuild {} finished with status: {}", build_id, status);
                    if let Some(web_url) = &build.web_url {
                        println!("Web URL: {}", web_url);
                    }
                }
                crate::cli::OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&build)?);
                }
            }
            break;
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    Ok(())
}

async fn handle_build_cancel(
    client: &TeamCityClient,
    args: BuildCancelArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let mut cancel_request = BuildCancelRequest::default();

    if let Some(comment) = &args.comment {
        cancel_request.comment = Some(comment.clone());
    }

    let build = build_api::cancel_build(&client.config, &args.build_id, None, Some(cancel_request))
        .await
        .context("Failed to cancel build")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("Build canceled successfully!");
            println!(
                "Build ID: {}",
                build.id.map_or("N/A".to_string(), |id| id.to_string())
            );
            if let Some(status) = &build.status {
                println!("Status: {}", status);
            }
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&build)?);
        }
    }

    Ok(())
}

async fn handle_build_status(
    client: &TeamCityClient,
    args: BuildStatusArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let status = build_api::get_build_status(&client.config, &args.build_id)
        .await
        .context("Failed to get build status")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("Build ID: {}", args.build_id);
            println!("Status: {}", status);
        }
        crate::cli::OutputFormat::Json => {
            let status_obj = serde_json::json!({
                "build_id": args.build_id,
                "status": status
            });
            println!("{}", serde_json::to_string_pretty(&status_obj)?);
        }
    }

    Ok(())
}

async fn handle_build_log(client: &TeamCityClient, args: BuildLogArgs) -> Result<()> {
    let build = build_api::get_build(&client.config, &args.build_id, None)
        .await
        .context("Failed to fetch build")?;

    let build_type_id = build.build_type_id.context("Build type ID not found")?;

    let log_url = format!(
        "{}/downloadBuildLog.html?buildTypeId={}&buildId={}",
        client.config.base_path, build_type_id, args.build_id
    );

    if args.download {
        println!("Log download URL: {}", log_url);
        println!("Use curl or wget to download:");
        println!(
            "  curl -H \"Authorization: Bearer $TOKEN\" \"{}\" -o build_{}.log",
            log_url, args.build_id
        );
        return Ok(());
    }

    if args.follow {
        println!("Note: --follow mode streams the build log. Press Ctrl+C to stop.");
        println!("Log URL: {}\n", log_url);
        stream_build_log(client, &args.build_id).await?;
    } else {
        let log_content = fetch_build_log(client, &args.build_id, args.tail).await?;
        println!("{}", log_content);
    }

    Ok(())
}

async fn fetch_build_log(
    client: &TeamCityClient,
    build_id: &str,
    tail_lines: u32,
) -> Result<String> {
    let build = build_api::get_build(&client.config, build_id, None)
        .await
        .context("Failed to fetch build")?;

    let build_type_id = build.build_type_id.context("Build type ID not found")?;

    let log_url = format!(
        "{}/downloadBuildLog.html?buildTypeId={}&buildId={}",
        client.config.base_path, build_type_id, build_id
    );

    let response = client
        .config
        .client
        .get(&log_url)
        .header("Accept", "text/plain")
        .send()
        .await
        .context("Failed to fetch build log")?;

    let log_text = response
        .text()
        .await
        .context("Failed to read log content")?;

    if tail_lines > 0 {
        let lines: Vec<&str> = log_text.lines().collect();
        let start = if lines.len() > tail_lines as usize {
            lines.len() - tail_lines as usize
        } else {
            0
        };
        Ok(lines[start..].join("\n"))
    } else {
        Ok(log_text)
    }
}

async fn stream_build_log(client: &TeamCityClient, build_id: &str) -> Result<()> {
    use std::io::Write;
    use std::time::Duration;

    let mut last_size = 0u64;

    loop {
        let build = build_api::get_build(&client.config, build_id, None)
            .await
            .context("Failed to fetch build status")?;

        let is_finished = matches!(
            build.state.as_ref().map(|s| format!("{:?}", s)).as_deref(),
            Some("finished")
        );

        let log_content = fetch_build_log(client, build_id, 0).await?;
        let current_size = log_content.len() as u64;

        if current_size > last_size {
            let new_content = &log_content[last_size as usize..];
            print!("{}", new_content);
            std::io::stdout().flush().ok();
            last_size = current_size;
        }

        if is_finished {
            break;
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}

async fn handle_build_artifacts(
    client: &TeamCityClient,
    args: BuildArtifactsArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    match args.command {
        crate::cli::ArtifactCommands::List { path } => {
            let files = if let Some(p) = path {
                build_api::get_files_list_for_subpath_of_build(
                    &client.config,
                    &format!("/{}", p),
                    &args.build_id,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
                .await
                .context("Failed to fetch artifacts")?
            } else {
                build_api::get_files_list_of_build(
                    &client.config,
                    &args.build_id,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
                .await
                .context("Failed to fetch artifacts")?
            };

            match output_format {
                crate::cli::OutputFormat::Table => {
                    println!("{}", output::format_artifacts_table(&files));
                }
                crate::cli::OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&files)?);
                }
            }
        }
        crate::cli::ArtifactCommands::Download { path, output } => {
            download_artifact(client, &args.build_id, &path, &output).await?;
        }
    }

    Ok(())
}

async fn download_artifact(
    client: &TeamCityClient,
    build_id: &str,
    artifact_path: &str,
    output_dir: &str,
) -> Result<()> {
    let artifact_url = format!(
        "{}/app/rest/builds/id:{}/artifacts/files/{}",
        client.config.base_path, build_id, artifact_path
    );

    println!("Downloading artifact: {}", artifact_path);
    println!("Output directory: {}", output_dir);

    let response = client
        .config
        .client
        .get(&artifact_url)
        .send()
        .await
        .context("Failed to download artifact")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to download artifact: HTTP {}", response.status());
    }

    let file_name = artifact_path.rsplit('/').next().unwrap_or(artifact_path);
    let output_path = format!("{}/{}", output_dir.trim_end_matches('/'), file_name);

    let content = response
        .bytes()
        .await
        .context("Failed to read artifact content")?;

    std::fs::create_dir_all(output_dir).context("Failed to create output directory")?;
    std::fs::write(&output_path, &content).context("Failed to write artifact file")?;

    println!("Artifact downloaded to: {}", output_path);

    Ok(())
}

async fn handle_build_tags(
    client: &TeamCityClient,
    args: BuildTagsArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    match args.command {
        crate::cli::TagCommands::List => {
            let tags = build_api::get_build_tags(&client.config, &args.build_id, None, None)
                .await
                .context("Failed to fetch build tags")?;

            match output_format {
                crate::cli::OutputFormat::Table => {
                    println!("{}", output::format_tags_table(&tags));
                }
                crate::cli::OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&tags)?);
                }
            }
        }
        crate::cli::TagCommands::Add { tag } => {
            let mut new_tag = Tag::default();
            new_tag.name = Some(tag.clone());

            let mut tags = Tags::default();
            tags.tag = Some(vec![new_tag]);

            let result =
                build_api::add_tags_to_build(&client.config, &args.build_id, None, Some(tags))
                    .await
                    .context("Failed to add tag")?;

            match output_format {
                crate::cli::OutputFormat::Table => {
                    println!("Tag '{}' added to build {}", tag, args.build_id);
                }
                crate::cli::OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
            }
        }
        crate::cli::TagCommands::Remove { tag } => {
            let current_tags =
                build_api::get_build_tags(&client.config, &args.build_id, None, None)
                    .await
                    .context("Failed to fetch current tags")?;

            let filtered_tags: Vec<Tag> = current_tags
                .tag
                .unwrap_or_default()
                .into_iter()
                .filter(|t| t.name.as_deref() != Some(tag.as_str()))
                .collect();

            let mut updated_tags = Tags::default();
            updated_tags.tag = Some(filtered_tags);

            let result = build_api::set_build_tags(
                &client.config,
                &args.build_id,
                None,
                None,
                Some(updated_tags),
            )
            .await
            .context("Failed to remove tag")?;

            match output_format {
                crate::cli::OutputFormat::Table => {
                    println!("Tag '{}' removed from build {}", tag, args.build_id);
                }
                crate::cli::OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
            }
        }
    }

    Ok(())
}

async fn handle_build_pin(
    client: &TeamCityClient,
    args: BuildPinArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let mut pin_info = PinInfo::default();
    pin_info.status = Some(!args.unpin);

    if let Some(comment) = &args.comment {
        let mut comment_obj = api::models::Comment::default();
        comment_obj.text = Some(comment.clone());
        pin_info.comment = Some(Box::new(comment_obj));
    }

    let result =
        build_api::set_build_pin_info(&client.config, &args.build_id, None, Some(pin_info))
            .await
            .context("Failed to update pin status")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            if args.unpin {
                println!("Build {} unpinned", args.build_id);
            } else {
                println!("Build {} pinned", args.build_id);
            }
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }

    Ok(())
}

fn parse_parameters(params: &[String]) -> Result<Properties> {
    let properties: Vec<Property> = params
        .iter()
        .map(|p| {
            let parts: Vec<&str> = p.splitn(2, '=').collect();
            if parts.len() != 2 {
                anyhow::bail!("Invalid parameter format: {}. Expected KEY=VALUE", p);
            }
            Ok(Property {
                name: Some(parts[0].to_string()),
                value: Some(parts[1].to_string()),
                ..Default::default()
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Properties {
        property: Some(properties),
        ..Default::default()
    })
}
