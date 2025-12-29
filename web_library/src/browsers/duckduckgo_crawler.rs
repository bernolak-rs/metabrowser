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

#[derive(Debug, Deserialize)]
struct DuckDuckGoResponse {
    Heading: Option<String>,
    AbstractText: Option<String>,
    AbstractURL: Option<String>,
    RelatedTopics: Option<Vec<RelatedTopic>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum RelatedTopic {
    Topic { Text: String, FirstURL: String },
    Category { Topics: Vec<RelatedTopic> },
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
            response.Heading,
            response.AbstractText,
            response.AbstractURL,
        ) {
            results.push(SearchResult {
                title,
                snippet: text,
                url,
                source: self.name().into(),
                score: 2.0,
            });
        }

        if let Some(topics) = response.RelatedTopics {
            Self::extract_topics(topics, &mut results);
        }

        Ok(results)
    }
}

impl DuckDuckGo {
    fn extract_topics(topics: Vec<RelatedTopic>, results: &mut Vec<SearchResult>) {
        for topic in topics {
            match topic {
                RelatedTopic::Topic { Text, FirstURL } => {
                    results.push(SearchResult {
                        title: Text.clone(),
                        snippet: Text,
                        url: FirstURL,
                        source: "DuckDuckGo".into(),
                        score: 1.0,
                    });
                }
                RelatedTopic::Category { Topics } => {
                    Self::extract_topics(Topics, results);
                }
            }
        }
    }
}
