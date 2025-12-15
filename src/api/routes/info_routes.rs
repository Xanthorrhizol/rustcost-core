//! Info routes (e.g., /api/v1/info/*)

use axum::Router;
use crate::api::routes::{info_live_routes, info_stored_routes};
use crate::app_state::AppState;

pub fn info_routes() -> Router<AppState> {
    Router::new()
        .merge(info_stored_routes::info_stored_routes())
        .merge(info_live_routes::info_live_routes())
}
