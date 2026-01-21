//! Stored info routes (backed by persisted data)

use crate::refactor::api::controller::*;
use axum::{
    routing::{get, patch},
    Router,
};
use xan_actor::ActorSystem;

pub fn info_stored_routes() -> Router<ActorSystem> {
    Router::new()
        .route(
            "/settings",
            get(InfoSettingController::get_info_settings)
                .put(InfoSettingController::upsert_info_settings),
        )
        .route(
            "/alerts",
            get(InfoAlertController::get_info_alerts).put(InfoAlertController::upsert_info_alerts),
        )
        .route(
            "/llm",
            get(InfoLlmController::get_info_llm).put(InfoLlmController::upsert_info_llm),
        )
        .route(
            "/unit-prices",
            get(InfoController::get_info_unit_prices).put(InfoController::upsert_info_unit_prices),
        )
        .route("/versions", get(InfoController::get_info_versions))
        .route(
            "/k8s/store/nodes",
            get(InfoK8sNodeController::list_k8s_nodes),
        )
        .route("/k8s/store/pods", get(InfoK8sPodController::list_k8s_pods))
        .route(
            "/k8s/store/containers",
            get(InfoK8sContainerController::list_k8s_containers),
        )
        .route(
            "/k8s/store/nodes/{node_name}",
            get(InfoK8sNodeController::get_info_k8s_node),
        )
        .route(
            "/k8s/store/pods/{pod_uid}",
            get(InfoK8sPodController::get_info_k8s_pod),
        )
        .route(
            "/k8s/store/containers/{id}",
            get(InfoK8sContainerController::get_info_k8s_container),
        )
        .route(
            "/k8s/store/nodes/{node_name}/filter",
            patch(InfoK8sNodeController::patch_info_k8s_node_filter),
        )
        .route(
            "/k8s/store/nodes/{node_name}/price",
            patch(InfoK8sNodeController::patch_info_k8s_node_price),
        )
        .route(
            "/k8s/store/pods/{pod_uid}",
            patch(InfoK8sPodController::patch_info_k8s_pod),
        )
        .route(
            "/k8s/store/containers/{id}",
            patch(InfoK8sContainerController::patch_info_k8s_container),
        )
}
