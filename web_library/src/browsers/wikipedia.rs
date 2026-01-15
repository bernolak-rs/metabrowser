/// # Wikipedia module
///
/// Implements SearchEngine trait.
/// This module provides WikipediaClient that searches wikipedia encyclopedia.
/// It provides summaries for searched topics.
use crate::SearchResult;
use crate::browsers::SearchEngine;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSummary {
    pub title: String,
    pub extract: String,
    pub thumbnail: Option<WikiThumbnail>,
    pub content_urls: WikiContentUrls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiThumbnail {
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiContentUrls {
    pub desktop: WikiUrls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiUrls {
    pub page: String,
}

pub struct WikipediaClient {
    client: Client,
}

impl WikipediaClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl Default for WikipediaClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SearchEngine for WikipediaClient {
    fn name(&self) -> &'static str {
        "Wikipedia"
    }

    async fn search(&self, query: &str) -> anyhow::Result<Vec<SearchResult>> {
        let formatted_query = query
            .split_whitespace()
            .map(|w| {
                let mut chars = w.chars();
                match chars.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join("_");

        let url = format!(
            "https://en.wikipedia.org/api/rest_v1/page/summary/{}",
            urlencoding::encode(&formatted_query)
        );

        let response = self
            .client
            .get(url)
            .header("User-Agent", "Metabrowser/1.0")
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(vec![]);
        }

        let data: WikiSummary = response.json().await?;

        Ok(vec![SearchResult {
            title: data.title,
            url: data.content_urls.desktop.page,
            snippet: data.extract,
            source: "Wikipedia".to_string(),
            score: 10.0,
        }])
    }
}
