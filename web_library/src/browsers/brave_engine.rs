use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

use crate::SearchResult;
use crate::browsers::Config;
use crate::browsers::SearchEngine;

#[derive(Debug, Deserialize)]
struct BraveResponse {
    web: Option<BraveWeb>,
}

#[derive(Debug, Deserialize)]
struct BraveWeb {
    results: Vec<BraveResult>,
}

#[derive(Debug, Deserialize)]
struct BraveResult {
    title: String,
    url: String,
    description: Option<String>,
}

pub struct BraveSearchEngine {
    client: Client,
    api_key: String,
}

impl BraveSearchEngine {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            api_key: config.brave_api_key.clone(),
        }
    }
}

#[async_trait]
impl SearchEngine for BraveSearchEngine {
    fn name(&self) -> &'static str {
        "brave"
    }
    async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        let response: BraveResponse = self
            .client
            .get("https://api.search.brave.com/res/v1/web/search")
            .header("Accept", "application/json")
            .header("X-Subscription-Token", &self.api_key)
            .query(&[("q", query), ("count", "10")])
            .send()
            .await
            .context("Failed to send request to Brave Search API")?
            .error_for_status()
            .context("Brave Search API returned an error status")?
            .json()
            .await
            .context("Failed to deserialize Brave Search response")?;

        let results = response
            .web
            .map(|web| web.results)
            .unwrap_or_default()
            .into_iter()
            .map(|item| SearchResult {
                title: item.title,
                url: item.url,
                snippet: item.description.unwrap_or_default(),
                score: 1.0,
                source: self.name().into(),
            })
            .collect();
        Ok(results)
    }
}
