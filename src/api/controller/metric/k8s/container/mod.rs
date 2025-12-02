use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde_json::Value;

use crate::api::util::json::to_json;
use crate::api::dto::{metrics_dto::RangeQuery, ApiResponse};
use crate::app_state::AppState;
use crate::errors::AppError;

pub struct K8sContainerMetricsController;

impl K8sContainerMetricsController {
    pub async fn get_metric_k8s_containers_raw(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        let container_keys = state.k8s_state.get_container_keys().await;
        to_json(
            state
                .metric_service
                .get_metric_k8s_containers_raw(q, container_keys)
                .await,
        )
    }

    pub async fn get_metric_k8s_containers_raw_summary(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        let container_keys = state.k8s_state.get_container_keys().await;
        to_json(
            state
                .metric_service
                .get_metric_k8s_containers_raw_summary(q, container_keys)
                .await,
        )
    }

    pub async fn get_metric_k8s_containers_raw_efficiency(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        let container_keys = state.k8s_state.get_container_keys().await;
        to_json(
            state
                .metric_service
                .get_metric_k8s_containers_raw_efficiency(q, container_keys)
                .await,
        )
    }

    pub async fn get_metric_k8s_container_raw(
        State(state): State<AppState>,
        Path(id): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        to_json(
            state
                .metric_service
                .get_metric_k8s_container_raw(id, q)
                .await,
        )
    }

    pub async fn get_metric_k8s_container_raw_summary(
        State(state): State<AppState>,
        Path(id): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        to_json(
            state
                .metric_service
                .get_metric_k8s_container_raw_summary(id, q)
                .await,
        )
    }

    pub async fn get_metric_k8s_container_raw_efficiency(
        State(state): State<AppState>,
        Path(id): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        to_json(
            state
                .metric_service
                .get_metric_k8s_container_raw_efficiency(id, q)
                .await,
        )
    }

    pub async fn get_metric_k8s_containers_cost(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        let container_keys = state.k8s_state.get_container_keys().await;
        to_json(
            state
                .metric_service
                .get_metric_k8s_containers_cost(q, container_keys)
                .await,
        )
    }

    pub async fn get_metric_k8s_containers_cost_summary(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        let container_keys = state.k8s_state.get_container_keys().await;
        to_json(
            state
                .metric_service
                .get_metric_k8s_containers_cost_summary(q, container_keys)
                .await,
        )
    }

    pub async fn get_metric_k8s_containers_cost_trend(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        let container_keys = state.k8s_state.get_container_keys().await;
        to_json(
            state
                .metric_service
                .get_metric_k8s_containers_cost_trend(q, container_keys)
                .await,
        )
    }

    pub async fn get_metric_k8s_container_cost(
        State(state): State<AppState>,
        Path(id): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        to_json(
            state
                .metric_service
                .get_metric_k8s_container_cost(id, q)
                .await,
        )
    }

    pub async fn get_metric_k8s_container_cost_summary(
        State(state): State<AppState>,
        Path(id): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        to_json(
            state
                .metric_service
                .get_metric_k8s_container_cost_summary(id, q)
                .await,
        )
    }

    pub async fn get_metric_k8s_container_cost_trend(
        State(state): State<AppState>,
        Path(id): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        to_json(
            state
                .metric_service
                .get_metric_k8s_container_cost_trend(id, q)
                .await,
        )
    }
}
