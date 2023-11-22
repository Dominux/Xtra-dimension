use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Form, Router,
};

use crate::{
    common::{
        jwt_manager::AuthUser,
        routing::{app_state::AppState, middlewares::auth::logged_in_required},
    },
    errors::{PentaractError, PentaractResult},
    schemas::storages::InStorageSchema,
    services::storages::StoragesService,
    templates::storages::{
        create_form::StoragesCreateFormTemplate,
        index::{StoragesIndexTemplate, StoragesListTemplate},
    },
};

use super::{auth::AuthRouter, files::FilesRouter};

pub struct StoragesRouter;

impl StoragesRouter {
    pub fn get_router(state: Arc<AppState>) -> Router {
        let files_router = FilesRouter::get_router(state.clone());
        Router::new()
            .route("/", get(Self::index).post(Self::create))
            .route("/list", get(Self::list))
            .route("/create", get(Self::get_create_form))
            .nest("/:storage_id/files", files_router)
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                logged_in_required,
            ))
            .with_state(state)
    }

    async fn index(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
    ) -> impl IntoResponse {
        let list_result = StoragesService::new(&state.db).list(&user).await;
        match list_result {
            Ok(s) => Html(StoragesIndexTemplate::new(s).render().unwrap()).into_response(),
            Err(e) => <(StatusCode, String)>::from(e).into_response(),
        }
    }

    async fn get_create_form() -> impl IntoResponse {
        Html(StoragesCreateFormTemplate::default().render().unwrap())
    }

    async fn create(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
        Form(in_schema): Form<InStorageSchema>,
    ) -> impl IntoResponse {
        let service = StoragesService::new(&state.db);

        if let Err(e) = service.create(in_schema, &user).await {
            return match e {
                PentaractError::StorageNameConflict => (
                    StatusCode::CONFLICT,
                    Html(
                        StoragesCreateFormTemplate::new(Some("This name isn't unique"))
                            .render()
                            .unwrap(),
                    ),
                )
                    .into_response(),
                PentaractError::UserWasRemoved => {
                    AuthRouter::logout_for_htmx().await.into_response()
                }
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response(),
            };
        };

        match Self::_list(service, &user).await {
            Ok(page) => (StatusCode::CREATED, page).into_response(),
            Err(e) => <(StatusCode, String)>::from(e).into_response(),
        }
    }

    async fn list(
        State(state): State<Arc<AppState>>,
        Extension(user): Extension<AuthUser>,
    ) -> impl IntoResponse {
        let service = StoragesService::new(&state.db);
        match Self::_list(service, &user).await {
            Ok(page) => page.into_response(),
            Err(e) => <(StatusCode, String)>::from(e).into_response(),
        }
    }

    async fn _list<'a>(
        service: StoragesService<'a>,
        user: &AuthUser,
    ) -> PentaractResult<Html<String>> {
        service
            .list(&user)
            .await
            .map(|s| Html(StoragesListTemplate::new(s).render().unwrap()))
    }
}
