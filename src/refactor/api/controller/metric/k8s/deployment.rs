use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde_json::Value;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::RangeQuery;
use crate::refactor::k8s::{By, K8sActor, K8sActorMessage, Target};
use crate::refactor::state::{
    MetricServiceMessage, StateActor, StateActorMessage, StateActorResult,
};

pub struct K8sDeploymentMetricsController;

impl K8sDeploymentMetricsController {
    pub async fn get_metric_k8s_deployments_raw(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let deployment_names = get_deployments(&mut state).await;
        to_json(get_metric_k8s_deployments_raw(&mut state, q, deployment_names).await)
    }

    pub async fn get_metric_k8s_deployments_raw_summary(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let deployment_names = get_deployments(&mut state).await;
        to_json(get_metric_k8s_deployments_raw_summary(&mut state, q, deployment_names).await)
    }

    pub async fn get_metric_k8s_deployments_raw_efficiency(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let deployment_names = get_deployments(&mut state).await;
        to_json(get_metric_k8s_deployments_raw_efficiency(&mut state, q, deployment_names).await)
    }

    pub async fn get_metric_k8s_deployment_raw(
        State(mut state): State<ActorSystem>,
        Path(deployment): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_deployment_raw(&mut state, deployment, q).await)
    }

    pub async fn get_metric_k8s_deployment_raw_summary(
        State(mut state): State<ActorSystem>,
        Path(deployment): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_deployment_raw_summary(&mut state, deployment, q).await)
    }

    pub async fn get_metric_k8s_deployment_raw_efficiency(
        State(mut state): State<ActorSystem>,
        Path(deployment): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_deployment_raw_efficiency(&mut state, deployment, q).await)
    }

    pub async fn get_metric_k8s_deployments_cost(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let deployment_names = get_deployments(&mut state).await;
        to_json(get_metric_k8s_deployments_cost(&mut state, q, deployment_names).await)
    }

    pub async fn get_metric_k8s_deployments_cost_summary(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let deployment_names = get_deployments(&mut state).await;
        to_json(get_metric_k8s_deployments_cost_summary(&mut state, q, deployment_names).await)
    }

    pub async fn get_metric_k8s_deployments_cost_trend(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let deployment_names = get_deployments(&mut state).await;
        to_json(get_metric_k8s_deployments_cost_trend(&mut state, q, deployment_names).await)
    }

    pub async fn get_metric_k8s_deployment_cost(
        State(mut state): State<ActorSystem>,
        Path(deployment): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_deployment_cost(&mut state, deployment, q).await)
    }

    pub async fn get_metric_k8s_deployment_cost_summary(
        State(mut state): State<ActorSystem>,
        Path(deployment): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_deployment_cost_summary(&mut state, deployment, q).await)
    }

    pub async fn get_metric_k8s_deployment_cost_trend(
        State(mut state): State<ActorSystem>,
        Path(deployment): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_deployment_cost_trend(&mut state, deployment, q).await)
    }
}

async fn get_deployments(state: &mut ActorSystem) -> Vec<String> {
    state
        .send_and_recv::<K8sActor>(
            k8s_actor_addr!().to_string(),
            K8sActorMessage::Deployments(Target::All, By::None),
        )
        .await
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect()
}

async fn get_metric_k8s_deployments_raw(
    state: &mut ActorSystem,
    q: RangeQuery,
    deployment_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sDeploymentsRaw(
                q,
                deployment_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_deployments_raw_summary(
    state: &mut ActorSystem,
    q: RangeQuery,
    deployment_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sDeploymentsRawSummary(
                q,
                deployment_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_deployments_raw_efficiency(
    state: &mut ActorSystem,
    q: RangeQuery,
    deployment_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sDeploymentsRawEfficiency(
                q,
                deployment_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_deployment_raw(
    state: &mut ActorSystem,
    deployment: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sDeploymentRaw(
                deployment, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_deployment_raw_summary(
    state: &mut ActorSystem,
    deployment: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sDeploymentRawSummary(
                deployment, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_deployment_raw_efficiency(
    state: &mut ActorSystem,
    deployment: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sDeploymentRawEfficiency(
                deployment, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_deployments_cost(
    state: &mut ActorSystem,
    q: RangeQuery,
    deployment_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sDeploymentsCost(
                q,
                deployment_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_deployments_cost_summary(
    state: &mut ActorSystem,
    q: RangeQuery,
    deployment_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sDeploymentsCostSummary(
                q,
                deployment_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_deployments_cost_trend(
    state: &mut ActorSystem,
    q: RangeQuery,
    deployment_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sDeploymentsCostTrend(
                q,
                deployment_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_deployment_cost(
    state: &mut ActorSystem,
    deployment: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sDeploymentCost(
                deployment, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_deployment_cost_summary(
    state: &mut ActorSystem,
    deployment: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sDeploymentCostSummary(
                deployment, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_deployment_cost_trend(
    state: &mut ActorSystem,
    deployment: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sDeploymentCostTrend(
                deployment, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}
