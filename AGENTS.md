# AGENTS.md

Coding agent instructions for the Citizen (TeamCity CLI) project.

## Project Overview

Citizen is a CLI tool for TeamCity CI/CD servers, written in Rust. The project consists of:
- Main crate (`citizen`): CLI application in `src/`
- `api` crate: Auto-generated OpenAPI client (DO NOT manually edit)

## Build/Lint/Test Commands

```bash
cargo build                    # Build the project
cargo test                     # Run all tests
cargo test test_name           # Run a single test by name
cargo test module_name::test_name  # Run a test in a specific module
cargo fmt                      # Format code
```

## Project-Specific Notes

### API Crate

The `api/` directory contains auto-generated code from TeamCity's OpenAPI spec. 
- DO NOT manually edit files in `api/src/`
- The crate uses special clippy allowances in `api/src/lib.rs`