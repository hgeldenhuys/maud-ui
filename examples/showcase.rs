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
        [(
            header::CONTENT_TYPE,
            "application/javascript; charset=utf-8",
        )],
        js,
    )
}

/// The mark. Long cache: the bytes are content-stable and a favicon refetch on
/// every navigation is pure waste.
async fn serve_favicon() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "image/svg+xml"),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        include_str!("../assets/favicon.svg"),
    )
}

async fn serve_touch_icon() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "image/png"),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        include_bytes!("../assets/apple-touch-icon.png").as_slice(),
    )
}

/// Link-preview card. Fetched by crawlers (X, Slack, Discord, iMessage,
/// Mastodon), which is why it is a committed PNG rather than rendered on
/// demand — none of them will run a build step.
async fn serve_og() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "image/png"),
            (header::CACHE_CONTROL, "public, max-age=86400"),
        ],
        include_bytes!("../assets/og.png").as_slice(),
    )
}

#[tokio::main]
async fn main() {
    let app =
        Router::new()
            .route("/", get(|| async { maud_ui::showcase::landing_page() }))
            // The 72-component index. It lived at `/` until the landing page
            // took that slot; a static route outranks the `/{component}`
            // catch-all below, so no component slug can shadow it.
            .route("/gallery", get(|| async { maud_ui::showcase::showcase_page() }))
            .route(
                "/getting-started",
                get(|| async { maud_ui::showcase::getting_started_page() }),
            )
            .route(
                "/theme",
                get(|| async { maud_ui::showcase::theme_customizer_page() }),
            )
            .route(
                "/blocks",
                get(|| async { maud_ui::showcase::blocks_index_page() }),
            )
            .route(
                "/blocks/{slug}",
                get(|Path(slug): Path<String>| async move {
                    maud_ui::showcase::block_page_by_name(&slug)
                }),
            )
            .route(
                "/integrations/monaco-editor",
                get(|| async { maud_ui::showcase::integrations_monaco_page() }),
            )
            .route(
                "/integrations/xyflow",
                get(|| async { maud_ui::showcase::integrations_xyflow_page() }),
            )
            .route(
                "/integrations/excalidraw",
                get(|| async { maud_ui::showcase::integrations_excalidraw_page() }),
            )
            .route(
                "/integrations/xterm",
                get(|| async { maud_ui::showcase::integrations_xterm_page() }),
            )
            .route(
                "/integrations/fullcalendar",
                get(|| async { maud_ui::showcase::integrations_fullcalendar_page() }),
            )
            .route(
                "/integrations/leaflet",
                get(|| async { maud_ui::showcase::integrations_leaflet_page() }),
            )
            .route(
                "/integrations/tiptap",
                get(|| async { maud_ui::showcase::integrations_tiptap_page() }),
            )
            .route(
                "/integrations/threejs",
                get(|| async { maud_ui::showcase::integrations_threejs_page() }),
            )
            .route(
                "/integrations/ag-grid",
                get(|| async { maud_ui::showcase::integrations_aggrid_page() }),
            )
            .route(
                "/integrations/mermaid",
                get(|| async { maud_ui::showcase::integrations_mermaid_page() }),
            )
            .route(
                "/integrations/echarts",
                get(|| async { maud_ui::showcase::integrations_echarts_page() }),
            )
            .route(
                "/integrations/wavesurfer",
                get(|| async { maud_ui::showcase::integrations_wavesurfer_page() }),
            )
            .route(
                "/integrations/pdfjs",
                get(|| async { maud_ui::showcase::integrations_pdfjs_page() }),
            )
            .route(
                "/integrations/cytoscape",
                get(|| async { maud_ui::showcase::integrations_cytoscape_page() }),
            )
            .route(
                "/integrations/sortable",
                get(|| async { maud_ui::showcase::integrations_sortable_page() }),
            )
            .route("/css/maud-ui.css", get(serve_css))
            .route("/js/maud-ui.js", get(serve_js))
            .route("/favicon.svg", get(serve_favicon))
            .route("/apple-touch-icon.png", get(serve_touch_icon))
            .route("/og.png", get(serve_og))
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
