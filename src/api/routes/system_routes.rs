//! System routes (e.g., /api/v1/system/*)

use axum::{routing::{get, post}, Router};
use crate::api::controller::system as sc;
use crate::api::controller::system::{get_system_log_file_list, get_system_log_lines};
use crate::app_state::AppState;

pub fn system_routes() -> Router<AppState> {
    Router::new()
        .route("/status", get(sc::status))
        .route("/health", get(sc::health))
        .route("/backup", post(sc::backup))
        .route("/resync", post(sc::resync))

        .route("/logs/{date}", get(get_system_log_lines))
        .route("/logs", get(get_system_log_file_list))
}
