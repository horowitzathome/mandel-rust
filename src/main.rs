use axum::{routing, Router};
use tracing::*;

mod listen;
mod mandel;

pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
    // Tracing!
    register_tracing();

    // Now trace the started message
    info!("Started mandel");

    // Create Router
    let router = create_router();

    // Start Web Server at port 8080
    use tokio::signal::unix as usig;
    let mut shutdown = usig::signal(usig::SignalKind::terminate())?;
    let server = axum::Server::bind(&std::net::SocketAddr::from(([0, 0, 0, 0], 8080)))
        .serve(router.into_make_service())
        .with_graceful_shutdown(async move {
            shutdown.recv().await;
        });

    // Wait for all async tasks to be completed
    tokio::select! {
        _ = server => info!("axum server exited"),
    }

    // We are done
    Ok(())
}

fn create_router() -> Router {
    let routers = Router::new()
        // Here the business routes later
        .route("/mandel_json/:max_iter", routing::get(listen::mandel_json))
        .route("/mandel_text/:max_iter", routing::get(listen::mandel_text))
        //.layer(Extension(reader_deployment))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        // Reminder: routes added *after* TraceLayer are not subject to its logging behavior
        .route("/actuator/health", routing::get(listen::health));

    Router::new().nest("/mandel-rust", routers)
}

fn register_tracing() {
    tracing_subscriber::fmt()
        .json()
        .flatten_event(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_max_level(Level::INFO)
        .init();
}
