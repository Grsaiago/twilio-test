use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use std::error::Error;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer,
};
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .inspect_err(|err| info!("{err:?}"))?;

    let listening_ip = listener.local_addr()?.ip();
    let listening_port = listener.local_addr()?.port();

    info!("Listening on {}:{}", listening_ip, listening_port);

    let (prom_layer, prom_handler) = PrometheusMetricLayer::pair();

    let router = Router::new()
        .route("/ping", get(|| async move { "pong" }))
        .route("/metrics", get(|| async move { prom_handler.render() }))
        .layer(
            ServiceBuilder::new().layer(prom_layer).layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO))
                    .on_failure(DefaultOnFailure::new()),
            ),
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
