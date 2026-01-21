use crate::refactor::api::controller::*;
use axum::{
    routing::{get, post},
    Router,
};
use xan_actor::ActorSystem;

pub fn state_routes() -> Router<ActorSystem> {
    Router::new()
        // --- K8s Runtime State ---
        .route("/k8s", get(K8sStateController::get_full))
        .route("/k8s/summary", get(K8sStateController::get_summary))
        // --- Alerts Runtime State ---
        .route("/alerts", get(AlertStateController::get_active))
        .route("/alerts/all", get(AlertStateController::get_all))
        .route("/alerts/fire", post(AlertStateController::fire))
        .route("/alerts/resolve/{id}", post(AlertStateController::resolve))
}
