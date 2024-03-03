use anyhow::Context;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::{extract::Request, response::Response};
use base64::Engine;
use futures::future::BoxFuture;
use secrecy::Secret;
use std::task::Poll;
use tower::{Layer, Service};

use crate::startup::AppState;

pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}

#[derive(Clone)]
pub struct BasicAuthLayer {
    pub state: AppState,
}

impl<S> Layer<S> for BasicAuthLayer {
    type Service = BasicAuth<S>;

    fn layer(&self, inner: S) -> Self::Service {
        BasicAuth {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct BasicAuth<S> {
    inner: S,
    state: AppState,
}

impl<S> Service<Request> for BasicAuth<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let headers = request.headers();
        let credentials = basic_authentication(headers);
        let state = self.state.clone();
        let future = self.inner.call(request);
        Box::pin(async move {
            let state = state;
            match credentials {
                Ok(cred) => {
                    match state.bank.handler().await.authorize_system(cred) {
                        Ok(()) => {
                            tracing::info!("Basic auth passed");
                            let response: Response = future.await?;
                            Ok(response)
                        }
                        Err(e) => {
                            tracing::info!("Failed to authorize: {}", e);
                            Ok(StatusCode::UNAUTHORIZED.into_response())
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to authorize: {e}");
                    Ok(StatusCode::UNAUTHORIZED.into_response())
                }
            }
        })
    }
}

fn basic_authentication(
    headers: &HeaderMap,
) -> Result<Credentials, anyhow::Error> {
    let header_value = headers
        .get("Authorization")
        .context("The 'Authorization' header was missing")?
        .to_str()
        .context("The 'Authorization' header was not a valid UTF8 string.")?;
    let base64encoded_segment = header_value
        .strip_prefix("Basic")
        .context("The authorization scheme was not 'Basic")?
        .trim();
    let decoded_bytes = base64::engine::general_purpose::STANDARD
        .decode(base64encoded_segment)
        .context("The decoded credential string is not a valid UTF 8.")?;
    let decoded_credentials = String::from_utf8(decoded_bytes)
        .context("The decoded credential string is not valid UTF8")?;

    // Split into two segments using ':' as delimiter
    let mut credentials = decoded_credentials.splitn(2, ':');
    let username = credentials
        .next()
        .ok_or_else(|| {
            anyhow::anyhow!("A username must be provided in 'Basic' auth.")
        })?
        .to_string();
    let password = credentials
        .next()
        .ok_or_else(|| {
            anyhow::anyhow!("A password must be provided in 'Basic' auth.")
        })?
        .to_string();
    Ok(Credentials {
        username,
        password: Secret::new(password),
    })
}
