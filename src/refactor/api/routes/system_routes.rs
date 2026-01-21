//! System routes (e.g., /api/v1/system/*)

use crate::refactor::api::controller::*;
use axum::{
    routing::{get, post},
    Router,
};
use xan_actor::ActorSystem;

pub fn system_routes() -> Router<ActorSystem> {
    Router::new()
        .route("/status", get(SystemController::status))
        .route("/health", get(SystemController::health))
        .route("/backup", post(SystemController::backup))
        .route("/resync", post(SystemController::resync))
        .route("/logs/{date}", get(SystemController::get_system_log_lines))
        .route("/logs", get(SystemController::get_system_log_file_list))
}
