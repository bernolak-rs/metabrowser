use crate::browsers::SearchEngine;
use crate::search_result::SearchResult;

pub struct Aggregator {
    engines: Vec<Box<dyn SearchEngine>>,
}

impl Aggregator {
    pub fn new(engines: Vec<Box<dyn SearchEngine>>) -> Self {
        Self { engines }
    }

    pub async fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut all_results = Vec::new();

        for engine in &self.engines {
            match engine.search(query).await {
                Ok(mut results) => {
                    all_results.append(&mut results);
                }
                Err(e) => {
                    eprintln!("{} failed: {}", engine.name(), e);
                }
            }
        }

        self.rank(all_results)
    }

    fn rank(&self, mut results: Vec<SearchResult>) -> Vec<SearchResult> {
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }
}
