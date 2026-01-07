use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::db::schema::{users, search_history};
use chrono::NaiveDateTime;


#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
}


#[derive(Queryable, Selectable, Associations, Serialize, Debug)]
#[diesel(belongs_to(User))]
#[diesel(table_name = search_history)]
pub struct SearchHistory {
    pub id: i32,
    pub user_id: i32,
    pub query_text: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = search_history)]
pub struct NewSearchEntry {
    pub user_id: i32,
    pub query_text: String,
}
