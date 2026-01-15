use serde::Serialize;

/// Represents result of a search query.
#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub title: String,
    pub snippet: String,
    pub url: String,
    pub source: String,
    pub score: f32,
}
