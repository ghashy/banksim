use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

use acquisim::ws_tracing_subscriber::WebSocketAppender;
use acquisim::Application;
use acquisim::Settings;

#[tokio::main]
async fn main() {
    let (appender, rx) = WebSocketAppender::new();
    appender.run(rx);
    let (non_blocking, _guard) =
        tracing_appender::non_blocking(appender.clone());

    // Layer 1 is websocket appender
    let layer1 = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::default())
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .compact()
        .with_level(true)
        .with_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(Level::INFO.into())
                .add_directive("axum::rejection=trace".parse().unwrap())
                .add_directive("tower_sessions_core=warn".parse().unwrap())
                .add_directive("aws_config=warn".parse().unwrap()),
        );

    // Layer 2 is the stdout writer
    let (non_blocking, _guard) =
        tracing_appender::non_blocking(std::io::stdout());
    let layer2 = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::default())
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .compact()
        .with_level(true)
        .with_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(Level::INFO.into())
                .add_directive("axum::rejection=trace".parse().unwrap())
                .add_directive("tower_sessions_core=warn".parse().unwrap())
                .add_directive("aws_config=warn".parse().unwrap()),
        );

    tracing_subscriber::registry()
        .with(layer1)
        .with(layer2)
        .init();

    let settings = Settings::load_configuration().unwrap();

    if let Err(e) = Application::build(settings, appender)
        .await
        .expect("Failed to build application")
        .run_until_stopped()
        .await
    {
        eprintln!("Error: {}", e);
    }
}
