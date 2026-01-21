use axum::extract::{Path, Query, State};
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

pub struct K8sNodeMetricsController;

impl K8sNodeMetricsController {
    pub async fn get_metric_k8s_nodes_raw(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let node_names = get_nodes(&mut state).await;
        to_json(get_metric_k8s_nodes_raw(&mut state, q, node_names).await)
    }

    pub async fn get_metric_k8s_nodes_raw_summary(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let node_names = get_nodes(&mut state).await;
        to_json(get_metric_k8s_nodes_raw_summary(&mut state, q, node_names).await)
    }

    pub async fn get_metric_k8s_nodes_raw_efficiency(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let node_names = get_nodes(&mut state).await;
        to_json(get_metric_k8s_nodes_raw_efficiency(&mut state, q, node_names).await)
    }

    pub async fn get_metric_k8s_node_raw(
        State(mut state): State<ActorSystem>,
        Path(node_name): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_node_raw(&mut state, node_name, q).await)
    }

    pub async fn get_metric_k8s_node_raw_summary(
        State(mut state): State<ActorSystem>,
        Path(node_name): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_node_raw_summary(&mut state, node_name, q).await)
    }

    pub async fn get_metric_k8s_node_raw_efficiency(
        State(mut state): State<ActorSystem>,
        Path(node_name): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_node_raw_efficiency(&mut state, node_name, q).await)
    }

    pub async fn get_metric_k8s_nodes_cost(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let node_names = get_nodes(&mut state).await;
        to_json(get_metric_k8s_nodes_cost(&mut state, q, node_names).await)
    }

    pub async fn get_metric_k8s_nodes_cost_summary(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let node_names = get_nodes(&mut state).await;
        to_json(get_metric_k8s_nodes_cost_summary(&mut state, q, node_names).await)
    }

    pub async fn get_metric_k8s_nodes_cost_trend(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let node_names = get_nodes(&mut state).await;
        to_json(get_metric_k8s_nodes_cost_trend(&mut state, q, node_names).await)
    }

    pub async fn get_metric_k8s_node_cost(
        State(mut state): State<ActorSystem>,
        Path(node_name): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_node_cost(&mut state, node_name, q).await)
    }

    pub async fn get_metric_k8s_node_cost_summary(
        State(mut state): State<ActorSystem>,
        Path(node_name): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_node_cost_summary(&mut state, node_name, q).await)
    }

    pub async fn get_metric_k8s_node_cost_trend(
        State(mut state): State<ActorSystem>,
        Path(node_name): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_node_cost_trend(&mut state, node_name, q).await)
    }
}

async fn get_nodes(state: &mut ActorSystem) -> Vec<String> {
    state
        .send_and_recv::<K8sActor>(
            k8s_actor_addr!().to_string(),
            K8sActorMessage::Nodes(Target::All, By::None),
        )
        .await
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect()
}

async fn get_metric_k8s_nodes_raw(
    state: &mut ActorSystem,
    q: RangeQuery,
    node_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNodesRaw(q, node_names)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_nodes_raw_summary(
    state: &mut ActorSystem,
    q: RangeQuery,
    node_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNodesRawSummary(
                q, node_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_nodes_raw_efficiency(
    state: &mut ActorSystem,
    q: RangeQuery,
    node_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNodesRawEfficiency(
                q, node_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_node_raw(
    state: &mut ActorSystem,
    node_name: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNodeRaw(node_name, q)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_node_raw_summary(
    state: &mut ActorSystem,
    node_name: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNodeRawSummary(
                node_name, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_node_raw_efficiency(
    state: &mut ActorSystem,
    node_name: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNodeRawEfficiency(
                node_name, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_nodes_cost(
    state: &mut ActorSystem,
    q: RangeQuery,
    node_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNodesCost(q, node_names)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_nodes_cost_summary(
    state: &mut ActorSystem,
    q: RangeQuery,
    node_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNodesCostSummary(
                q, node_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_nodes_cost_trend(
    state: &mut ActorSystem,
    q: RangeQuery,
    node_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNodesCostTrend(
                q, node_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_node_cost(
    state: &mut ActorSystem,
    node_name: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNodeCost(node_name, q)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_node_cost_summary(
    state: &mut ActorSystem,
    node_name: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNodeCostSummary(
                node_name, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_node_cost_trend(
    state: &mut ActorSystem,
    node_name: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNodeCostTrend(
                node_name, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}
