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

pub struct K8sContainerMetricsController;

impl K8sContainerMetricsController {
    pub async fn get_metric_k8s_containers_raw(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let container_keys = get_container_keys(&mut state).await;
        to_json(get_metric_k8s_containers_raw(&mut state, q, container_keys).await)
    }

    pub async fn get_metric_k8s_containers_raw_summary(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let container_keys = get_container_keys(&mut state).await;
        to_json(get_metric_k8s_containers_raw_summary(&mut state, q, container_keys).await)
    }

    pub async fn get_metric_k8s_containers_raw_efficiency(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let container_keys = get_container_keys(&mut state).await;
        to_json(get_metric_k8s_containers_raw_efficiency(&mut state, q, container_keys).await)
    }

    pub async fn get_metric_k8s_container_raw(
        State(mut state): State<ActorSystem>,
        Path(id): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_container_raw(&mut state, id, q).await)
    }

    pub async fn get_metric_k8s_container_raw_summary(
        State(mut state): State<ActorSystem>,
        Path(id): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_container_raw_summary(&mut state, id, q).await)
    }

    pub async fn get_metric_k8s_container_raw_efficiency(
        State(mut state): State<ActorSystem>,
        Path(id): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_container_raw_efficiency(&mut state, id, q).await)
    }

    pub async fn get_metric_k8s_containers_cost(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let container_keys = get_container_keys(&mut state).await;
        to_json(get_metric_k8s_containers_cost(&mut state, q, container_keys).await)
    }

    pub async fn get_metric_k8s_containers_cost_summary(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let container_keys = get_container_keys(&mut state).await;
        to_json(get_metric_k8s_containers_cost_summary(&mut state, q, container_keys).await)
    }

    pub async fn get_metric_k8s_containers_cost_trend(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let container_keys = get_container_keys(&mut state).await;
        to_json(get_metric_k8s_containers_cost_trend(&mut state, q, container_keys).await)
    }

    pub async fn get_metric_k8s_container_cost(
        State(mut state): State<ActorSystem>,
        Path(id): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_container_cost(&mut state, id, q).await)
    }

    pub async fn get_metric_k8s_container_cost_summary(
        State(mut state): State<ActorSystem>,
        Path(id): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_container_cost_summary(&mut state, id, q).await)
    }

    pub async fn get_metric_k8s_container_cost_trend(
        State(mut state): State<ActorSystem>,
        Path(id): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_container_cost_trend(&mut state, id, q).await)
    }
}

async fn get_container_keys(state: &mut ActorSystem) -> Vec<String> {
    state
        .send_and_recv::<K8sActor>(
            k8s_actor_addr!().to_string(),
            K8sActorMessage::Pods(Target::All, By::None),
        )
        .await
        .expect("failed to get containers")
        .as_array()
        .expect("failed to get containers")
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect()
}

async fn get_metric_k8s_containers_raw(
    state: &mut ActorSystem,
    q: RangeQuery,
    container_keys: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sContainersRaw(
                q,
                container_keys,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_containers_raw_summary(
    state: &mut ActorSystem,
    q: RangeQuery,
    container_keys: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sContainersRawSummary(
                q,
                container_keys,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_containers_raw_efficiency(
    state: &mut ActorSystem,
    q: RangeQuery,
    container_keys: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sContainersRawEfficiency(
                q,
                container_keys,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_container_raw(
    state: &mut ActorSystem,
    id: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sContainerRaw(id, q)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_container_raw_summary(
    state: &mut ActorSystem,
    id: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sContainerRawSummary(id, q)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_container_raw_efficiency(
    state: &mut ActorSystem,
    id: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sContainerRawEfficiency(
                id, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_containers_cost(
    state: &mut ActorSystem,
    q: RangeQuery,
    container_keys: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sContainersCost(
                q,
                container_keys,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_containers_cost_summary(
    state: &mut ActorSystem,
    q: RangeQuery,
    container_keys: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sContainersCostSummary(
                q,
                container_keys,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_containers_cost_trend(
    state: &mut ActorSystem,
    q: RangeQuery,
    container_keys: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sContainersCostTrend(
                q,
                container_keys,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_container_cost(
    state: &mut ActorSystem,
    id: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sContainerCost(id, q)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_container_cost_summary(
    state: &mut ActorSystem,
    id: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sContainerCostSummary(
                id, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_container_cost_trend(
    state: &mut ActorSystem,
    id: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sContainerCostTrend(id, q)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}
