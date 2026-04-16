use axum::{extract::Path, routing::get, Router};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { maud_ui::showcase::showcase_page() }))
        .route(
            "/{component}",
            get(|Path(name): Path<String>| async move {
                maud_ui::showcase::component_page_by_name(&name)
            }),
        )
        .nest_service("/dist", ServeDir::new("dist"));

    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3457".to_string());
    let addr = addr.as_str();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("maud-ui gallery running on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
