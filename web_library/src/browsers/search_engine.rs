use crate::search_result::SearchResult;
use async_trait::async_trait;

#[async_trait]
pub trait SearchEngine: Send + Sync {
    fn name(&self) -> &'static str;
    async fn search(&self, query: &str) -> anyhow::Result<Vec<SearchResult>>;
}
