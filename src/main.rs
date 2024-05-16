mod settings;
mod templates;

use axum::{http::StatusCode, response::{Html, IntoResponse}, routing::get, Router};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use tower_livereload::LiveReloadLayer;
use tracing::{info, warn};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;

use askama::Template;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber
    let trace_sub = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new("askama_axum_rust_template=debug"))
        .finish();
    tracing::subscriber::set_global_default(trace_sub).unwrap();

    let app = Router::new()
        .route("/", get(handle_main))
        .route("/projects", get(handle_projects))
        .route("/examples", get(handle_examples))
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(LiveReloadLayer::new());

    let listen_addr: SocketAddr = format!("{}:{}", settings::default_ip(), settings::default_port())
        .parse()
        .unwrap();

    info!("Listening on http://{}", listen_addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn handle_main() -> impl IntoResponse {
    let nav_links = generate_nav_links();
    let navbar = templates::Navbar { links: nav_links };
    let main = templates::Main { links: navbar.links };

    match main.render() {
        Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
        Err(err) => {
            warn!("Failed to render template: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".into_response())
        }
    }
}

async fn handle_projects() -> impl IntoResponse {
    let nav_links = generate_nav_links();
    let projects = templates::Projects { links: nav_links };

    match projects.render() {
        Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
        Err(err) => {
            warn!("Failed to render projects template: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".into_response())
        }
    }
}

async fn handle_examples() -> impl IntoResponse {
    let nav_links = generate_nav_links();
    let examples = templates::Examples { links: nav_links };

    match examples.render() {
        Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
        Err(err) => {
            warn!("Failed to render examples template: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".into_response())
        }
    }
}

fn generate_nav_links() -> Vec<templates::NavLink> {
    vec![
        templates::NavLink {
            name: "Home",
            url: "/",
        },
        templates::NavLink {
            name: "Projects",
            url: "/projects",
        },
        templates::NavLink {
            name: "Examples",
            url: "/examples",
        },
    ]
}
