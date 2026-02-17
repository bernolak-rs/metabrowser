use crate::{SearchEngine, SearchResult};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Scholar {
    #[serde(default)]
    data: Vec<ScholarEntry>,
}

#[derive(Debug, Deserialize)]
struct ScholarEntry {
    title: String,
    url: Option<String>,
    #[serde(rename = "description")]
    description: Option<String>,
    #[serde(rename = "citationCount")]
    citation_count: Option<i32>,
}

pub struct ScholarClient {
    client: Client,
}
impl ScholarClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Default for ScholarClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SearchEngine for ScholarClient {
    fn name(&self) -> &'static str {
        "Semantic scholar"
    }

    async fn search(&self, query: &str) -> anyhow::Result<Vec<SearchResult>> {
        let url = format!(
            "https://api.semanticscholar.org/graph/v1/paper/search?query={}&limit=10&fields=title,url,abstract,citationCount",
            query
        );

        let response = self
            .client
            .get(url)
            .header("User-Agent", "Metabrowser")
            .send()
            .await?
            .json::<Scholar>()
            .await?;

        let results = response
            .data
            .into_iter()
            .map(|entry| SearchResult {
                title: entry.title,
                url: entry.url.unwrap_or_default(),
                snippet: entry
                    .description
                    .unwrap_or_else(|| "No abstract available".to_string())
                    .chars()
                    .take(200)
                    .collect::<String>()
                    + "...",
                source: "Semantic Scholar".to_string(),
                score: 0.8 + (entry.citation_count.unwrap_or(0) as f32 / 4.0).max(15.0),
            })
            .collect();

        Ok(results)
    }
}
