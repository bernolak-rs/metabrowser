use async_trait::async_trait;
use reqwest::{self, Client};
use serde::Deserialize;

use crate::{SearchEngine, SearchResult};

#[derive(Debug, Deserialize)]
struct Arxiv {
    #[serde(rename = "entry", default)]
    entries: Vec<ArxivEntry>,
}

#[derive(Debug, Deserialize)]
struct ArxivEntry {
    title: String,
    #[serde(rename = "summary")]
    description: String,
    id: String,
}

pub struct ArxivClient {
    client: Client,
}

impl ArxivClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl Default for ArxivClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SearchEngine for ArxivClient {
    fn name(&self) -> &'static str {
        "Arxiv"
    }

    async fn search(&self, query: &str) -> anyhow::Result<Vec<SearchResult>> {
        let url = format!(
            "https://export.arxiv.org/api/query?search_query=all:{}&max_results=10",
            query
        );

        let response = self.client.get(url).send().await?.text().await?;

        let feed: Arxiv = serde_xml_rs::from_str(&response)?;

        let results = feed
            .entries
            .into_iter()
            .map(|entry| SearchResult {
                title: entry.title.replace('\n', " ").trim().to_string(),
                url: entry.id,
                snippet: entry.description.chars().take(200).collect::<String>() + "...",
                source: "ArXiv".to_string(),
                score: 0.9,
            })
            .collect();

        Ok(results)
    }
}
