use std::sync::Arc;

use axum::{
    extract::State,
    headers::{Cookie, HeaderMapExt},
    http::{HeaderMap, HeaderValue, Request},
    middleware::Next,
    response::{Redirect, Response},
};

use crate::{
    common::{
        constants::ACCESS_TOKEN_NAME,
        jwt_manager::{AuthUser, JWTManager},
        routing::app_state::AppState,
    },
    errors::{PentaractError, PentaractResult},
};

/// Middleware that requires to be loggen in
pub async fn logged_in_required<B>(
    State(state): State<Arc<AppState>>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, Redirect> {
    let auth_user = authenticate(req.headers(), &state.config.secret_key)
        .map_err(|_| Redirect::to("/auth/login"))?;

    req.extensions_mut().insert(auth_user);
    Ok(next.run(req).await)
}

/// Middleware that requires to be logged out
pub async fn logged_out_required<B>(
    State(state): State<Arc<AppState>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, Redirect> {
    match authenticate(req.headers(), &state.config.secret_key) {
        Ok(_) => Err(Redirect::to("/")),
        _ => Ok(next.run(req).await),
    }
}

#[inline]
fn authenticate(headers: &HeaderMap<HeaderValue>, secret_key: &str) -> PentaractResult<AuthUser> {
    let cookies = headers
        .typed_get::<Cookie>()
        .ok_or(PentaractError::NotAuthenticated)?;
    let token = cookies
        .get(ACCESS_TOKEN_NAME)
        .ok_or(PentaractError::NotAuthenticated)?;

    JWTManager::validate(token, secret_key)
}
