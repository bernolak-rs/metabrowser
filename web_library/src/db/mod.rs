//! # Database Module
//!
//! This module handles database logic. It uses Diesel ORM.
//! It provides connection and pooling for ORM related to handling user accounts and user accounts
//! history.

pub mod managed;
pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}
