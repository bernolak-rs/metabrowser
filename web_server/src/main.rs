use actix_web::{self, App, HttpResponse, HttpServer, Responder, get};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(hello), components(schemas(SimpleResponse)))]
struct ApiDoc;

#[derive(serde::Serialize, utoipa::ToSchema)]
struct SimpleResponse {
    status: u16,
}
#[utoipa::path(
    responses(
        (status = 200, description = "API is alive", body = SimpleResponse)
    )
)]
#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(SimpleResponse { status: 200 })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
