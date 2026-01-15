//! # Search Engine and Browsers
//!
//! This module contains the business logic for handling web searching.
//! Each actual engine implements SearchEngine trait, that mainly contains asynchronous method
//! "search" that returns searched results.
//!
//! ## Implemented engines:
//! * **DuckDuckGo
//! * **BraveAPI
//! * **WikipediaClient

pub mod brave_engine;
pub mod duckduckgo_engine;
pub mod search_engine;
pub mod wikipedia;

pub use brave_engine::BraveSearchEngine;
pub use duckduckgo_engine::DuckDuckGo;
pub use search_engine::SearchEngine;
pub use wikipedia::WikipediaClient;
