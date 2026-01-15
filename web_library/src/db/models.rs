use crate::db::schema::{search_history, users};
use chrono::NaiveDateTime;
/// Models created for database.
/// Models are structs use for ORM.
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Table entry of individual users search history
#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

/// Table entry representing new user
#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
}

/// Table entry containing registered users of website
#[derive(Queryable, Selectable, Associations, Serialize, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = search_history)]
pub struct SearchHistory {
    pub id: i32,
    pub user_id: i32,
    pub query_text: String,
    pub created_at: NaiveDateTime,
}

/// Table entry representing new search
#[derive(Insertable, Deserialize)]
#[diesel(table_name = search_history)]
pub struct NewSearchEntry {
    pub user_id: i32,
    pub query_text: String,
}
