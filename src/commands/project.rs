use anyhow::{Context, Result};
use api::apis::build_type_api;
use api::apis::project_api;

use crate::client::TeamCityClient;
use crate::output;

pub struct ProjectListArgs {
    pub parent: Option<String>,
    pub archived: bool,
}

pub struct ProjectGetArgs {
    pub project_id: String,
}

pub struct ProjectBuildTypesArgs {
    pub project_id: String,
}

pub struct BuildTypeListArgs {
    pub project_id: Option<String>,
}

pub struct BuildTypeGetArgs {
    pub build_type_id: String,
}

pub struct BuildTypeParametersArgs {
    pub build_type_id: String,
}

pub async fn handle_project_command(
    client: &TeamCityClient,
    command: crate::cli::ProjectCommands,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    match command {
        crate::cli::ProjectCommands::List { parent, archived } => {
            handle_project_list(client, ProjectListArgs { parent, archived }, output_format).await
        }
        crate::cli::ProjectCommands::Get { project_id } => {
            handle_project_get(client, ProjectGetArgs { project_id }, output_format).await
        }
        crate::cli::ProjectCommands::BuildTypes { project_id } => {
            handle_project_build_types(client, ProjectBuildTypesArgs { project_id }, output_format)
                .await
        }
    }
}

pub async fn handle_buildtype_command(
    client: &TeamCityClient,
    command: crate::cli::BuildTypeCommands,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    match command {
        crate::cli::BuildTypeCommands::List { project_id } => {
            handle_buildtype_list(client, BuildTypeListArgs { project_id }, output_format).await
        }
        crate::cli::BuildTypeCommands::Get { build_type_id } => {
            handle_buildtype_get(client, BuildTypeGetArgs { build_type_id }, output_format).await
        }
        crate::cli::BuildTypeCommands::Parameters { build_type_id } => {
            handle_buildtype_parameters(
                client,
                BuildTypeParametersArgs { build_type_id },
                output_format,
            )
            .await
        }
    }
}

async fn handle_project_list(
    client: &TeamCityClient,
    args: ProjectListArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let mut locator_parts = Vec::new();

    if let Some(parent) = &args.parent {
        locator_parts.push(format!("parent:(id:{})", parent));
    }

    if !args.archived {
        locator_parts.push("archived:false".to_string());
    }

    let locator = if locator_parts.is_empty() {
        None
    } else {
        Some(locator_parts.join(","))
    };

    let projects = project_api::get_all_projects(&client.config, locator.as_deref(), None)
        .await
        .context("Failed to fetch projects")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("{}", output::format_projects_table(&projects));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&projects)?);
        }
    }

    Ok(())
}

async fn handle_project_get(
    client: &TeamCityClient,
    args: ProjectGetArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let project = project_api::get_project(&client.config, &args.project_id, None)
        .await
        .context("Failed to fetch project")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("{}", output::format_project_details(&project));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&project)?);
        }
    }

    Ok(())
}

async fn handle_project_build_types(
    client: &TeamCityClient,
    args: ProjectBuildTypesArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let build_types =
        project_api::get_all_build_types_ordered(&client.config, &args.project_id, None)
            .await
            .context("Failed to fetch build types")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("{}", output::format_build_types_table(&build_types));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&build_types)?);
        }
    }

    Ok(())
}

async fn handle_buildtype_list(
    client: &TeamCityClient,
    args: BuildTypeListArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let locator = args.project_id.map(|p| format!("project:(id:{})", p));

    let build_types = build_type_api::get_all_build_types(&client.config, locator.as_deref(), None)
        .await
        .context("Failed to fetch build types")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("{}", output::format_build_types_table(&build_types));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&build_types)?);
        }
    }

    Ok(())
}

async fn handle_buildtype_get(
    client: &TeamCityClient,
    args: BuildTypeGetArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let build_type = build_type_api::get_build_type(&client.config, &args.build_type_id, None)
        .await
        .context("Failed to fetch build type")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("{}", output::format_build_type_details(&build_type));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&build_type)?);
        }
    }

    Ok(())
}

async fn handle_buildtype_parameters(
    client: &TeamCityClient,
    args: BuildTypeParametersArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let parameters = build_type_api::get_build_parameters_of_build_type(
        &client.config,
        &args.build_type_id,
        None,
        None,
    )
    .await
    .context("Failed to fetch build type parameters")?;

    match output_format {
        crate::cli::OutputFormat::Table => {
            println!("{}", output::format_properties_table(&parameters));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&parameters)?);
        }
    }

    Ok(())
}
