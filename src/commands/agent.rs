use anyhow::{Context, Result};
use api::apis::{agent_api, agent_pool_api};
use api::models::{AuthorizedInfo, EnabledInfo};

use crate::client::TeamCityClient;
use crate::output;

pub struct AgentListArgs {
    pub connected: Option<bool>,
    pub authorized: Option<bool>,
    pub enabled: Option<bool>,
}

pub struct AgentGetArgs {
    pub agent_id: String,
}

pub struct AgentEnableArgs {
    pub agent_id: String,
}

pub struct AgentDisableArgs {
    pub agent_id: String,
    pub comment: Option<String>,
}

pub struct AgentAuthorizeArgs {
    pub agent_id: String,
}

pub struct AgentUnauthorizeArgs {
    pub agent_id: String,
}

pub struct AgentPoolListArgs;

pub struct AgentPoolGetArgs {
    pub pool_id: String,
}

pub async fn handle_agent_command(
    client: &TeamCityClient,
    command: crate::cli::AgentCommands,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    match command {
        crate::cli::AgentCommands::List {
            connected,
            authorized,
            enabled,
        } => {
            handle_agent_list(
                client,
                AgentListArgs {
                    connected,
                    authorized,
                    enabled,
                },
                output_format,
            )
            .await
        }
        crate::cli::AgentCommands::Get { agent_id } => {
            handle_agent_get(client, AgentGetArgs { agent_id }, output_format).await
        }
        crate::cli::AgentCommands::Enable { agent_id } => {
            handle_agent_enable(client, AgentEnableArgs { agent_id }, output_format).await
        }
        crate::cli::AgentCommands::Disable { agent_id, comment } => {
            handle_agent_disable(
                client,
                AgentDisableArgs { agent_id, comment },
                output_format,
            )
            .await
        }
        crate::cli::AgentCommands::Authorize { agent_id } => {
            handle_agent_authorize(client, AgentAuthorizeArgs { agent_id }, output_format).await
        }
        crate::cli::AgentCommands::Unauthorize { agent_id } => {
            handle_agent_unauthorize(client, AgentUnauthorizeArgs { agent_id }, output_format).await
        }
        crate::cli::AgentCommands::Pool { command } => {
            handle_agent_pool_command(client, command, output_format).await
        }
    }
}

pub async fn handle_agent_pool_command(
    client: &TeamCityClient,
    command: crate::cli::AgentPoolCommands,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    match command {
        crate::cli::AgentPoolCommands::List => {
            handle_agent_pool_list(client, AgentPoolListArgs, output_format).await
        }
        crate::cli::AgentPoolCommands::Get { pool_id } => {
            handle_agent_pool_get(client, AgentPoolGetArgs { pool_id }, output_format).await
        }
    }
}

async fn handle_agent_list(
    client: &TeamCityClient,
    args: AgentListArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let mut locator_parts = Vec::new();

    if let Some(connected) = args.connected {
        locator_parts.push(format!("connected:{}", connected));
    }

    if let Some(authorized) = args.authorized {
        locator_parts.push(format!("authorized:{}", authorized));
    }

    if let Some(enabled) = args.enabled {
        locator_parts.push(format!("enabled:{}", enabled));
    }

    let locator = if locator_parts.is_empty() {
        None
    } else {
        Some(locator_parts.join(","))
    };

    let agents = agent_api::get_all_agents(&client.config, locator.as_deref(), None)
        .await
        .context("Failed to fetch agents")?;

    match output_format {
        crate::cli::OutputFormat::Human => {
            println!("{}", output::format_agents_table(&agents));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&agents)?);
        }
    }

    Ok(())
}

async fn handle_agent_get(
    client: &TeamCityClient,
    args: AgentGetArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let agent = agent_api::get_agent(&client.config, &args.agent_id, None)
        .await
        .context("Failed to fetch agent")?;

    match output_format {
        crate::cli::OutputFormat::Human => {
            println!("{}", output::format_agent_details(&agent));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&agent)?);
        }
    }

    Ok(())
}

async fn handle_agent_enable(
    client: &TeamCityClient,
    args: AgentEnableArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let mut enabled_info = EnabledInfo::default();
    enabled_info.status = Some(true);

    let result =
        agent_api::set_enabled_info(&client.config, &args.agent_id, None, Some(enabled_info))
            .await
            .context("Failed to enable agent")?;

    match output_format {
        crate::cli::OutputFormat::Human => {
            println!("Agent {} enabled", args.agent_id);
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }

    Ok(())
}

async fn handle_agent_disable(
    client: &TeamCityClient,
    args: AgentDisableArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let mut enabled_info = EnabledInfo::default();
    enabled_info.status = Some(false);

    if let Some(comment) = &args.comment {
        let mut comment_obj = api::models::Comment::default();
        comment_obj.text = Some(comment.clone());
        enabled_info.comment = Some(Box::new(comment_obj));
    }

    let result =
        agent_api::set_enabled_info(&client.config, &args.agent_id, None, Some(enabled_info))
            .await
            .context("Failed to disable agent")?;

    match output_format {
        crate::cli::OutputFormat::Human => {
            println!("Agent {} disabled", args.agent_id);
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }

    Ok(())
}

async fn handle_agent_authorize(
    client: &TeamCityClient,
    args: AgentAuthorizeArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let mut authorized_info = AuthorizedInfo::default();
    authorized_info.status = Some(true);

    let result =
        agent_api::set_authorized_info(&client.config, &args.agent_id, None, Some(authorized_info))
            .await
            .context("Failed to authorize agent")?;

    match output_format {
        crate::cli::OutputFormat::Human => {
            println!("Agent {} authorized", args.agent_id);
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }

    Ok(())
}

async fn handle_agent_unauthorize(
    client: &TeamCityClient,
    args: AgentUnauthorizeArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let mut authorized_info = AuthorizedInfo::default();
    authorized_info.status = Some(false);

    let result =
        agent_api::set_authorized_info(&client.config, &args.agent_id, None, Some(authorized_info))
            .await
            .context("Failed to unauthorize agent")?;

    match output_format {
        crate::cli::OutputFormat::Human => {
            println!("Agent {} unauthorized", args.agent_id);
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }

    Ok(())
}

async fn handle_agent_pool_list(
    client: &TeamCityClient,
    _args: AgentPoolListArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let pools = agent_pool_api::get_all_agent_pools(&client.config, None, None)
        .await
        .context("Failed to fetch agent pools")?;

    match output_format {
        crate::cli::OutputFormat::Human => {
            println!("{}", output::format_agent_pools_table(&pools));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&pools)?);
        }
    }

    Ok(())
}

async fn handle_agent_pool_get(
    client: &TeamCityClient,
    args: AgentPoolGetArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    let pool = agent_pool_api::get_agent_pool_of_agent_pool(&client.config, &args.pool_id, None)
        .await
        .context("Failed to fetch agent pool")?;

    match output_format {
        crate::cli::OutputFormat::Human => {
            println!("{}", output::format_agent_pool_details(&pool));
        }
        crate::cli::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&pool)?);
        }
    }

    Ok(())
}
