use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

/// Serve the bundled CSS from dist/maud-ui.css
async fn serve_css() -> impl IntoResponse {
    let css = include_str!("../dist/maud-ui.css");
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        css,
    )
}

/// Serve the bundled JS from dist/maud-ui.js
async fn serve_js() -> impl IntoResponse {
    let js = include_str!("../dist/maud-ui.js");
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/javascript; charset=utf-8")],
        js,
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { maud_ui::showcase::showcase_page() }))
        .route(
            "/getting-started",
            get(|| async { maud_ui::showcase::getting_started_page() }),
        )
        .route(
            "/css/maud-ui.css",
            get(serve_css),
        )
        .route(
            "/js/maud-ui.js",
            get(serve_js),
        )
        .route(
            "/{component}",
            get(|Path(name): Path<String>| async move {
                maud_ui::showcase::component_page_by_name(&name)
            }),
        )
        .nest_service("/dist", ServeDir::new("dist"));

    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3456".to_string());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("maud-ui gallery running on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
