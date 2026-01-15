//! # Web Library
//!
//! This library contains main business logic of this project, such as aggregation of searched
//! results, their processing and their storage in database.
//!
//! ## Core Architecture
//!
//! The library is organized into several key components:
//! * **[browsers]**: This module contains the specific implementations for various search engines.
//!     * It defines the **[SearchEngine]** trait, which all providers (Wikipedia, DuckDuckGo, etc.) must implement.
//! * **[db]**: Handles all database interactions and connectivity.
//!     * It manages the **[DbPool]** and provides functions for registering users and saving search history.
//! * **[Aggregator]**: The orchestrator that executes searches across multiple engines in parallel.
//! * **[SearchEngine]**: A trait that defines the interface for adding new search providers.
//! * **[DbPool]**: A thread-safe PostgreSQL connection pool for persisting user history.
//! * **[SearchResult]**: The unified data model representing a result from any source.
//!
//! ## Feature Highlights
//! * **Concurrency**: Uses `join_all` to fetch results from all engines simultaneously, ensuring high performance.
//! * **Resilience**: If one engine fails, the aggregator logs the error and returns the remaining results.
//! * **Ranking**: Results are automatically sorted based on an internal scoring system.
//!
//! ## Examples
//!
//! ```rust
//! use web_library::{Aggregator, browsers::DuckDuckGo, browsers::WikipediaClient, Config};
//!
//! async fn run_search() -> anyhow::Result<()> {
//!     // 1. Initialize search engines
//!     let ddg = Box::new(DuckDuckGo::new());
//!     let wiki = Box::new(WikipediaClient::new());
//!
//!     // 2. Initialize the Aggregator with multiple engines
//!     let aggregator = Aggregator::new(vec![ddg, wiki]);
//!
//!     // 3. Perform a concurrent search
//!     let query = "Rust programming";
//!     let results = aggregator.search(query).await;
//!
//!     println!("Found {} results for '{}'", results.len(), query);
//!     Ok(())
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
