use std::collections::BTreeSet;
use std::sync::Arc;

use axum::routing::{self, IntoMakeService};
use axum::serve::Serve;
use axum::Router;
use http::{Method, StatusCode};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use crate::routes::html_pages_and_triggers::pages_and_triggers_router;
use crate::routes::session::session_router;
use crate::routes::token::token_router;
use crate::session::InteractionSessions;
use crate::ws_tracing_subscriber::WebSocketAppender;
use crate::{bank::Bank, config::Settings, routes::system::system_router};

type Server = Serve<IntoMakeService<Router>, Router>;

pub struct Application {
    _port: u16,
    server: Server,
}

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub bank: Bank,
    pub sessions: InteractionSessions,
    pub ws_appender: WebSocketAppender,
    pub http_client: reqwest::Client,
    pub ws_tokens: Arc<Mutex<BTreeSet<uuid::Uuid>>>,
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
            settings: Arc::new(config.clone()),
            sessions: InteractionSessions::new(),
            ws_appender,
            http_client: reqwest::Client::new(),
            ws_tokens: Arc::new(Mutex::new(BTreeSet::new())),
        };

        let cors = CorsLayer::new()
            // allow `GET` and `POST` when accessing the resource
            .allow_methods([Method::GET, Method::POST])
            // allow requests from any origin
            .allow_origin(Any);

        let app = pages_and_triggers_router()
            .nest("/token", token_router())
            .nest("/session", session_router())
            .nest("/system", system_router(app_state.clone()))
            .route("/healthcheck", routing::get(|| async { StatusCode::OK }))
            .with_state(app_state)
            .fallback_service(ServeDir::new(&config.frontend_path))
            .layer(cors);

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
