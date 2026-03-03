# Citizen - TeamCity CLI

A command-line interface for TeamCity CI/CD server.

## Installation

```bash
cargo install --path .
```

## Configuration

Create a `citizen.toml` file in your current directory (higher priority) or home directory:

```toml
[default]
server_url="https://teamcity.jetbrains.com"
token="generate a token for your user in My Settings & Tools | Access Tokens in TeamCity."

[profile.work]
server_url = "https://teamcity.work.example.com"
token = "your-work-api-token"
```

Alternatively, use environment variables:

```bash
export TEAMCITY_URL="https://teamcity.example.com"
export TEAMCITY_TOKEN="your-api-token"
```

## Usage

```bash
citizen [OPTIONS] <COMMAND>
```

### Options

| Option | Description |
|--------|-------------|
| `-p, --profile <PROFILE>` | Configuration profile name |
| `-u, --server <SERVER>` | TeamCity server URL |
| `-t, --token <TOKEN>` | API token |
| `-o, --output <OUTPUT>` | Output format: `human` (default) or `json` |

### Interactive Mode

```bash
citizen interactive
# or
citizen i
```

Launch an interactive TUI for browsing projects, build types, and triggering builds.

## Commands

### Build Commands

```bash
# List builds
citizen build list [--build-type <ID>] [--status <STATUS>] [--branch <BRANCH>] [--running] [--queued] [--limit <N>]

# Get build details
citizen build get <BUILD_ID>

# Trigger a build
citizen build trigger --build-type <ID> [--branch <BRANCH>] [--parameter KEY=VALUE]... [--wait] [--watch] [--interactive]

# Cancel a build
citizen build cancel <BUILD_ID> [--comment <TEXT>]

# Get build status
citizen build status <BUILD_ID>

# View build log
citizen build log <BUILD_ID> [--follow] [--tail <N>] [--download]

# Manage build artifacts
citizen build artifacts <BUILD_ID> list [--path <PATH>]

# Manage build tags
citizen build tags <BUILD_ID> list
citizen build tags <BUILD_ID> add <TAG>
citizen build tags <BUILD_ID> remove <TAG>

# Pin/unpin a build
citizen build pin <BUILD_ID> [--unpin] [--comment <TEXT>]
```

### Project Commands

```bash
# List projects
citizen project list [--parent <PROJECT_ID>]

# Get project details
citizen project get <PROJECT_ID>

# List build types in a project
citizen project build-types <PROJECT_ID>
```

### Build Type Commands

```bash
# List build types
citizen buildtype list [--project-id <ID>]

# Get build type details
citizen buildtype get <BUILD_TYPE_ID>

# Get build type parameters
citizen buildtype parameters <BUILD_TYPE_ID>
```

### Queue Commands

```bash
# List queued builds
citizen queue list [--build-type <ID>] [--limit <N>]

# Get queued build details
citizen queue get <BUILD_ID>

# Cancel a queued build
citizen queue cancel <BUILD_ID> [--comment <TEXT>]

# Reorder a queued build
citizen queue reorder <BUILD_ID> [--position <N>] [--top]
```

### Server Commands

```bash
# Get server information
citizen server info
```

### Agent Commands

```bash
# List agents
citizen agent list [--connected] [--authorized] [--enabled]

# Get agent details
citizen agent get <AGENT_ID>

# Enable/disable agents
citizen agent enable <AGENT_ID>
citizen agent disable <AGENT_ID> [--comment <TEXT>]

# Authorize/unauthorize agents
citizen agent authorize <AGENT_ID>
citizen agent unauthorize <AGENT_ID>

# Agent pool commands
citizen agent pool list
citizen agent pool get <POOL_ID>
```

### Shell Completion

```bash
# Generate shell completion script
citizen completion bash > ~/.config/bash-completion/completions/citizen
citizen completion zsh > ~/.zsh/completion/_citizen
citizen completion fish > ~/.config/fish/completions/citizen.fish
```

## Examples

### Trigger a build and wait for completion

```bash
citizen build trigger --build-type MyProject_MyBuildConfig --branch feature-branch --wait
```

### Watch a running build

```bash
citizen build log 12345 --follow
```

### List running builds

```bash
citizen build list --running
```

### Download build artifacts

```bash
citizen build artifacts 12345 list
```

### JSON output for scripting

```bash
citizen --output json build list --build-type MyBuildConfig --limit 10 | jq '.build[] | {id, status, number}'
```

## Development

### Build

```bash
cargo build
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Lint

```bash
cargo fmt
cargo clippy
```

### Regenerate API Client

The API client is generated from TeamCity's OpenAPI spec:

```bash
java -Xmx32G -jar ~/.m2/repository/org/openapitools/openapi-generator-cli/7.21.0-SNAPSHOT/openapi-generator-cli-7.21.0-SNAPSHOT.jar generate -g rust -i https://teamcity.jetbrains.com/app/rest/swagger.json -o ./api --skip-validate-spec --additional-properties=packageName=api
```