//! # DuckDuckGo module
//!
//! Implements SearchEngine trait.
//! This module provides querying for DDG engine.

use reqwest::Client;
use serde::Deserialize;

use crate::browsers::SearchEngine;
use crate::search_result::SearchResult;

pub struct DuckDuckGo {
    client: Client,
}

impl DuckDuckGo {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl Default for DuckDuckGo {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DuckDuckGoResponse {
    heading: Option<String>,
    abstract_text: Option<String>,
    abstract_url: Option<String>,
    related_topics: Option<Vec<RelatedTopic>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RelatedTopic {
    Topic {
        #[serde(rename = "Text")]
        text: String,
        #[serde(rename = "FirstURL")]
        first_url: String,
    },
    Category {
        #[serde(rename = "Topics")]
        topics: Vec<RelatedTopic>,
    },
}

#[async_trait::async_trait]
impl SearchEngine for DuckDuckGo {
    fn name(&self) -> &'static str {
        "DuckDuckGo"
    }

    async fn search(&self, query: &str) -> anyhow::Result<Vec<SearchResult>> {
        let response: DuckDuckGoResponse = self
            .client
            .get("https://api.duckduckgo.com/")
            .query(&[
                ("q", query),
                ("format", "json"),
                ("no_redirect", "1"),
                ("no_html", "1"),
            ])
            .send()
            .await?
            .json()
            .await?;

        let mut results = Vec::new();

        if let (Some(title), Some(text), Some(url)) = (
            response.heading,
            response.abstract_text,
            response.abstract_url,
        ) {
            results.push(SearchResult {
                title,
                snippet: text,
                url,
                source: self.name().into(),
                score: 2.0,
            });
        }

        if let Some(topics) = response.related_topics {
            Self::extract_topics(topics, &mut results);
        }

        Ok(results)
        // Ok(vec![
        //     SearchResult {
        //         title: format!("Search results for {} - DuckDuckGo", query),
        //         url: "https://duckduckgo.com".to_string(),
        //         snippet: "This is a mock result to prove the aggregator works!".to_string(),
        //         source: "DuckDuckGo".to_string(),
        //         score: 1.0,
        //     },
        //     SearchResult {
        //         title: "Rust Programming Language".to_string(),
        //         url: "https://www.rust-lang.org".to_string(),
        //         snippet: "A language empowering everyone to build reliable and efficient software."
        //             .to_string(),
        //         source: "DuckDuckGo".to_string(),
        //         score: 0.9,
        //     },
        // ])
    }
}

impl DuckDuckGo {
    fn extract_topics(topics: Vec<RelatedTopic>, results: &mut Vec<SearchResult>) {
        for topic in topics {
            match topic {
                RelatedTopic::Topic { text, first_url } => {
                    results.push(SearchResult {
                        title: text.clone(),
                        snippet: text,
                        url: first_url,
                        source: "DuckDuckGo".into(),
                        score: 1.0,
                    });
                }
                RelatedTopic::Category { topics } => {
                    Self::extract_topics(topics, results);
                }
            }
        }
    }
}
