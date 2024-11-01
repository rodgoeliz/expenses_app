use aide::scalar::Scalar;
use poem::{endpoint::make_sync, listener::TcpListener, web::Html, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
struct Api;

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }
}

#[tokio::main]
async fn main() {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000");
    let spec_yaml = api_service.spec_endpoint_yaml();

    let app = Route::new()
        .nest("/", api_service)
        .nest("/spec", spec_yaml)
        .nest(
            "/docs",
            make_sync(|_| Html(Scalar::new("/spec").with_title("Aide Axum").html())),
        );

    let _ = Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;
}
