mod settings;
mod templates;
mod data;

use axum::{body::Body, extract::Path, http::{Response, StatusCode}, response::{Html, IntoResponse, Redirect}, routing::get, Router};
use sha2::{Digest, Sha256};
use templates::{Header, Navbar, Raw};
use std::net::SocketAddr;
use tracing::{info, warn};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;

use askama::Template;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

impl Asset {
    fn get_hashed_url(file: &str) -> String {
        if let Some(content) = Asset::get(file) {
            let hash = Sha256::digest(&content.data);
            let hash_str = format!("{:x}", hash);
            format!("/assets/{}?v={}", file, &hash_str[..8])
        } else {
            panic!("/assets/{} not found", file);
        }
    }
}


#[tokio::main]
async fn main() {
    let trace_sub = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new("askama_axum_rust_template=debug"))
        .finish();
    tracing::subscriber::set_global_default(trace_sub).unwrap();

    let app = Router::new()
        .route("/", get(|| render(templates::Main::create())))

        .route("/projects", get(|| async { Redirect::to("/projects/sursface") }))
        .route("/projects/sursface", get(|| render(templates::ProjectsSursface::create())))
        .route("/projects/website", get(|| render(templates::ProjectsWebsite::create())))

        .route("/examples", get(|| async { Redirect::to("/examples/sursface") }))
        .route("/examples/sursface", get(|| render(templates::ExamplesSursface::create())))
        .route("/examples/sayle", get(|| render(templates::ExamplesSayle::create())))

        .route("/blog", get(|| render(templates::Blog::create())))
        .route("/blog/crust", get(|| render(templates::BlogCrust::default())))

        .route("/favicon.ico", get(|| async { Redirect::to("/assets/images/flake.png") }))
        .route("/assets/*file", get(serve_embedded_file))
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

async fn serve_embedded_file(Path(file): Path<String>) -> impl IntoResponse {
    match Asset::get(&file) {
        Some(content) => {
            let body = content.data.into_owned();
            let mime_type = mime_guess::from_path(&file).first_or_octet_stream();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", mime_type.as_ref())
                .body(Body::from(body))
                .unwrap()
        },
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("File Not Found"))
            .unwrap(),
    }
}

async fn handle_404(uri: axum::extract::OriginalUri) -> impl IntoResponse {
    let path = uri.0.path();
    if path == "/" || path.is_empty() {
        (StatusCode::NOT_FOUND, "Not Found").into_response()
    } else if let Some(new_path) = path.rsplit_once('/') {
        let new_path = if new_path.0.is_empty() { "/" } else { new_path.0 };
        Redirect::to(new_path).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Not Found").into_response()
    }
}
