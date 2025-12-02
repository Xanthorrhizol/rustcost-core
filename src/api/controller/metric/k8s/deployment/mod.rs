use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde_json::Value;

use crate::api::util::json::to_json;
use crate::api::dto::{metrics_dto::RangeQuery, ApiResponse};
use crate::app_state::AppState;
use crate::errors::AppError;

pub struct K8sDeploymentMetricsController;

impl K8sDeploymentMetricsController {
    pub async fn get_metric_k8s_deployments_raw(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        let deployment_names = state.k8s_state.get_deployments().await;
        to_json(
            state
                .metric_service
                .get_metric_k8s_deployments_raw(q, deployment_names)
                .await,
        )
    }

    pub async fn get_metric_k8s_deployments_raw_summary(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        let deployment_names = state.k8s_state.get_deployments().await;
        to_json(
            state
                .metric_service
                .get_metric_k8s_deployments_raw_summary(q, deployment_names)
                .await,
        )
    }

    pub async fn get_metric_k8s_deployments_raw_efficiency(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        let deployment_names = state.k8s_state.get_deployments().await;
        to_json(
            state
                .metric_service
                .get_metric_k8s_deployments_raw_efficiency(q, deployment_names)
                .await,
        )
    }

    pub async fn get_metric_k8s_deployment_raw(
        State(state): State<AppState>,
        Path(deployment): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        to_json(
            state
                .metric_service
                .get_metric_k8s_deployment_raw(deployment, q)
                .await,
        )
    }

    pub async fn get_metric_k8s_deployment_raw_summary(
        State(state): State<AppState>,
        Path(deployment): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        to_json(
            state
                .metric_service
                .get_metric_k8s_deployment_raw_summary(deployment, q)
                .await,
        )
    }

    pub async fn get_metric_k8s_deployment_raw_efficiency(
        State(state): State<AppState>,
        Path(deployment): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        to_json(
            state
                .metric_service
                .get_metric_k8s_deployment_raw_efficiency(deployment, q)
                .await,
        )
    }

    pub async fn get_metric_k8s_deployments_cost(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        let deployment_names = state.k8s_state.get_deployments().await;
        to_json(
            state
                .metric_service
                .get_metric_k8s_deployments_cost(q, deployment_names)
                .await,
        )
    }

    pub async fn get_metric_k8s_deployments_cost_summary(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        let deployment_names = state.k8s_state.get_deployments().await;
        to_json(
            state
                .metric_service
                .get_metric_k8s_deployments_cost_summary(q, deployment_names)
                .await,
        )
    }

    pub async fn get_metric_k8s_deployments_cost_trend(
        State(state): State<AppState>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        let deployment_names = state.k8s_state.get_deployments().await;
        to_json(
            state
                .metric_service
                .get_metric_k8s_deployments_cost_trend(q, deployment_names)
                .await,
        )
    }

    pub async fn get_metric_k8s_deployment_cost(
        State(state): State<AppState>,
        Path(deployment): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        to_json(
            state
                .metric_service
                .get_metric_k8s_deployment_cost(deployment, q)
                .await,
        )
    }

    pub async fn get_metric_k8s_deployment_cost_summary(
        State(state): State<AppState>,
        Path(deployment): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        to_json(
            state
                .metric_service
                .get_metric_k8s_deployment_cost_summary(deployment, q)
                .await,
        )
    }

    pub async fn get_metric_k8s_deployment_cost_trend(
        State(state): State<AppState>,
        Path(deployment): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        state.k8s_state.ensure_resynced().await?;
        to_json(
            state
                .metric_service
                .get_metric_k8s_deployment_cost_trend(deployment, q)
                .await,
        )
    }
}
