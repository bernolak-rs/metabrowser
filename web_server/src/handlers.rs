use actix_web::{self, HttpResponse, Responder, get, post, web};

use web_library::Aggregator;
use web_library::SearchResult;
use web_library::db;

use actix_session::Session;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Basic HTTP responses
#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct SimpleResponse {
    status: u16,
}

/// Data required for authentification of a user
#[derive(Serialize, Deserialize, ToSchema)]
pub struct AuthRequest {
    #[schema(example = "adam_test")]
    pub username: String,
    #[schema(example = "password123")]
    pub password: String,
}

/// Search result that is being send to frontend
#[derive(Serialize, ToSchema)]
pub struct SearchResultDto {
    pub title: String,
    pub url: String,
    pub description: String,
    pub source: String,
}

/// Converts SearchResult to DTO for API
impl From<SearchResult> for SearchResultDto {
    fn from(value: SearchResult) -> Self {
        Self {
            title: value.title,
            url: value.url,
            description: value.snippet,
            source: value.source,
        }
    }
}

/// History entry for user to see on frontend
#[derive(Serialize, ToSchema)]
pub struct HistoryEntryDto {
    pub query_text: String,
    pub created_at: String,
}

/// Endpoint for easy test of the web server
/// Return 200 if everything runs correctly
#[utoipa::path(
    responses(
        (status = 200, description = "API is alive", body = SimpleResponse),
    )
)]
#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(SimpleResponse { status: 200 })
}

/// Asynchronous search of the used browsers
/// If user is logged in his search also gets stored in DB
/// Parameters:
/// - aggregator - aggregates multiple search engines for asynchronous search
/// - pool - database pool
/// - query - searched phrase
/// - session - user session
#[utoipa::path(
    params(
        ("query" = String, Path, description = "Search query")
    ),
    responses(
        (status = 200, description = "Search results", body = [SearchResultDto]),
        (status = 500, description = "Internal error"),
    )
)]
#[get("/search/{query}")]
async fn search(
    aggregator: web::Data<Aggregator>,
    pool: web::Data<web_library::db::DbPool>,
    query: web::Path<String>,
    session: Session,
) -> impl Responder {
    let query_str = query.into_inner();

    println!("DEBUG: Request received for query: '{}'", query_str);

    let user_id: i32 = session.get::<i32>("user_id").unwrap_or(None).unwrap_or(0);

    if user_id != 0 {
        let query_for_db = query_str.clone();
        let pool_for_db = pool.clone();

        let _ = web::block(move || {
            let mut conn = pool_for_db.get().expect("Failed to get connection");
            web_library::db::managed::save_search(&mut conn, user_id, &query_for_db)
        })
        .await;
    }

    let results = aggregator.search(&query_str).await;
    println!(
        "Debug: Found {} results for query '{}'",
        results.len(),
        query_str
    );
    let dto: Vec<SearchResultDto> = results.into_iter().map(Into::into).collect();

    HttpResponse::Ok().json(dto)
}

/// Registers new user as an entry into a DB
/// Returns responses whether user was correctly added
#[utoipa::path(
    request_body = AuthRequest,
    responses(
        (status = 201, description = "User registered successfully"),
        (status = 409, description = "Username already exists"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/register")]
async fn register(
    pool: web::Data<web_library::db::DbPool>,
    form: web::Json<AuthRequest>,
) -> impl Responder {
    let pool_copy = pool.clone();

    let result = web::block(move || {
        let mut conn = pool_copy.get().expect("DB connection failed");
        db::managed::register_user(&mut conn, &form.username, &form.password)
    })
    .await;

    match result {
        Ok(_) => HttpResponse::Created().body("User registered successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Username might already exist"),
    }
}

/// Logs user in
/// If user logs correctly user_id gets inserted into session
#[utoipa::path(
    request_body = AuthRequest,
    responses(
        (status = 200, description = "Login successful"),
        (status = 401, description = "Invalid credentials")
    )
)]
#[post("/login")]
async fn login(
    pool: web::Data<web_library::db::DbPool>,
    form: web::Json<AuthRequest>,
    session: Session,
) -> impl Responder {
    let pool_copy = pool.clone();

    let result = web::block(move || {
        let mut conn = pool_copy.get().expect("DB connection failed");
        db::managed::login_user(&mut conn, &form.username, &form.password)
    })
    .await;

    match result {
        Ok(Ok(user_id)) => {
            session
                .insert("user_id", user_id)
                .expect("Failed to set session");
            HttpResponse::Ok().body("Login successful")
        }
        _ => HttpResponse::Unauthorized().body("Invalid username or password"),
    }
}

/// Logs out user -> removes user_id from session
#[utoipa::path(
    responses(
        (status = 200, description = "Logged out successfully"),
    )
)]
#[post("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok().body("Logged out successfully")
}

/// Loads users history if user is logged in
#[utoipa::path(
    responses(
        (status = 200, description = "List of past searches", body = [HistoryEntryDto]),
        (status = 401, description = "Unauthorized - Must be logged in")
    )
)]
#[get("/history")]
async fn get_user_history(
    pool: web::Data<web_library::db::DbPool>,
    session: Session,
) -> impl Responder {
    let user_id: i32 = match session.get::<i32>("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().body("Please log in to see history"),
    };

    let pool_copy = pool.clone();
    let result = web::block(move || {
        let mut conn = pool_copy.get().expect("DB connection failed");
        db::managed::get_history(&mut conn, user_id)
    })
    .await;

    match result {
        Ok(Ok(history)) => {
            let dtos: Vec<HistoryEntryDto> = history
                .into_iter()
                .map(|h| HistoryEntryDto {
                    query_text: h.query_text,
                    created_at: h.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                })
                .collect();
            HttpResponse::Ok().json(dtos)
        }
        _ => HttpResponse::InternalServerError().body("Failed to load history"),
    }
}
