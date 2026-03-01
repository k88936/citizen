use anyhow::{Context, Result};
use api::apis::build_api;
use api::apis::build_queue_api;
use api::apis::build_type_api;
use api::apis::project_api;
use api::models::build::State;
use api::models::{Build, Projects};
use inquire::{Select, Text};
use std::fmt;

use crate::client::TeamCityClient;

#[derive(Clone)]
pub struct InteractiveBuild {
    pub id: i64,
    pub number: String,
    pub build_type_name: String,
    pub status: String,
    pub branch: Option<String>,
}

#[derive(Clone)]
pub struct InteractiveProject {
    pub id: String,
    pub name: String,
    #[allow(dead_code)]
    pub parent_project_id: Option<String>,
}

#[derive(Clone)]
pub struct InteractiveBuildType {
    pub id: String,
    pub name: String,
    pub project_name: String,
}

impl fmt::Display for InteractiveBuild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let branch = self
            .branch
            .as_deref()
            .map(|b| format!(" [{}]", b))
            .unwrap_or_default();
        let status_icon = match self.status.as_str() {
            "SUCCESS" => "✓",
            "FAILURE" => "✗",
            "ERROR" => "⚠",
            _ => "○",
        };
        write!(
            f,
            "{} {} {}{} - {}",
            status_icon, self.number, self.build_type_name, branch, self.id
        )
    }
}

impl fmt::Display for InteractiveProject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.id)
    }
}

impl fmt::Display for InteractiveBuildType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} / {} ({})", self.project_name, self.name, self.id)
    }
}

pub async fn run_interactive_mode(client: &TeamCityClient) -> Result<()> {
    loop {
        let action = Select::new(
            "What would you like to do?",
            vec![
                InteractiveAction::TriggerBuild,
                InteractiveAction::ViewBuilds,
                InteractiveAction::ViewProjects,
                InteractiveAction::ViewBuildTypes,
                InteractiveAction::ViewQueue,
                InteractiveAction::Exit,
            ],
        )
        .prompt()?;

        match action {
            InteractiveAction::TriggerBuild => {
                if let Err(e) = interactive_trigger_build(client).await {
                    println!("Error: {}", e);
                }
            }
            InteractiveAction::ViewBuilds => {
                if let Err(e) = interactive_view_builds(client).await {
                    println!("Error: {}", e);
                }
            }
            InteractiveAction::ViewProjects => {
                if let Err(e) = interactive_view_projects(client).await {
                    println!("Error: {}", e);
                }
            }
            InteractiveAction::ViewBuildTypes => {
                if let Err(e) = interactive_view_build_types(client).await {
                    println!("Error: {}", e);
                }
            }
            InteractiveAction::ViewQueue => {
                if let Err(e) = interactive_view_queue(client).await {
                    println!("Error: {}", e);
                }
            }
            InteractiveAction::Exit => {
                println!("Goodbye!");
                break;
            }
        }
    }

    Ok(())
}

#[derive(Clone, Copy)]
enum InteractiveAction {
    TriggerBuild,
    ViewBuilds,
    ViewProjects,
    ViewBuildTypes,
    ViewQueue,
    Exit,
}

impl fmt::Display for InteractiveAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InteractiveAction::TriggerBuild => write!(f, "* Trigger a build"),
            InteractiveAction::ViewBuilds => write!(f, "* View recent builds"),
            InteractiveAction::ViewProjects => write!(f, "* Browse projects"),
            InteractiveAction::ViewBuildTypes => write!(f, "* Browse build types"),
            InteractiveAction::ViewQueue => write!(f, "* View build queue"),
            InteractiveAction::Exit => write!(f, "x Exit"),
        }
    }
}

async fn interactive_trigger_build(client: &TeamCityClient) -> Result<()> {
    println!("Loading build types...");
    let build_types = fetch_all_build_types(client).await?;

    if build_types.is_empty() {
        println!("No build types found.");
        return Ok(());
    }

    let build_type = fuzzy_select_build_type(&build_types)?;

    let branch = Text::new("Branch (leave empty for default):")
        .with_default("")
        .prompt()?;

    let branch = if branch.is_empty() {
        None
    } else {
        Some(branch)
    };

    let should_trigger = Select::new(
        "Ready to trigger. Proceed?",
        vec![
            TriggerChoice::Yes,
            TriggerChoice::YesAndWait,
            TriggerChoice::No,
        ],
    )
    .prompt()?;

    match should_trigger {
        TriggerChoice::Yes => {
            trigger_build(client, &build_type.id, branch.clone(), None, false).await?;
        }
        TriggerChoice::YesAndWait => {
            trigger_build(client, &build_type.id, branch.clone(), None, true).await?;
        }
        TriggerChoice::No => {
            println!("Cancelled.");
        }
    }

    Ok(())
}

#[derive(Clone, Copy)]
enum TriggerChoice {
    Yes,
    YesAndWait,
    No,
}

impl fmt::Display for TriggerChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TriggerChoice::Yes => write!(f, "Yes, trigger now"),
            TriggerChoice::YesAndWait => write!(f, "Yes, and wait for completion"),
            TriggerChoice::No => write!(f, "Cancel"),
        }
    }
}

async fn interactive_view_builds(client: &TeamCityClient) -> Result<()> {
    println!("Loading recent builds...");
    let builds = fetch_recent_builds(client, 50).await?;

    if builds.is_empty() {
        println!("No builds found.");
        return Ok(());
    }

    let build = fuzzy_select_build(&builds)?;

    show_build_details(client, build.id).await?;

    println!("\nCommands:");
    println!("  citizen build log {}          # View build log", build.id);
    println!("  citizen build log {} --follow # Follow build log", build.id);
    println!("  citizen build artifacts {} list # List artifacts", build.id);

    Ok(())
}

async fn interactive_view_projects(client: &TeamCityClient) -> Result<()> {
    println!("Loading projects...");
    let projects = fetch_all_projects(client).await?;

    if projects.is_empty() {
        println!("No projects found.");
        return Ok(());
    }

    let project = fuzzy_select_project(&projects)?;
    show_project_details(client, &project.id).await?;

    Ok(())
}

async fn interactive_view_build_types(client: &TeamCityClient) -> Result<()> {
    println!("Loading build types...");
    let build_types = fetch_all_build_types(client).await?;

    if build_types.is_empty() {
        println!("No build types found.");
        return Ok(());
    }

    let build_type = fuzzy_select_build_type(&build_types)?;

    show_build_type_details(client, &build_type.id).await?;

    Ok(())
}

async fn interactive_view_queue(client: &TeamCityClient) -> Result<()> {
    println!("Loading build queue...");
    let queued_builds = build_queue_api::get_all_queued_builds(&client.config, None, None)
        .await
        .context("Failed to fetch build queue")?;

    let builds: Vec<InteractiveBuild> = queued_builds
        .build
        .unwrap_or_default()
        .into_iter()
        .filter_map(|b| {
            Some(InteractiveBuild {
                id: b.id?,
                number: b.number.unwrap_or_else(|| "?".to_string()),
                build_type_name: b
                    .build_type
                    .as_ref()
                    .and_then(|bt| bt.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string()),
                status: "QUEUED".to_string(),
                branch: b.branch_name,
            })
        })
        .collect();

    if builds.is_empty() {
        println!("Build queue is empty.");
        return Ok(());
    }

    let build = fuzzy_select_build(&builds)?;

    let action = Select::new(
        "What would you like to do?",
        vec![
            QueueAction::ViewDetails,
            QueueAction::Cancel,
            QueueAction::Back,
        ],
    )
    .prompt()?;

    match action {
        QueueAction::ViewDetails => {
            show_build_details(client, build.id).await?;
        }
        QueueAction::Cancel => {
            cancel_queued_build(client, build.id).await?;
        }
        QueueAction::Back => {}
    }

    Ok(())
}

#[derive(Clone, Copy)]
enum QueueAction {
    ViewDetails,
    Cancel,
    Back,
}

impl fmt::Display for QueueAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueueAction::ViewDetails => write!(f, "View details"),
            QueueAction::Cancel => write!(f, "Cancel this build"),
            QueueAction::Back => write!(f, "Back"),
        }
    }
}

fn fuzzy_select_build(builds: &[InteractiveBuild]) -> Result<InteractiveBuild> {
    let ans = Select::new("Select a build:", builds.to_vec())
        .with_page_size(10)
        .prompt()?;

    Ok(ans)
}

fn fuzzy_select_project(projects: &[InteractiveProject]) -> Result<InteractiveProject> {
    let ans = Select::new("Select a project:", projects.to_vec())
        .with_page_size(10)
        .prompt()?;

    Ok(ans)
}

fn fuzzy_select_build_type(build_types: &[InteractiveBuildType]) -> Result<InteractiveBuildType> {
    let ans = Select::new("Select a build type:", build_types.to_vec())
        .with_page_size(10)
        .prompt()?;

    Ok(ans)
}

async fn fetch_recent_builds(client: &TeamCityClient, limit: u32) -> Result<Vec<InteractiveBuild>> {
    let builds = build_api::get_all_builds(&client.config, Some(&format!("count:{}", limit)), None)
        .await
        .context("Failed to fetch builds")?;

    let interactive_builds: Vec<InteractiveBuild> = builds
        .build
        .unwrap_or_default()
        .into_iter()
        .filter_map(|b| {
            Some(InteractiveBuild {
                id: b.id?,
                number: b.number.unwrap_or_else(|| "?".to_string()),
                build_type_name: b
                    .build_type
                    .as_ref()
                    .and_then(|bt| bt.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string()),
                status: b.status.unwrap_or_else(|| "UNKNOWN".to_string()),
                branch: b.branch_name,
            })
        })
        .collect();

    Ok(interactive_builds)
}

async fn fetch_all_projects(client: &TeamCityClient) -> Result<Vec<InteractiveProject>> {
    let projects = project_api::get_all_projects(&client.config, None, None)
        .await
        .context("Failed to fetch projects")?;

    let interactive_projects: Vec<InteractiveProject> = projects
        .project
        .unwrap_or_default()
        .into_iter()
        .filter_map(|p| {
            Some(InteractiveProject {
                id: p.id?,
                name: p.name.unwrap_or_else(|| "Unknown".to_string()),
                parent_project_id: p.parent_project_id,
            })
        })
        .collect();

    Ok(interactive_projects)
}

async fn fetch_all_build_types(client: &TeamCityClient) -> Result<Vec<InteractiveBuildType>> {
    let build_types = build_type_api::get_all_build_types(&client.config, None, None)
        .await
        .context("Failed to fetch build types")?;

    let interactive_build_types: Vec<InteractiveBuildType> = build_types
        .build_type
        .unwrap_or_default()
        .into_iter()
        .filter_map(|bt| {
            Some(InteractiveBuildType {
                id: bt.id?,
                name: bt.name.unwrap_or_else(|| "Unknown".to_string()),
                project_name: bt
                    .project
                    .as_ref()
                    .and_then(|p| p.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string()),
            })
        })
        .collect();

    Ok(interactive_build_types)
}

async fn trigger_build(
    client: &TeamCityClient,
    build_type_id: &str,
    branch: Option<String>,
    _comment: Option<String>,
    wait: bool,
) -> Result<()> {
    let mut build = Build::default();
    build.build_type_id = Some(build_type_id.to_string());

    if let Some(branch) = &branch {
        build.branch_name = Some(branch.clone());
    }

    let queued_build = build_queue_api::add_build_to_queue(&client.config, None, Some(build))
        .await
        .context("Failed to trigger build")?;

    let build_id = queued_build.id;

    println!(
        "Build triggered! ID: {}",
        build_id.map_or("N/A".to_string(), |id| id.to_string())
    );

    if let Some(web_url) = &queued_build.web_url {
        println!("URL: {}", web_url);
    }

    if wait {
        if let Some(id) = build_id {
            wait_for_build(client, id).await?;
        }
    }

    Ok(())
}

async fn wait_for_build(client: &TeamCityClient, build_id: i64) -> Result<()> {
    use std::io::Write;
    use std::time::Duration;

    println!("Waiting for build {} to complete...", build_id);

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
        let percentage = build.percentage_complete.unwrap_or(0);

        print!("\r\x1b[2K");
        print!(
            "Build {}: {}% - {} ({})",
            build_id, percentage, state, status
        );
        std::io::stdout().flush().ok();

        let is_finished = matches!(
            build.state.as_ref().map(|s| format!("{:?}", s)).as_deref(),
            Some("finished")
        );

        if is_finished {
            println!();
            println!("Build finished with status: {}", status);
            if let Some(web_url) = &build.web_url {
                println!("URL: {}", web_url);
            }
            break;
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    Ok(())
}

async fn show_build_details(client: &TeamCityClient, build_id: i64) -> Result<()> {
    let build = build_api::get_build(&client.config, &build_id.to_string(), None)
        .await
        .context("Failed to fetch build")?;

    println!("\n{}", "─".repeat(50));
    println!("Build Details");
    println!("{}", "─".repeat(50));
    println!("ID: {}", build_id);
    println!(
        "Number: {}",
        build.number.unwrap_or_else(|| "N/A".to_string())
    );
    println!(
        "Status: {}",
        build.status.unwrap_or_else(|| "UNKNOWN".to_string())
    );
    println!("State: {:?}", build.state.unwrap_or(State::Finished));
    if let Some(branch) = &build.branch_name {
        println!("Branch: {}", branch);
    }
    if let Some(bt) = &build.build_type {
        if let Some(name) = &bt.name {
            println!("Build Type: {}", name);
        }
    }
    if let Some(agent) = &build.agent {
        if let Some(name) = &agent.name {
            println!("Agent: {}", name);
        }
    }
    if let Some(start_date) = &build.start_date {
        println!("Started: {}", start_date);
    }
    if let Some(finish_date) = &build.finish_date {
        println!("Finished: {}", finish_date);
    }
    if let Some(web_url) = &build.web_url {
        println!("URL: {}", web_url);
    }
    println!("{}", "─".repeat(50));

    Ok(())
}

async fn show_project_details(client: &TeamCityClient, project_id: &str) -> Result<()> {
    let project = project_api::get_project(&client.config, project_id, None)
        .await
        .context("Failed to fetch project")?;

    println!("\n{}", "─".repeat(50));
    println!("Project Details");
    println!("{}", "─".repeat(50));
    println!("ID: {}", project.id.unwrap_or_else(|| "N/A".to_string()));
    println!(
        "Name: {}",
        project.name.unwrap_or_else(|| "N/A".to_string())
    );
    if let Some(description) = &project.description {
        println!("Description: {}", description);
    }
    if let Some(parent) = &project.parent_project_id {
        println!("Parent Project: {}", parent);
    }
    if let Some(web_url) = &project.web_url {
        println!("URL: {}", web_url);
    }

    if let Some(build_types) = &project.build_types {
        if let Some(types) = &build_types.build_type {
            if !types.is_empty() {
                println!("\nBuild Types:");
                for bt in types {
                    println!(
                        "  - {} ({})",
                        bt.name.as_deref().unwrap_or("?"),
                        bt.id.as_deref().unwrap_or("?")
                    );
                }
            }
        }
    }

    println!("{}", "─".repeat(50));

    Ok(())
}

async fn show_build_type_details(client: &TeamCityClient, build_type_id: &str) -> Result<()> {
    let build_type = build_type_api::get_build_type(&client.config, build_type_id, None)
        .await
        .context("Failed to fetch build type")?;

    println!("\n{}", "─".repeat(50));
    println!("Build Type Details");
    println!("{}", "─".repeat(50));
    println!("ID: {}", build_type.id.unwrap_or_else(|| "N/A".to_string()));
    println!(
        "Name: {}",
        build_type.name.unwrap_or_else(|| "N/A".to_string())
    );
    if let Some(project) = &build_type.project {
        if let Some(name) = &project.name {
            println!("Project: {}", name);
        }
    }
    if let Some(description) = &build_type.description {
        println!("Description: {}", description);
    }
    if let Some(paused) = &build_type.paused {
        println!("Paused: {}", paused);
    }
    if let Some(web_url) = &build_type.web_url {
        println!("URL: {}", web_url);
    }
    println!("{}", "─".repeat(50));

    Ok(())
}

async fn cancel_queued_build(client: &TeamCityClient, build_id: i64) -> Result<()> {
    use api::models::BuildCancelRequest;

    let cancel_request = BuildCancelRequest::default();

    let build = build_queue_api::cancel_queued_build(
        &client.config,
        &build_id.to_string(),
        Some(cancel_request),
    )
    .await
    .context("Failed to cancel queued build")?;

    println!(
        "Build {} cancelled from queue.",
        build.id.map_or("N/A".to_string(), |id| id.to_string())
    );

    Ok(())
}

#[allow(dead_code)]
pub async fn select_build_interactive(
    client: &TeamCityClient,
    build_type: Option<&str>,
    status: Option<&str>,
    branch: Option<&str>,
    running: bool,
) -> Result<Option<InteractiveBuild>> {
    let mut locator_parts = Vec::new();

    if let Some(bt) = build_type {
        locator_parts.push(format!("buildType:({})", bt));
    }

    if let Some(s) = status {
        locator_parts.push(format!("status:{}", s));
    }

    if let Some(b) = branch {
        locator_parts.push(format!("branch:{}", b));
    }

    if running {
        locator_parts.push("running:true".to_string());
    }

    locator_parts.push("count:50".to_string());

    let locator = locator_parts.join(",");

    let builds = build_api::get_all_builds(&client.config, Some(&locator), None)
        .await
        .context("Failed to fetch builds")?;

    let interactive_builds: Vec<InteractiveBuild> = builds
        .build
        .unwrap_or_default()
        .into_iter()
        .filter_map(|b| {
            Some(InteractiveBuild {
                id: b.id?,
                number: b.number.unwrap_or_else(|| "?".to_string()),
                build_type_name: b
                    .build_type
                    .as_ref()
                    .and_then(|bt| bt.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string()),
                status: b.status.unwrap_or_else(|| "UNKNOWN".to_string()),
                branch: b.branch_name,
            })
        })
        .collect();

    if interactive_builds.is_empty() {
        return Ok(None);
    }

    let build = fuzzy_select_build(&interactive_builds)?;
    Ok(Some(build))
}

pub async fn select_build_type_interactive(
    client: &TeamCityClient,
    project_id: Option<&str>,
) -> Result<Option<InteractiveBuildType>> {
    let locator = project_id.map(|p| format!("project:(id:{})", p));

    let build_types = build_type_api::get_all_build_types(&client.config, locator.as_deref(), None)
        .await
        .context("Failed to fetch build types")?;

    let interactive_build_types: Vec<InteractiveBuildType> = build_types
        .build_type
        .unwrap_or_default()
        .into_iter()
        .filter_map(|bt| {
            Some(InteractiveBuildType {
                id: bt.id?,
                name: bt.name.unwrap_or_else(|| "Unknown".to_string()),
                project_name: bt
                    .project
                    .as_ref()
                    .and_then(|p| p.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string()),
            })
        })
        .collect();

    if interactive_build_types.is_empty() {
        return Ok(None);
    }

    let build_type = fuzzy_select_build_type(&interactive_build_types)?;
    Ok(Some(build_type))
}

#[allow(dead_code)]
pub async fn select_project_interactive(
    client: &TeamCityClient,
    parent_id: Option<&str>,
) -> Result<Option<InteractiveProject>> {
    let projects = if let Some(parent) = parent_id {
        let project = project_api::get_project(&client.config, parent, None)
            .await
            .context("Failed to fetch parent project")?;

        let subprojects = project
            .projects
            .unwrap_or_else(|| Box::new(Projects::default()));

        subprojects
            .project
            .unwrap_or_default()
            .into_iter()
            .filter_map(|p| {
                Some(InteractiveProject {
                    id: p.id?,
                    name: p.name.unwrap_or_else(|| "Unknown".to_string()),
                    parent_project_id: p.parent_project_id,
                })
            })
            .collect()
    } else {
        fetch_all_projects(client).await?
    };

    if projects.is_empty() {
        return Ok(None);
    }

    let project = fuzzy_select_project(&projects)?;
    Ok(Some(project))
}
