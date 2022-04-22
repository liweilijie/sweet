use axum::error_handling::HandleErrorLayer;
use axum::Server;
use std::net::SocketAddr;
use std::time::Duration;
use sweet::api::apply_routes;
use sweet::error::Error;
use sweet::logger::Logger;
use sweet::middleware::handle_error;
use sweet::repository::init_db;
use sweet::settings::Settings;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let settings = Settings::from_env()?;
    Logger::init();

    init_db().await;

    let middlewares = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(HandleErrorLayer::new(handle_error))
        .layer(CompressionLayer::new())
        .timeout(Duration::from_secs(10));

    let routes = apply_routes().layer(middlewares);

    let port = settings.app_port.unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Starting server on {addr}");
    debug!("debug enable.");

    Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("app started failed.");

    Ok(())
}
