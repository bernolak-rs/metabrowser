use crate::db::models::NewSearchEntry;
use crate::db::models::NewUser;
use crate::db::schema::search_history;
use crate::db::schema::users;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use diesel::prelude::*;

/// Registers a new user with hashed password.
pub fn register_user(
    conn: &mut PgConnection,
    username: &str,
    password_raw: &str,
) -> QueryResult<usize> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password_raw.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    let new_user = NewUser {
        username: username.to_string(),
        password_hash,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
}

/// Verifies users login information.
pub fn login_user(
    conn: &mut PgConnection,
    targer_username: &str,
    password_raw: &str,
) -> Result<i32, String> {
    use crate::db::schema::users::dsl::*;

    let user = users
        .filter(username.eq(targer_username))
        .first::<crate::db::models::User>(conn)
        .map_err(|_| "User not found".to_string())?;

    let parsed_hash =
        PasswordHash::new(&user.password_hash).map_err(|_| "Invalid hash format".to_string())?;

    if Argon2::default()
        .verify_password(password_raw.as_bytes(), &parsed_hash)
        .is_ok()
    {
        Ok(user.id)
    } else {
        Err("Invalid password".to_string())
    }
}

/// Saves search into database for history access.
pub fn save_search(conn: &mut PgConnection, user_id: i32, query: &str) -> QueryResult<usize> {
    let new_entry = NewSearchEntry {
        user_id,
        query_text: query.to_string(),
    };

    diesel::insert_into(search_history::table)
        .values(&new_entry)
        .execute(conn)
}

/// Fetches user searches and sorts them.
pub fn get_history(
    conn: &mut PgConnection,
    uid: i32,
) -> QueryResult<Vec<crate::db::models::SearchHistory>> {
    use crate::db::schema::search_history::dsl::*;

    search_history
        .filter(user_id.eq(uid))
        .order(created_at.desc())
        .limit(50)
        .load::<crate::db::models::SearchHistory>(conn)
}
