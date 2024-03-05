use std::sync::Arc;

use axum::routing::IntoMakeService;
use axum::serve::Serve;
use axum::Router;
use tokio::net::TcpListener;

use crate::interaction_sessions::InteractionSessions;
use crate::routes::html_pages_and_triggers::html_pages_and_triggers_router;
use crate::ws_tracing_subscriber::WebSocketAppender;
use crate::{
    bank::Bank,
    config::Settings,
    routes::{api::api_router, system::system_router},
};

type Server = Serve<IntoMakeService<Router>, Router>;

pub struct Application {
    _port: u16,
    server: Server,
}

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub bank: Bank,
    pub interaction_sessions: InteractionSessions,
    pub ws_appender: WebSocketAppender,
}

impl Application {
    pub async fn build(
        config: Settings,
        ws_appender: WebSocketAppender,
    ) -> Result<Self, anyhow::Error> {
        let port = config.port;
        let addr = format!("{}:{}", config.addr, port);
        let listener = TcpListener::bind(addr).await?;

        // Notificator is mpsc::Receiver which is notified
        // when there are new bank request.
        let bank = match config.data_backend_type {
            crate::config::DataBackendType::Pg => {
                Bank::new::<crate::bank::pg::PostgresStorage>(&config)
            }
            crate::config::DataBackendType::Mem => {
                Bank::new::<crate::bank::memory::MemoryStorage>(&config)
            }
        };

        let app_state = AppState {
            bank,
            settings: Arc::new(config),
            interaction_sessions: InteractionSessions::new(),
            ws_appender,
        };

        let app = html_pages_and_triggers_router()
            .with_state(app_state.clone())
            .nest("/api", api_router(app_state.clone()))
            .nest("/system", system_router(app_state.clone()));

        let server = axum::serve(listener, app.into_make_service());

        Ok(Self {
            _port: port,
            server,
        })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server
            .with_graceful_shutdown(shutdown_signal())
            .await?;
        Ok(())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    let terminate = async {
        tokio::signal::unix::signal(
            tokio::signal::unix::SignalKind::terminate(),
        )
        .expect("failed to install signal handler")
        .recv()
        .await;
    };
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
    tracing::info!("Terminate signal received");
}
