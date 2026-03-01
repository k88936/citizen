use api::models::{Build, Builds, Files, Server, Tags};
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
