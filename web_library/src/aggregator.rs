use crate::browsers::SearchEngine;
use crate::search_result::SearchResult;
use futures::future::join_all;

pub struct Aggregator {
    engines: Vec<Box<dyn SearchEngine + Send + Sync>>,
}

impl Aggregator {
    pub fn new(engines: Vec<Box<dyn SearchEngine + Send + Sync>>) -> Self {
        Self { engines }
    }

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

    fn rank(&self, mut results: Vec<SearchResult>) -> Vec<SearchResult> {
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }
}
