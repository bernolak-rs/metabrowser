use actix_cors::Cors;
use actix_files as fs;
use actix_web::{self, App, HttpServer, web};
use dotenvy::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use web_library::Aggregator;
use web_library::browsers::BraveSearchEngine;
use web_library::browsers::Config;
use web_library::browsers::WikipediaClient;
use web_library::{SearchEngine, browsers::DuckDuckGo};

mod handlers;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::hello,
        handlers::search,
        handlers::register,
        handlers::login,
        handlers::logout,
        handlers::get_user_history
    ),
    components(schemas(
        handlers::SimpleResponse,
        handlers::SearchResultDto,
        handlers::AuthRequest,
        handlers::HistoryEntryDto
    ))
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let openapi = ApiDoc::openapi();

    let config = Config::from_env().expect("Failed to load config");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = web_library::db::establish_connection_pool(&database_url);
    let pool_data = web::Data::new(pool);

    let ddg: Box<dyn SearchEngine + Send + Sync> = Box::new(DuckDuckGo::new());
    let brave: Box<dyn SearchEngine + Send + Sync> = Box::new(BraveSearchEngine::new(&config));
    let wiki: Box<dyn SearchEngine + Send + Sync> = Box::new(WikipediaClient::new());
    let aggregator = web::Data::new(Aggregator::new(vec![ddg, wiki]));

    let secret_key = actix_web::cookie::Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_session::SessionMiddleware::new(
                actix_session::storage::CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .wrap(Cors::permissive())
            .wrap(actix_web::middleware::Logger::default())
            .app_data(aggregator.clone())
            .app_data(pool_data.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(handlers::hello)
            .service(handlers::search)
            .service(handlers::register)
            .service(handlers::login)
            .service(handlers::logout)
            .service(handlers::get_user_history)
            .service(
                fs::Files::new("/", "./frontend")
                    .index_file("index.html")
                    .show_files_listing(),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
