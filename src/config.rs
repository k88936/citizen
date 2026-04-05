use anyhow::{Context, Result};
use inquire::{Password, Text};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
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

    pub fn resolve(&self, cli_server: Option<&str>, cli_token: Option<&str>) -> (String, String) {
        let server = cli_server
            .map(|s| s.to_string())
            .or_else(|| env::var("TEAMCITY_URL").ok())
            .or_else(|| self.server_url.clone())
            .expect(
                "Server URL must be provided via --server, TEAMCITY_URL env var, or config file",
            );

        let token = cli_token
            .map(|s| s.to_string())
            .or_else(|| env::var("TEAMCITY_TOKEN").ok())
            .or_else(|| self.token.clone())
            .expect(
                "API token must be provided via --token, TEAMCITY_TOKEN env var, or config file",
            );

        (server, token)
    }

    pub fn run_setup_wizard() -> Result<PathBuf> {
        let existing = Self::load().unwrap_or_default();

        let default_server = existing
            .server_url
            .or_else(|| env::var("TEAMCITY_URL").ok())
            .unwrap_or_else(|| "https://teamcity.jetbrains.com".to_string());

        let server_url = Text::new("TeamCity server URL:")
            .with_default(&default_server)
            .prompt()?;

        let token = Password::new("TeamCity token:")
            .without_confirmation()
            .prompt()?;

        let config = Config {
            server_url: Some(server_url),
            token: Some(token),
        };

        config.save_to_home()
    }

    pub fn save_to_home(&self) -> Result<PathBuf> {
        let path = Self::home_config_path()?;
        let content = toml::to_string(self).with_context(|| "Failed to serialize config")?;
        std::fs::write(&path, content)
            .with_context(|| format!("Failed to write config file: {:?}", path))?;
        Ok(path)
    }

    fn home_config_path() -> Result<PathBuf> {
        const CONFIG_FILE: &str = "citizen.toml";

        if let Ok(home) = env::var("HOME") {
            return Ok(PathBuf::from(home).join(CONFIG_FILE));
        }

        if let Ok(profile) = env::var("USERPROFILE") {
            return Ok(PathBuf::from(profile).join(CONFIG_FILE));
        }

        anyhow::bail!("Could not determine home directory from HOME or USERPROFILE");
    }
}
