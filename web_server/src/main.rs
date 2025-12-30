use actix_cors::Cors;
use actix_files as fs;
use actix_web::{self, App, HttpResponse, HttpServer, Responder, get, web};
use dotenvy::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use web_library::browsers::BraveSearchEngine;
use web_library::browsers::Config;
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
}

impl From<SearchResult> for SearchResultDto {
    fn from(value: SearchResult) -> Self {
        Self {
            title: value.title,
            url: value.url,
            snippet: value.snippet,
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
async fn search(
    ddg: web::Data<DuckDuckGo>,
    brave: web::Data<BraveSearchEngine>,
    query: web::Path<String>,
) -> impl Responder {
    let query_str = query.into_inner();

    // Run both searches in parallel
    let ddg_fut = ddg.get_ref().search(&query_str);
    let brave_fut = brave.get_ref().search(&query_str);

    let (ddg_res, brave_res) = tokio::join!(ddg_fut, brave_fut);

    let mut results = Vec::new();

    if let Ok(ddg_results) = ddg_res {
        results.extend(ddg_results);
    }

    if let Ok(brave_results) = brave_res {
        results.extend(brave_results);
    }

    // Map to DTOs
    let dto: Vec<SearchResultDto> = results.into_iter().map(Into::into).collect();

    HttpResponse::Ok().json(dto)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let openapi = ApiDoc::openapi();

    let config = Config::from_env().expect("Failed to load config");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(DuckDuckGo::new()))
            .app_data(web::Data::new(BraveSearchEngine::new(&config)))
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
