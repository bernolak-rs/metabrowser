//! # Web Library
//!
//! This library contains main business logic of this project, such as aggregation of searched
//! results, their processing and their storage in database.
//!
//! ## Example
//!
//! ```rust
//! use web_library::{Aggregator, browsers::DuckDuckGo, Config};
//!
//! async fn demo() {
//!     let ddg = Box::new(DuckDuckGo::new());
//!     let aggregator = Aggregator::new(vec![ddg]);
//!     let results = aggregator.search("rust programming").await;
//! }
//! ```
pub mod aggregator;
pub mod browsers;
pub mod config;
pub mod db;
pub mod search_result;

pub use aggregator::Aggregator;
pub use browsers::SearchEngine;
pub use config::Config;
pub use db::DbPool;
pub use search_result::SearchResult;
