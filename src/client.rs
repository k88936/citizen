use anyhow::Result;
use api::apis::configuration::Configuration;

pub struct TeamCityClient {
    pub config: Configuration,
}

impl TeamCityClient {
    pub fn new(server_url: &str, token: &str) -> Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", token).parse()?,
        );
        headers.insert(reqwest::header::ACCEPT, "application/json".parse()?);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let mut config = Configuration::new();
        config.base_path = server_url.trim_end_matches('/').to_string();
        config.client = client;

        Ok(TeamCityClient { config })
    }
}
