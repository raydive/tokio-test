use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::TracerProvider;
use opentelemetry_stdout as stdout;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

pub fn init_provider() -> TracerProvider {
    let exporter = stdout::SpanExporter::default();
    TracerProvider::builder()
        .with_simple_exporter(exporter)
        .build()
}

pub fn set_telemetry(provider: &TracerProvider) {
    let telemetry = tracing_opentelemetry::layer().with_tracer(provider.tracer("example-tracer"));
    let json = tracing_subscriber::fmt::layer().json();
    let subscriber = Registry::default().with(telemetry).with(json);
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
