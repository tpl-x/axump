use std::str::FromStr;

use tracing_appender::rolling;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tower_http::trace::{self, HttpMakeClassifier, TraceLayer};
use tracing::Level;


pub fn with_tracing(config: &crate::config::AppConfig) -> TraceLayer<HttpMakeClassifier> {
    let mut _tracing_level: Level = Level::INFO;
    if let Ok(level) = Level::from_str(config.log_level.as_str()) {
        _tracing_level = level
    }
    let info_file = rolling::daily(
        config.log_dir.clone(),
        config.log_name_prefix.clone(),
    )
        .with_max_level(_tracing_level);
    
    tracing_subscriber::fmt()
        .with_writer(info_file)
        .with_ansi(false)
        .init();


    TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(_tracing_level))
        .on_response(trace::DefaultOnResponse::new().level(_tracing_level))
}

