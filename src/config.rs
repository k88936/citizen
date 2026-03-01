use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub default: Profile,
    #[serde(default)]
    pub profile: HashMap<String, Profile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Profile {
    pub server_url: Option<String>,
    pub token: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        const CONFIG_FILE: &str = "citizen.toml";

        let paths: Vec<PathBuf> = [
            env::current_dir().ok().map(|d| d.join(CONFIG_FILE)),
            env::var("HOME")
                .ok()
                .map(|h| PathBuf::from(h).join(CONFIG_FILE)),
            env::var("USERPROFILE")
                .ok()
                .map(|h| PathBuf::from(h).join(CONFIG_FILE)),
        ]
        .into_iter()
        .flatten()
        .collect();

        for path in paths {
            if path.exists() {
                let content = std::fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read config file: {:?}", path))?;
                let config: Config =
                    toml::from_str(&content).with_context(|| "Failed to parse config file")?;
                return Ok(config);
            }
        }

        Ok(Config::default())
    }

    pub fn resolve(
        &self,
        cli_server: Option<&str>,
        cli_token: Option<&str>,
        profile_name: Option<&str>,
    ) -> (String, String) {
        let server = cli_server
            .map(|s| s.to_string())
            .or_else(|| env::var("TEAMCITY_URL").ok())
            .or_else(|| {
                profile_name
                    .and_then(|name| self.profile.get(name))
                    .and_then(|p| p.server_url.clone())
            })
            .or_else(|| self.default.server_url.clone())
            .expect(
                "Server URL must be provided via --server, TEAMCITY_URL env var, or config file",
            );

        let token = cli_token
            .map(|s| s.to_string())
            .or_else(|| env::var("TEAMCITY_TOKEN").ok())
            .or_else(|| {
                profile_name
                    .and_then(|name| self.profile.get(name))
                    .and_then(|p| p.token.clone())
            })
            .or_else(|| self.default.token.clone())
            .expect(
                "API token must be provided via --token, TEAMCITY_TOKEN env var, or config file",
            );

        (server, token)
    }
}
