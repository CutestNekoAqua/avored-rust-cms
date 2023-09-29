use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::api::page::handlers::page_table_handler::page_table_handler;
use crate::avored_state::AvoRedState;

pub fn page_routes(state: Arc<AvoRedState>) -> Router {
    Router::new()
        .route("/admin/page", get(page_table_handler))

        // .route_layer(middleware::from_fn_with_state(
        //     state.clone(),
        //     require_authentication,
        // ))
        .with_state(state)
}