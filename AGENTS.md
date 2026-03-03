# AGENTS.md

Coding agent instructions for the Citizen (TeamCity CLI) project.

## Project Overview

Citizen is a CLI tool for TeamCity CI/CD servers, written in Rust. The project consists of:
- Main crate (`citizen`): CLI application in `src/`
- `api` crate: Auto-generated OpenAPI client (DO NOT manually edit)

## Build/Lint/Test Commands

```bash
cargo build                    # Build the project
cargo build --release          # Build for release
cargo test                     # Run all tests
cargo test test_name           # Run a single test by name
cargo test module_name::test_name  # Run a test in a specific module
cargo fmt                      # Format code (always run before committing)
cargo fmt -- --check           # Check formatting without modifying
cargo clippy                   # Run linter
cargo clippy -- -D warnings    # Run linter with warnings as errors
```

## Project Structure

```
src/
  main.rs           # Entry point, command routing
  cli.rs            # CLI definitions (clap derive macros)
  client.rs         # TeamCity API client wrapper
  config.rs         # Configuration file handling
  interactive.rs    # Interactive mode implementation
  commands/
    mod.rs          # Command handlers re-exports
    build.rs        # Build-related commands
    project.rs      # Project and build type commands
    queue.rs        # Build queue commands
    server.rs       # Server info commands
    agent.rs        # Agent commands
  output/
    mod.rs          # Output formatter re-exports
    human.rs        # Human-readable output formatters

api/                # Auto-generated OpenAPI client
  src/
    lib.rs          # API crate entry (clippy allowances)
    apis/           # API endpoints (auto-generated)
    models/         # Data models (auto-generated)
```

### Error Handling

Use `anyhow::Result` for all fallible functions. Add context with `.context()`:

```rust
pub async fn fetch_build(client: &TeamCityClient, build_id: &str) -> Result<()> {
    let build = build_api::get_build(&client.config, build_id, None)
        .await
        .context("Failed to fetch build")?;
    Ok(())
}
```

Use `anyhow::bail!` for early returns with errors:

```rust
if build_type.is_none() {
    anyhow::bail!("--build-type is required when not using --interactive mode");
}
```

### Naming Conventions

- **Modules**: `snake_case` (e.g., `build_type`, `build_queue`)
- **Types/Structs**: `PascalCase` (e.g., `BuildListArgs`, `TeamCityClient`)
- **Functions**: `snake_case` (e.g., `handle_build_list`, `format_builds_table`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `CONFIG_FILE`)
- **CLI enums**: `PascalCase` (e.g., `BuildCommands`, `OutputFormat`)

### Command Handler Pattern

Each command module follows this pattern:

```rust
// 1. Define args structs for each subcommand
pub struct BuildListArgs {
    pub build_type: Option<String>,
    pub limit: u32,
}

// 2. Main handler that dispatches to sub-handlers
pub async fn handle_build_command(
    client: &TeamCityClient,
    command: crate::cli::BuildCommands,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    match command {
        crate::cli::BuildCommands::List { build_type, limit } => {
            handle_build_list(client, BuildListArgs { build_type, limit }, output_format).await
        }
        // ...
    }
}

// 3. Private sub-handlers
async fn handle_build_list(
    client: &TeamCityClient,
    args: BuildListArgs,
    output_format: crate::cli::OutputFormat,
) -> Result<()> {
    // Implementation
}
```

### Output Formatting

Always support both `Human` and `Json` output formats:

```rust
match output_format {
    crate::cli::OutputFormat::Human => {
        println!("{}", output::format_builds_table(&builds));
    }
    crate::cli::OutputFormat::Json => {
        println!("{}", serde_json::to_string_pretty(&builds)?);
    }
}
```

Place human-readable formatters in `src/output/human.rs` and re-export from `src/output/mod.rs`.

### CLI Argument Definitions

Use clap derive macros in `src/cli.rs`:

```rust
#[derive(Parser)]
#[command(name = "citizen")]
pub struct Cli {
    #[arg(short, long, env = "TEAMCITY_URL")]
    pub server: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Build {
        #[command(subcommand)]
        command: BuildCommands,
    },
}
```

## API Crate Guidelines

The `api/` directory contains auto-generated code from TeamCity's OpenAPI spec:
- **NEVER manually edit files in `api/src/`**
- The crate uses clippy allowances in `api/src/lib.rs`
- Use APIs from `api::apis::*` and models from `api::models::*`
