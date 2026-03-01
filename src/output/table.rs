use api::models::{
    Build, BuildType, BuildTypes, Builds, Files, Project, Projects, Properties, Server, Tags,
};
use colored::Colorize;
use tabled::{
    Table, Tabled,
    settings::{Alignment, Modify, Style, object::Rows},
};

#[derive(Tabled)]
struct BuildRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Build Type")]
    build_type: String,
    #[tabled(rename = "Number")]
    number: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Branch")]
    branch: String,
    #[tabled(rename = "State")]
    state: String,
    #[tabled(rename = "Started")]
    started: String,
}

pub fn format_builds_table(builds: &Builds) -> String {
    let rows: Vec<BuildRow> = builds
        .build
        .as_ref()
        .map(|b| {
            b.iter()
                .map(|build| BuildRow {
                    id: build.id.map_or("N/A".to_string(), |id| id.to_string()),
                    build_type: build
                        .build_type_id
                        .as_ref()
                        .map_or("N/A".to_string(), |bt| bt.to_string()),
                    number: build
                        .number
                        .as_ref()
                        .map_or("N/A".to_string(), |n| n.to_string()),
                    status: format_status(build.status.as_deref()),
                    branch: build
                        .branch_name
                        .as_ref()
                        .map_or("default".to_string(), |b| b.to_string()),
                    state: build
                        .state
                        .as_ref()
                        .map_or("N/A".to_string(), |s| format!("{:?}", s)),
                    started: build
                        .start_date
                        .as_ref()
                        .map_or("N/A".to_string(), |d| d.to_string()),
                })
                .collect()
        })
        .unwrap_or_default();

    let mut table = Table::new(rows);
    table
        .with(Style::psql())
        .with(Modify::new(Rows::new(1..)).with(Alignment::left()));

    table.to_string()
}

fn format_status(status: Option<&str>) -> String {
    match status {
        Some("SUCCESS") => "SUCCESS".green().to_string(),
        Some("FAILURE") => "FAILURE".red().to_string(),
        Some("ERROR") => "ERROR".red().bold().to_string(),
        Some(s) => s.yellow().to_string(),
        None => "UNKNOWN".yellow().to_string(),
    }
}

pub fn format_build_details(build: &Build) -> String {
    let mut details = Vec::new();

    details.push(format!(
        "{}: {}",
        "ID".bold(),
        build.id.map_or("N/A".to_string(), |id| id.to_string())
    ));
    details.push(format!(
        "{}: {}",
        "Build Type".bold(),
        build
            .build_type_id
            .as_ref()
            .map_or("N/A".to_string(), |bt| bt.to_string())
    ));
    details.push(format!(
        "{}: {}",
        "Number".bold(),
        build
            .number
            .as_ref()
            .map_or("N/A".to_string(), |n| n.to_string())
    ));
    details.push(format!(
        "{}: {}",
        "Status".bold(),
        format_status(build.status.as_deref())
    ));
    details.push(format!(
        "{}: {}",
        "State".bold(),
        build
            .state
            .as_ref()
            .map_or("N/A".to_string(), |s| format!("{:?}", s))
    ));
    details.push(format!(
        "{}: {}",
        "Branch".bold(),
        build
            .branch_name
            .as_ref()
            .map_or("default".to_string(), |b| b.to_string())
    ));

    if let Some(started) = &build.start_date {
        details.push(format!("{}: {}", "Started".bold(), started));
    }

    if let Some(finished) = &build.finish_date {
        details.push(format!("{}: {}", "Finished".bold(), finished));
    }

    if let Some(status_text) = &build.status_text {
        details.push(format!("{}: {}", "Status Text".bold(), status_text));
    }

    if let Some(queued) = &build.queued_date {
        details.push(format!("{}: {}", "Queued".bold(), queued));
    }

    if let Some(web_url) = &build.web_url {
        details.push(format!("{}: {}", "Web URL".bold(), web_url));
    }

    if let Some(comment) = &build.comment {
        if let Some(text) = &comment.text {
            details.push(format!("{}: {}", "Comment".bold(), text));
        }
    }

    details.join("\n")
}

pub fn format_server_info(server: &Server) -> String {
    let mut info = Vec::new();

    info.push(format!(
        "{}: {}",
        "Version".bold(),
        server
            .version
            .as_ref()
            .map_or("N/A".to_string(), |v| v.to_string())
    ));
    info.push(format!(
        "{}: {}",
        "Build Number".bold(),
        server
            .build_number
            .as_ref()
            .map_or("N/A".to_string(), |bn| bn.to_string())
    ));

    if let Some(web_url) = &server.web_url {
        info.push(format!("{}: {}", "Web URL".bold(), web_url));
    }

    if let Some(build_date) = &server.build_date {
        info.push(format!("{}: {}", "Build Date".bold(), build_date));
    }

    if let Some(current_time) = &server.current_time {
        info.push(format!("{}: {}", "Current Time".bold(), current_time));
    }

    if let Some(start_time) = &server.start_time {
        info.push(format!("{}: {}", "Start Time".bold(), start_time));
    }

    if let Some(role) = &server.role {
        info.push(format!("{}: {}", "Role".bold(), role));
    }

    info.join("\n")
}

#[derive(Tabled)]
struct ArtifactRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Size")]
    size: String,
    #[tabled(rename = "Modified")]
    modified: String,
}

pub fn format_artifacts_table(files: &Files) -> String {
    let rows: Vec<ArtifactRow> = files
        .file
        .as_ref()
        .map(|f| {
            f.iter()
                .map(|file| ArtifactRow {
                    name: file.name.clone().unwrap_or_else(|| "N/A".to_string()),
                    size: file
                        .size
                        .map(|s| format_size(s))
                        .unwrap_or_else(|| "-".to_string()),
                    modified: file
                        .modification_time
                        .clone()
                        .unwrap_or_else(|| "N/A".to_string()),
                })
                .collect()
        })
        .unwrap_or_default();

    if rows.is_empty() {
        return "No artifacts found".to_string();
    }

    let mut table = Table::new(rows);
    table
        .with(Style::psql())
        .with(Modify::new(Rows::new(1..)).with(Alignment::left()));

    table.to_string()
}

fn format_size(size: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

#[derive(Tabled)]
struct TagRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Private")]
    private: String,
}

pub fn format_tags_table(tags: &Tags) -> String {
    let rows: Vec<TagRow> = tags
        .tag
        .as_ref()
        .map(|t| {
            t.iter()
                .map(|tag| TagRow {
                    name: tag.name.clone().unwrap_or_else(|| "N/A".to_string()),
                    private: tag
                        .private
                        .map(|p| {
                            if p {
                                "Yes".to_string()
                            } else {
                                "No".to_string()
                            }
                        })
                        .unwrap_or_else(|| "No".to_string()),
                })
                .collect()
        })
        .unwrap_or_default();

    if rows.is_empty() {
        return "No tags found".to_string();
    }

    let mut table = Table::new(rows);
    table
        .with(Style::psql())
        .with(Modify::new(Rows::new(1..)).with(Alignment::left()));

    table.to_string()
}

#[derive(Tabled)]
struct QueueRow {
    #[tabled(rename = "Position")]
    position: String,
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Build Type")]
    build_type: String,
    #[tabled(rename = "Branch")]
    branch: String,
    #[tabled(rename = "Queued")]
    queued: String,
}

pub fn format_queue_table(builds: &Builds) -> String {
    let rows: Vec<QueueRow> = builds
        .build
        .as_ref()
        .map(|b| {
            b.iter()
                .enumerate()
                .map(|(idx, build)| QueueRow {
                    position: (idx + 1).to_string(),
                    id: build.id.map_or("N/A".to_string(), |id| id.to_string()),
                    build_type: build
                        .build_type_id
                        .as_ref()
                        .map_or("N/A".to_string(), |bt| bt.to_string()),
                    branch: build
                        .branch_name
                        .as_ref()
                        .map_or("default".to_string(), |b| b.to_string()),
                    queued: build
                        .queued_date
                        .as_ref()
                        .map_or("N/A".to_string(), |d| d.to_string()),
                })
                .collect()
        })
        .unwrap_or_default();

    if rows.is_empty() {
        return "Build queue is empty".to_string();
    }

    let mut table = Table::new(rows);
    table
        .with(Style::psql())
        .with(Modify::new(Rows::new(1..)).with(Alignment::left()));

    table.to_string()
}

pub fn format_queued_build_details(build: &Build) -> String {
    let mut details = Vec::new();

    details.push(format!(
        "{}: {}",
        "ID".bold(),
        build.id.map_or("N/A".to_string(), |id| id.to_string())
    ));
    details.push(format!(
        "{}: {}",
        "Build Type".bold(),
        build
            .build_type_id
            .as_ref()
            .map_or("N/A".to_string(), |bt| bt.to_string())
    ));
    details.push(format!(
        "{}: {}",
        "Branch".bold(),
        build
            .branch_name
            .as_ref()
            .map_or("default".to_string(), |b| b.to_string())
    ));

    if let Some(queued) = &build.queued_date {
        details.push(format!("{}: {}", "Queued".bold(), queued));
    }

    if let Some(web_url) = &build.web_url {
        details.push(format!("{}: {}", "Web URL".bold(), web_url));
    }

    if let Some(comment) = &build.comment {
        if let Some(text) = &comment.text {
            details.push(format!("{}: {}", "Comment".bold(), text));
        }
    }

    details.join("\n")
}

pub fn format_projects_tree(projects: &Projects) -> String {
    let project_list = match &projects.project {
        Some(p) if !p.is_empty() => p,
        _ => return "No projects found".to_string(),
    };

    let mut result = String::new();
    let mut sorted: Vec<_> = project_list.iter().collect();
    sorted.sort_by(|a, b| {
        a.name
            .as_ref()
            .unwrap_or(&String::new())
            .to_lowercase()
            .cmp(&b.name.as_ref().unwrap_or(&String::new()).to_lowercase())
    });

    let parent_ids: std::collections::HashSet<_> = project_list
        .iter()
        .filter_map(|p| p.id.as_ref())
        .cloned()
        .collect();

    let root_projects: Vec<_> = sorted
        .iter()
        .filter(|p| {
            p.parent_project_id
                .as_ref()
                .map(|id| id == "_Root" || !parent_ids.contains(id))
                .unwrap_or(true)
        })
        .collect();

    for (i, project) in root_projects.iter().enumerate() {
        let is_last = i == root_projects.len() - 1;
        format_project_tree_recursive(project, &sorted, "", is_last, &mut result, &parent_ids);
    }

    result.trim_end().to_string()
}

fn format_project_tree_recursive(
    project: &api::models::Project,
    all_projects: &[&api::models::Project],
    prefix: &str,
    is_last: bool,
    result: &mut String,
    parent_ids: &std::collections::HashSet<String>,
) {
    let connector = if is_last { "└── " } else { "├── " };
    let name = project.name.as_deref().unwrap_or("N/A");
    let id = project.id.as_deref().unwrap_or("N/A");
    let archived_marker = if project.archived.unwrap_or(false) {
        " [archived]"
    } else {
        ""
    };

    result.push_str(&format!(
        "{}{}{}{} ({})\n",
        prefix,
        connector,
        name,
        archived_marker,
        id.dimmed()
    ));

    let project_id = match &project.id {
        Some(id) => id,
        None => return,
    };

    let mut children: Vec<_> = all_projects
        .iter()
        .filter(|p| p.parent_project_id.as_ref() == Some(project_id))
        .collect();

    children.sort_by(|a, b| {
        a.name
            .as_ref()
            .unwrap_or(&String::new())
            .to_lowercase()
            .cmp(&b.name.as_ref().unwrap_or(&String::new()).to_lowercase())
    });

    let child_prefix = if is_last {
        format!("{}    ", prefix)
    } else {
        format!("{}│   ", prefix)
    };

    for (i, child) in children.iter().enumerate() {
        let child_is_last = i == children.len() - 1;
        format_project_tree_recursive(
            child,
            all_projects,
            &child_prefix,
            child_is_last,
            result,
            parent_ids,
        );
    }
}

pub fn format_project_details(project: &Project) -> String {
    let mut details = Vec::new();

    details.push(format!(
        "{}: {}",
        "ID".bold(),
        project.id.clone().unwrap_or_else(|| "N/A".to_string())
    ));
    details.push(format!(
        "{}: {}",
        "Name".bold(),
        project.name.clone().unwrap_or_else(|| "N/A".to_string())
    ));

    if let Some(description) = &project.description {
        details.push(format!("{}: {}", "Description".bold(), description));
    }

    if let Some(parent_id) = &project.parent_project_id {
        details.push(format!("{}: {}", "Parent Project ID".bold(), parent_id));
    }

    if let Some(parent_name) = &project.parent_project_name {
        details.push(format!("{}: {}", "Parent Project Name".bold(), parent_name));
    }

    if let Some(archived) = project.archived {
        details.push(format!("{}: {}", "Archived".bold(), archived));
    }

    if let Some(web_url) = &project.web_url {
        details.push(format!("{}: {}", "Web URL".bold(), web_url));
    }

    if let Some(build_types) = &project.build_types {
        if let Some(count) = build_types.count {
            details.push(format!("{}: {}", "Build Types Count".bold(), count));
        }
    }

    if let Some(subprojects) = &project.projects {
        if let Some(count) = subprojects.count {
            details.push(format!("{}: {}", "Subprojects Count".bold(), count));
        }
    }

    details.join("\n")
}

#[derive(Tabled)]
struct BuildTypeRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Project")]
    project: String,
    #[tabled(rename = "Paused")]
    paused: String,
}

pub fn format_build_types_table(build_types: &BuildTypes) -> String {
    let rows: Vec<BuildTypeRow> = build_types
        .build_type
        .as_ref()
        .map(|bt| {
            bt.iter()
                .map(|build_type| BuildTypeRow {
                    id: build_type.id.clone().unwrap_or_else(|| "N/A".to_string()),
                    name: build_type.name.clone().unwrap_or_else(|| "N/A".to_string()),
                    project: build_type
                        .project_name
                        .clone()
                        .unwrap_or_else(|| "N/A".to_string()),
                    paused: build_type
                        .paused
                        .map(|p| {
                            if p {
                                "Yes".to_string()
                            } else {
                                "No".to_string()
                            }
                        })
                        .unwrap_or_else(|| "No".to_string()),
                })
                .collect()
        })
        .unwrap_or_default();

    if rows.is_empty() {
        return "No build types found".to_string();
    }

    let mut table = Table::new(rows);
    table
        .with(Style::psql())
        .with(Modify::new(Rows::new(1..)).with(Alignment::left()));

    table.to_string()
}

pub fn format_build_type_details(build_type: &BuildType) -> String {
    let mut details = Vec::new();

    details.push(format!(
        "{}: {}",
        "ID".bold(),
        build_type.id.clone().unwrap_or_else(|| "N/A".to_string())
    ));
    details.push(format!(
        "{}: {}",
        "Name".bold(),
        build_type.name.clone().unwrap_or_else(|| "N/A".to_string())
    ));

    if let Some(description) = &build_type.description {
        details.push(format!("{}: {}", "Description".bold(), description));
    }

    if let Some(project_id) = &build_type.project_id {
        details.push(format!("{}: {}", "Project ID".bold(), project_id));
    }

    if let Some(project_name) = &build_type.project_name {
        details.push(format!("{}: {}", "Project Name".bold(), project_name));
    }

    if let Some(paused) = build_type.paused {
        details.push(format!("{}: {}", "Paused".bold(), paused));
    }

    if let Some(template_flag) = build_type.template_flag {
        details.push(format!("{}: {}", "Template".bold(), template_flag));
    }

    if let Some(web_url) = &build_type.web_url {
        details.push(format!("{}: {}", "Web URL".bold(), web_url));
    }

    details.join("\n")
}

#[derive(Tabled)]
struct PropertyRow {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Value")]
    value: String,
}

pub fn format_properties_table(properties: &Properties) -> String {
    let rows: Vec<PropertyRow> = properties
        .property
        .as_ref()
        .map(|p| {
            p.iter()
                .map(|prop| PropertyRow {
                    name: prop.name.clone().unwrap_or_else(|| "N/A".to_string()),
                    value: prop.value.clone().unwrap_or_else(|| "N/A".to_string()),
                })
                .collect()
        })
        .unwrap_or_default();

    if rows.is_empty() {
        return "No parameters found".to_string();
    }

    let mut table = Table::new(rows);
    table
        .with(Style::psql())
        .with(Modify::new(Rows::new(1..)).with(Alignment::left()));

    table.to_string()
}
