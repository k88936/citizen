# Citizen - TeamCity CLI

A command-line interface for TeamCity CI/CD server.

## Installation

```bash
cargo install --path .
```

## Configuration

Create a `citizen.toml` file in your current directory (higher priority) or home directory:

```toml
server_url="https://teamcity.jetbrains.com"
token="generate a token for your user in My Settings & Tools | Access Tokens in TeamCity."
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
### Regenerate API Client

The API client is generated from TeamCity's OpenAPI spec:

```bash
java -Xmx32G -jar ~/.m2/repository/org/openapitools/openapi-generator-cli/7.21.0-SNAPSHOT/openapi-generator-cli-7.21.0-SNAPSHOT.jar generate -g rust -i https://teamcity.jetbrains.com/app/rest/swagger.json -o ./api --skip-validate-spec --additional-properties=packageName=api
```
