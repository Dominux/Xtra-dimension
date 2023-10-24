use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Router};
use tokio::{sync::oneshot, time};
use tower::limit::ConcurrencyLimitLayer;

use crate::{
    common::{channels::ClientSender, routing::app_state::AppState},
    routers::{auth::AuthRouter, storage_workers::StorageWorkersRouter},
};

pub struct Server {
    router: Router,
}

impl Server {
    pub fn build_server(workers: usize, app_state: Arc<AppState>, tx: ClientSender) -> Self {
        let router = Router::new()
            .route(
                "/",
                get(|| async move {
                    let (resp_tx, resp_rx) = oneshot::channel();

                    tracing::debug!("started");
                    let _ = tx.send(resp_tx).await;

                    // simulating some io operations
                    time::sleep(time::Duration::from_millis(500)).await;

                    resp_rx.await.unwrap()
                }),
            )
            .nest("/auth", AuthRouter::get_router(app_state.clone()))
            .nest(
                "/storage_workers",
                StorageWorkersRouter::get_router(app_state.clone()),
            )
            .layer(ConcurrencyLimitLayer::new(workers.into()));

        Self { router }
    }

    pub async fn run(self, addr: &SocketAddr) {
        tracing::debug!("listening on http://{addr}");
        axum::Server::bind(addr)
            .serve(self.router.into_make_service())
            .await
            .unwrap();
    }
}
