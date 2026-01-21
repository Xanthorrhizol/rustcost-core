use axum::extract::{Query, State};
use axum::Json;
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

pub struct K8sClusterMetricsController;

impl K8sClusterMetricsController {
    pub async fn get_metric_k8s_cluster_raw(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let node_names = get_nodes(&mut state).await;

        to_json(get_metric_k8s_cluster_raw(&mut state, q, node_names).await)
    }

    pub async fn get_metric_k8s_cluster_raw_summary(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let node_names = get_nodes(&mut state).await;

        to_json(get_metric_k8s_cluster_raw_summary(&mut state, q, node_names).await)
    }

    pub async fn get_metric_k8s_cluster_cost(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let node_names = get_nodes(&mut state).await;

        to_json(get_metric_k8s_cluster_cost(&mut state, q, node_names).await)
    }

    pub async fn get_metric_k8s_cluster_cost_summary(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let node_names = get_nodes(&mut state).await;

        to_json(get_metric_k8s_cluster_cost_summary(&mut state, q, node_names).await)
    }

    pub async fn get_metric_k8s_cluster_cost_trend(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let node_names = get_nodes(&mut state).await;

        to_json(get_metric_k8s_cluster_cost_trend(&mut state, q, node_names).await)
    }

    pub async fn get_metric_k8s_cluster_raw_efficiency(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let node_names = get_nodes(&mut state).await;

        to_json(get_metric_k8s_cluster_raw_efficiency(&mut state, q, node_names).await)
    }
}

async fn get_nodes(state: &mut ActorSystem) -> Vec<String> {
    state
        .send_and_recv::<K8sActor>(
            k8s_actor_addr!().to_string(),
            K8sActorMessage::Nodes(Target::All, By::None),
        )
        .await
        .expect("failed to get nodes")
        .as_array()
        .expect("failed to get nodes")
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect()
}

async fn get_metric_k8s_cluster_raw(
    state: &mut ActorSystem,
    q: RangeQuery,
    node_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricsK8sClusterRaw(q, node_names)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_cluster_raw_summary(
    state: &mut ActorSystem,
    q: RangeQuery,
    node_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricsK8sClusterRawSummary(
                q, node_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_cluster_cost(
    state: &mut ActorSystem,
    q: RangeQuery,
    node_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricsK8sClusterCost(
                q, node_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_cluster_cost_summary(
    state: &mut ActorSystem,
    q: RangeQuery,
    node_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricsK8sClusterCostSummary(
                q, node_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_cluster_cost_trend(
    state: &mut ActorSystem,
    q: RangeQuery,
    node_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricsK8sClusterCostTrend(
                q, node_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_cluster_raw_efficiency(
    state: &mut ActorSystem,
    q: RangeQuery,
    node_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricsK8sClusterRawEfficiency(
                q, node_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}
