use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Tracer;
use opentelemetry_sdk::{runtime, Resource};
use tonic::metadata::MetadataMap;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;
use std::net::{Ipv4Addr, SocketAddr};


pub fn init_subscriber() {
    let tracer = init_tracer();
    let otel = tracing_opentelemetry::layer().with_tracer(tracer);
    let filter = tracing_subscriber::EnvFilter::new(LevelFilter::INFO.to_string());
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .flatten_event(true);

    Registry::default()
        .with(filter)
        .with(fmt_layer)
        .with(otel)
        .init();

    std::panic::set_hook(Box::new(tracing_panic::panic_hook));
}

pub fn init_tracer() -> Tracer {
    let honeycomb_key =
        std::env::var("HONEYCOMB_API_KEY").expect("`HONEYCOMB_API_KEY` must be set");
    let mut map = MetadataMap::with_capacity(1);
    map.insert("x-honeycomb-team", honeycomb_key.try_into().unwrap());

    // Correctly configuring your exporter is a bit of a black art and highly-dependent on the
    // specifics of your deployment environment.
    // We won't go into the details here, but you can read more about it in the OpenTelemetry
    // documentation (or grab me after the workshop to talk about it).
    // At a super high-level: you want batching and you want a sensible sampling strategy,
    // but beyond that it's hard to give general advice.

    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(
            Resource::new(vec![KeyValue::new("service.name", "telemetry_prototype")]),
        ))
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("https://api.honeycomb.io/api/traces")
                .with_timeout(std::time::Duration::from_secs(5))
                .with_metadata(map),
        )
        .install_batch(runtime::Tokio)
        .unwrap()
}

pub fn init_recorder() {
    let listener_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 9091);
    let builder = metrics_exporter_prometheus::PrometheusBuilder::new();
    builder
        .with_http_listener(listener_addr)
        .install()
        .expect("failed to install Prometheus recorder");

    tracing::info!("Prometheus endpoint at {}", listener_addr);
}