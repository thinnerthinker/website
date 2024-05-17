mod settings;
mod templates;
mod data;

use axum::{http::StatusCode, response::{Html, IntoResponse, Redirect}, routing::get, Router};
use templates::{Header, Navbar, Raw};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use tracing::{info, warn};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;

use askama::Template;

#[tokio::main]
async fn main() {
    let trace_sub = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new("askama_axum_rust_template=debug"))
        .finish();
    tracing::subscriber::set_global_default(trace_sub).unwrap();

    let app = Router::new()
        .route("/", get(|| render(templates::Main::create())))
        .route("/projects", get(|| render(templates::Projects::create())))
        .route("/examples", get(|| async { Redirect::to("/examples/sursface") }))
        .route("/examples/sursface", get(|| render(templates::ExamplesSursface::create())))
        .route("/favicon.ico", get(Redirect::to("/assets/images/flake.png")))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(handle_404);

    let listen_addr: SocketAddr = format!("{}:{}", settings::default_ip(), settings::default_port())
        .parse()
        .unwrap();

    info!("Listening on http://{}", listen_addr);

    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn render(template: impl Template) -> impl IntoResponse {
    match template.render() {
        Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
        Err(err) => {
            warn!("Failed to render template: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".into_response())
        }
    }
}

async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Html("<h1>404 - Not Found</h1><p>The page you are looking for does not exist.</p>"))
}