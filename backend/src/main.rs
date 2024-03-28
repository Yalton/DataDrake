//main.rs

use std::net::SocketAddr;
use std::time::Duration;



use axum::{error_handling::*, routing::*};
use axum::{middleware, Router, Server};
use setup::{Args, State};
use tower_http::ServiceBuilderExt;
use tower_http::{cors::*, request_id::*, trace::*};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt; // Add this line

mod auth;
mod endpoint;
mod services;
mod setup;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Tracing.
    let env = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "data_drake=trace,tower_http=trace".into());
    let fmt = tracing_subscriber::fmt::layer().pretty().with_target(true);
    tracing_subscriber::registry().with(fmt).with(env).init();

    // Services.
    let state = State::new(&args).await?;

    // Middlewares.
    let error_layer = HandleErrorLayer::new(endpoint::handle_box_error);
    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_response(DefaultOnResponse::new().include_headers(true));
    let cors_layer = CorsLayer::permissive();

    let middlewares = tower::ServiceBuilder::default()
        .layer(cors_layer)
        .layer(error_layer)
        .timeout(Duration::from_secs(300))
        .compression()
        // Strict order: Set > Trace > Propagate.
        .set_x_request_id(MakeRequestUuid)
        .layer(tracing_layer)
        .propagate_x_request_id();

    // Endpoints.
    let app = Router::default()
        .route(
            "/",
            get(|| async { "You have hit the backend congratulations" }),
        )
        .route("/scan_directory", post(endpoint::scan_directory))
        // .route("/get_job_status", get(endpoint::get_job_status))
        .route("/get_job_status/all", get(endpoint::get_all_jobs))
        .route("/get_root_paths", get(endpoint::get_root_paths))
        .route("/get_scan", post(endpoint::get_all_scans_by_root_path))
        .route("/get_scan/all", post(endpoint::get_all_scans))
        .route(
            "/delete_scan",
            delete(endpoint::delete_all_scans_by_root_path),
        )
        .route("/delete_file", delete(endpoint::delete_file_by_path))
        .route("/delete_scan/all", delete(endpoint::delete_all_scans))
        .route_layer(middleware::from_extractor::<auth::Auth>())
        .layer(middlewares)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    println!("Data-Drake Scanning service online @: http://{}", addr);
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
