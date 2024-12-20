use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use message::handle_message;
use std::error::Error;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
mod message;
use tracing::{error, info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .inspect_err(|err| error!("{err:?}"))?;
    let addr = format!("{host}:{port}");

    let listener = TcpListener::bind(&addr)
        .await
        .inspect_err(|err| info!("{err:?}"))?;

    info!("Listening on {addr}");

    let (prom_layer, prom_handler) = PrometheusMetricLayer::pair();

    let router = Router::new()
        .route("/ping", get(|| async move { "pong" }))
        .route("/metrics", get(|| async move { prom_handler.render() }))
        .route("/messages", get(handle_message).post(handle_message))
        // this makes the layer only apply when a route is matched, instead of '.layer()'
        .route_layer(
            ServiceBuilder::new()
                .layer(prom_layer)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(DefaultOnResponse::new().level(Level::INFO))
                        .on_eos(()) // disable it
                        .on_body_chunk(()), // disable it
                )
                .layer(CorsLayer::permissive()),
        );

    axum::serve(listener, router)
        .with_graceful_shutdown(graceful_shutdown())
        .await
        .inspect_err(|err| info!("{err:?}"))?;

    Ok(())
}

async fn graceful_shutdown() {
    let _ = tokio::signal::ctrl_c().await;
}
