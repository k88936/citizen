use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(name = "citizen")]
#[command(about = "A CLI tool for TeamCity", long_about = None)]
#[command(version)]
pub struct Cli {
    #[arg(short, long, env = "TEAMCITY_PROFILE")]
    pub profile: Option<String>,

    #[arg(short = 'u', long, env = "TEAMCITY_URL")]
    pub server: Option<String>,

    #[arg(short, long, env = "TEAMCITY_TOKEN")]
    pub token: Option<String>,

    #[arg(short, long, value_enum, default_value = "human")]
    pub output: OutputFormat,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Human,
    Json,
}

#[derive(Subcommand)]
pub enum Commands {
    Build {
        #[command(subcommand)]
        command: BuildCommands,
    },
    Project {
        #[command(subcommand)]
        command: ProjectCommands,
    },
    Buildtype {
        #[command(subcommand)]
        command: BuildTypeCommands,
    },
    Server {
        #[command(subcommand)]
        command: ServerCommands,
    },
    Queue {
        #[command(subcommand)]
        command: QueueCommands,
    },
    Agent {
        #[command(subcommand)]
        command: AgentCommands,
    },
    Interactive,
    Completion {
        shell: Shell,
    },
}

#[derive(Subcommand)]
pub enum BuildCommands {
    List {
        #[arg(short, long)]
        build_type: Option<String>,

        #[arg(short, long)]
        status: Option<String>,

        #[arg(short = 'B', long)]
        branch: Option<String>,

        #[arg(short, long, default_value = "100")]
        limit: u32,

        #[arg(long)]
        running: bool,

        #[arg(long)]
        queued: bool,
    },

    Get {
        #[arg(required = true)]
        build_id: String,
    },

    Trigger {
        #[arg(short, long)]
        build_type: Option<String>,

        #[arg(short = 'B', long)]
        branch: Option<String>,

        #[arg(short, long)]
        comment: Option<String>,

        #[arg(short = 'D', long)]
        parameter: Vec<String>,

        #[arg(long)]
        wait: bool,

        #[arg(long)]
        watch: bool,

        #[arg(short, long)]
        interactive: bool,
    },

    Cancel {
        #[arg(required = true)]
        build_id: String,

        #[arg(short, long)]
        comment: Option<String>,
    },

    Status {
        #[arg(required = true)]
        build_id: String,
    },

    Log {
        #[arg(required = true)]
        build_id: String,

        #[arg(long)]
        follow: bool,

        #[arg(long, default_value = "100")]
        tail: u32,

        #[arg(long)]
        download: bool,
    },

    Artifacts {
        #[arg(required = true)]
        build_id: String,

        #[command(subcommand)]
        command: ArtifactCommands,
    },

    Tags {
        #[arg(required = true)]
        build_id: String,

        #[command(subcommand)]
        command: TagCommands,
    },

    Pin {
        #[arg(required = true)]
        build_id: String,

        #[arg(long)]
        unpin: bool,

        #[arg(short, long)]
        comment: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ArtifactCommands {
    List {
        #[arg(short, long)]
        path: Option<String>,
    },

    Download {
        #[arg(required = true)]
        path: String,

        #[arg(short, long, default_value = ".")]
        output: String,
    },
}

#[derive(Subcommand)]
pub enum TagCommands {
    List,

    Add {
        #[arg(required = true)]
        tag: String,
    },

    Remove {
        #[arg(required = true)]
        tag: String,
    },
}

#[derive(Subcommand)]
pub enum QueueCommands {
    List {
        #[arg(short, long)]
        build_type: Option<String>,

        #[arg(short, long, default_value = "100")]
        limit: u32,
    },

    Get {
        #[arg(required = true)]
        build_id: String,
    },

    Cancel {
        #[arg(required = true)]
        build_id: String,

        #[arg(short, long)]
        comment: Option<String>,
    },

    Reorder {
        #[arg(required = true)]
        build_id: String,

        #[arg(short, long)]
        position: Option<i32>,

        #[arg(long)]
        top: bool,
    },
}

#[derive(Subcommand)]
pub enum ServerCommands {
    Info,
}

#[derive(Subcommand)]
pub enum ProjectCommands {
    List {
        #[arg(short, long)]
        parent: Option<String>,
    },

    Get {
        #[arg(required = true)]
        project_id: String,
    },

    BuildTypes {
        #[arg(required = true)]
        project_id: String,
    },
}

#[derive(Subcommand)]
pub enum BuildTypeCommands {
    List {
        #[arg(short, long)]
        project_id: Option<String>,
    },

    Get {
        #[arg(required = true)]
        build_type_id: String,
    },

    Parameters {
        #[arg(required = true)]
        build_type_id: String,
    },
}
#[derive(Subcommand)]
pub enum AgentCommands {
    List {
        #[arg(long)]
        connected: Option<bool>,

        #[arg(long)]
        authorized: Option<bool>,

        #[arg(long)]
        enabled: Option<bool>,
    },

    Get {
        #[arg(required = true)]
        agent_id: String,
    },

    Enable {
        #[arg(required = true)]
        agent_id: String,
    },

    Disable {
        #[arg(required = true)]
        agent_id: String,

        #[arg(short, long)]
        comment: Option<String>,
    },

    Authorize {
        #[arg(required = true)]
        agent_id: String,
    },

    Unauthorize {
        #[arg(required = true)]
        agent_id: String,
    },

    Pool {
        #[command(subcommand)]
        command: AgentPoolCommands,
    },
}

#[derive(Subcommand)]
pub enum AgentPoolCommands {
    List,

    Get {
        #[arg(required = true)]
        pool_id: String,
    },
}
