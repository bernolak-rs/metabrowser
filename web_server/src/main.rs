use actix_cors::Cors;
use actix_files as fs;
use actix_web::{self, App, HttpResponse, HttpServer, Responder, get, web};
use dotenvy::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use web_library::Aggregator;
use web_library::browsers::BraveSearchEngine;
use web_library::browsers::Config;
use web_library::browsers::WikipediaClient;
use web_library::{SearchEngine, SearchResult, browsers::DuckDuckGo};

#[derive(serde::Serialize, utoipa::ToSchema)]
struct SimpleResponse {
    status: u16,
}

#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
struct SearchResultDto {
    title: String,
    url: String,
    snippet: String,
    source: String,
}

impl From<SearchResult> for SearchResultDto {
    fn from(value: SearchResult) -> Self {
        Self {
            title: value.title,
            url: value.url,
            snippet: value.snippet,
            source: value.source,
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(hello, search),
    components(schemas(SimpleResponse, SearchResultDto))
)]
struct ApiDoc;

#[utoipa::path(
    responses(
        (status = 200, description = "API is alive", body = SimpleResponse),
    )
)]
#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(SimpleResponse { status: 200 })
}

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
async fn search(aggregator: web::Data<Aggregator>, query: web::Path<String>) -> impl Responder {
    let query_str = query.into_inner();

    let results = aggregator.search(&query_str).await;

    let dto: Vec<SearchResultDto> = results.into_iter().map(Into::into).collect();

    HttpResponse::Ok().json(dto)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let openapi = ApiDoc::openapi();

    let config = Config::from_env().expect("Failed to load config");

    let ddg: Box<dyn SearchEngine + Send + Sync> = Box::new(DuckDuckGo::new());
    let brave: Box<dyn SearchEngine + Send + Sync> = Box::new(BraveSearchEngine::new(&config));
    let wiki: Box<dyn SearchEngine + Send + Sync> = Box::new(WikipediaClient::new());
    let aggregator = web::Data::new(Aggregator::new(vec![ddg, wiki]));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(actix_web::middleware::Logger::default())
            .app_data(aggregator.clone())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(hello)
            .service(search)
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
