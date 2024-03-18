use tracing::trace;
use crate::config::AppConfig;

mod config;
mod router;
mod middleware;

#[tokio::main]
async fn main() {
    let _app_config_parse = config::parse_config(None).await;
    let mut app_config: AppConfig = AppConfig::new();
    match _app_config_parse {
        Ok(config) => {
            tracing::info!("load config success: {:?}", config);
            app_config = config;
        }
        Err(e) => {
            tracing::error!("parse config failed:{},use the created one instead", e);
        }
    }

    let mut app = router::setup_routes();

    app = app
        .layer(middleware::with_cors())
        .layer(middleware::with_tracing(&app_config))
    ;

    let bind_addr = format!("{}:{}", app_config.host, app_config.port);
    let listener = tokio::net::TcpListener::bind(bind_addr).await;
    match listener {
        Ok(ls) => {
            tracing::info!(
                "server start success, now running at {}",
                ls.local_addr().unwrap(),);
            axum::serve(ls, app).await.expect("start server failed")
        }
        Err(e) => {
            tracing::error!("create tcp binding failed:{}", e);
        }
    }
}