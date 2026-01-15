use crate::browsers::SearchEngine;
use crate::search_result::SearchResult;
use futures::future::join_all;

/// Contains search engines and searches web through them concurrently.
pub struct Aggregator {
    engines: Vec<Box<dyn SearchEngine + Send + Sync>>,
}

impl Aggregator {
    pub fn new(engines: Vec<Box<dyn SearchEngine + Send + Sync>>) -> Self {
        Self { engines }
    }

    /// Concurrently searches web through all engines.
    /// Creates concurrent task for each engine and uses join_all to wait for them to finish.
    /// If an error occurs, it will be logged and rest of the correct results will be returned.
    pub async fn search(&self, query: &str) -> Vec<SearchResult> {
        let tasks = self.engines.iter().map(|engine| async move {
            match engine.search(query).await {
                Ok(results) => results,
                Err(e) => {
                    log::error!("Engine {} failed: {:?}", engine.name(), e);
                    vec![]
                }
            }
        });

        let results_of_results = join_all(tasks).await;
        let all_results: Vec<SearchResult> = results_of_results.into_iter().flatten().collect();

        self.rank(all_results)
    }

    /// Sorts results based on their score
    fn rank(&self, mut results: Vec<SearchResult>) -> Vec<SearchResult> {
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }
}
