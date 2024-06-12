use axum;
use axum::{routing::get, Router};
use axum::response::Response;
use axum::extract::Request;
use axum::middleware::{self, Next};

use std::net::{Ipv4Addr, SocketAddr};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use metrics::{counter, gauge, histogram};

mod handlers;
mod models;
mod subscribers;

pub use handlers::{calculate, create_panic};
pub use subscribers::{init_subscriber, init_recorder};



async fn track(
    request: Request,
    next: Next,
) -> Response {
    // do something with `request`...

    counter!("invocations").increment(1);
    gauge!("current_requests").increment(1);

    let response = next.run(request).await;

    gauge!("current_requests").decrement(1);
    histogram!("request_duration").record(1);
    // do something with `response`...

    response
}


#[tokio::main]
async fn main() {
    init_subscriber();
    init_recorder();
    counter!("invocations").increment(1);

    let middleware = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO)),
    ); // Add high level tracing/logging to all requests

    let app = Router::new()
        .route("/", get(calculate))
        .route("/panic/", get(create_panic))
        .layer(middleware::from_fn(track))
        .layer(middleware);

    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, 5000));
    tracing::info!("Listening on {}", address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
