use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};
use maud_ui::primitives::{card, dialog};

async fn index() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Welcome — maud-ui" }
                link rel="stylesheet" href="/assets/maud-ui.min.css";
                script src="/assets/maud-ui.min.js" defer {}
            }
            body {
                main style="max-width:40rem;margin:3rem auto;padding:1.5rem;" {
                    h1 { "Your first screen" }
                    (card::render(card::Props {
                        title: Some("Welcome".into()),
                        description: Some("You're running maud-ui.".into()),
                        children: dialog::trigger("welcome", "Open welcome dialog"),
                        ..Default::default()
                    }))
                    (dialog::render(dialog::Props {
                        id: "welcome".into(),
                        title: "You're ready to build".into(),
                        children: html! { p { "Close this dialog or press Escape to return to your screen." } },
                        ..Default::default()
                    }))
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route(
            "/assets/maud-ui.min.css",
            get(|| async { ([("content-type", "text/css")], maud_ui::assets::CSS_MIN) }),
        )
        .route(
            "/assets/maud-ui.min.js",
            get(|| async {
                (
                    [("content-type", "application/javascript")],
                    maud_ui::assets::JS_MIN,
                )
            }),
        );

    let address = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("Open http://{address}");
    axum::serve(listener, app).await.unwrap();
}
