use axum::{routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { maud_ui::showcase::showcase_page() }))
        .nest_service("/dist", ServeDir::new("dist"));

    let addr = "127.0.0.1:3456";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("maud-ui gallery running on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
