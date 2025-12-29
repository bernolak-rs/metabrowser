use actix_web::{self, App, HttpResponse, HttpServer, Responder, get, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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
async fn search(ddg: web::Data<DuckDuckGo>, query: web::Path<String>) -> impl Responder {
    match ddg.get_ref().search(&query).await {
        Ok(results) => {
            let dto: Vec<SearchResultDto> = results.into_iter().map(Into::into).collect();

            HttpResponse::Ok().json(dto)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(DuckDuckGo::new()))
            .wrap(actix_web::middleware::Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(hello)
            .service(search)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
