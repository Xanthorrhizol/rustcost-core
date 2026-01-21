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

pub struct K8sPodMetricsController;

impl K8sPodMetricsController {
    pub async fn get_metric_k8s_pods_raw(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let pod_uids = if let Some(key) = &q.key {
            vec![key.to_string()] // or whatever q.key represents
        } else {
            get_pods(&mut state).await
        };

        to_json(get_metric_k8s_pods_raw(&mut state, q, pod_uids).await)
    }

    pub async fn get_metric_k8s_pods_raw_summary(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let pod_uids = if let Some(key) = &q.key {
            vec![key.to_string()] // or whatever q.key represents
        } else {
            get_pods(&mut state).await
        };
        to_json(get_metric_k8s_pods_raw_summary(&mut state, q, pod_uids).await)
    }

    pub async fn get_metric_k8s_pods_raw_efficiency(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let pod_uids = if let Some(key) = &q.key {
            vec![key.to_string()] // or whatever q.key represents
        } else {
            get_pods(&mut state).await
        };
        to_json(get_metric_k8s_pods_raw_efficiency(&mut state, q, pod_uids).await)
    }

    pub async fn get_metric_k8s_pod_raw(
        State(mut state): State<ActorSystem>,
        Path(pod_uid): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_pod_raw(&mut state, pod_uid, q).await)
    }

    pub async fn get_metric_k8s_pod_raw_summary(
        State(mut state): State<ActorSystem>,
        Path(pod_uid): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_pod_raw_summary(&mut state, pod_uid, q).await)
    }

    pub async fn get_metric_k8s_pod_raw_efficiency(
        State(mut state): State<ActorSystem>,
        Path(pod_uid): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_pod_raw_efficiency(&mut state, pod_uid, q).await)
    }

    pub async fn get_metric_k8s_pods_cost(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let pod_uids = if let Some(key) = &q.key {
            vec![key.to_string()] // or whatever q.key represents
        } else {
            get_pods(&mut state).await
        };
        to_json(get_metric_k8s_pods_cost(&mut state, q, pod_uids).await)
    }

    pub async fn get_metric_k8s_pods_cost_summary(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let pod_uids = if let Some(key) = &q.key {
            vec![key.to_string()] // or whatever q.key represents
        } else {
            get_pods(&mut state).await
        };
        to_json(get_metric_k8s_pods_cost_summary(&mut state, q, pod_uids).await)
    }

    pub async fn get_metric_k8s_pods_cost_trend(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let pod_uids = if let Some(key) = &q.key {
            vec![key.to_string()] // or whatever q.key represents
        } else {
            get_pods(&mut state).await
        };
        to_json(get_metric_k8s_pods_cost_trend(&mut state, q, pod_uids).await)
    }

    pub async fn get_metric_k8s_pod_cost(
        State(mut state): State<ActorSystem>,
        Path(pod_uid): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_pod_cost(&mut state, pod_uid, q).await)
    }

    pub async fn get_metric_k8s_pod_cost_summary(
        State(mut state): State<ActorSystem>,
        Path(pod_uid): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_pod_cost_summary(&mut state, pod_uid, q).await)
    }

    pub async fn get_metric_k8s_pod_cost_trend(
        State(mut state): State<ActorSystem>,
        Path(pod_uid): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_pod_cost_trend(&mut state, pod_uid, q).await)
    }
}

async fn get_pods(state: &mut ActorSystem) -> Vec<String> {
    state
        .send_and_recv::<K8sActor>(
            k8s_actor_addr!().to_string(),
            K8sActorMessage::Pods(Target::All, By::None),
        )
        .await
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect()
}

async fn get_metric_k8s_pods_raw(
    state: &mut ActorSystem,
    q: RangeQuery,
    pod_uids: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sPodsRaw(q, pod_uids)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_pods_raw_summary(
    state: &mut ActorSystem,
    q: RangeQuery,
    pod_uids: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sPodsRawSummary(
                q, pod_uids,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_pods_raw_efficiency(
    state: &mut ActorSystem,
    q: RangeQuery,
    pod_uids: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sPodsRawEfficiency(
                q, pod_uids,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

pub async fn get_metric_k8s_pod_raw(
    state: &mut ActorSystem,
    pod_uid: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sPodRaw(pod_uid, q)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

pub async fn get_metric_k8s_pod_raw_summary(
    state: &mut ActorSystem,
    pod_uid: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sPodRawSummary(pod_uid, q)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

pub async fn get_metric_k8s_pod_raw_efficiency(
    state: &mut ActorSystem,
    pod_uid: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sPodRawEfficiency(
                pod_uid, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_pods_cost(
    state: &mut ActorSystem,
    q: RangeQuery,
    pod_uids: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sPodsCost(q, pod_uids)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_pods_cost_summary(
    state: &mut ActorSystem,
    q: RangeQuery,
    pod_uids: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sPodsCostSummary(
                q, pod_uids,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_pods_cost_trend(
    state: &mut ActorSystem,
    q: RangeQuery,
    pod_uids: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sPodsCostTrend(q, pod_uids)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_pod_cost(
    state: &mut ActorSystem,
    pod_uid: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sPodCost(pod_uid, q)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_pod_cost_summary(
    state: &mut ActorSystem,
    pod_uid: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sPodCostSummary(pod_uid, q)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_pod_cost_trend(
    state: &mut ActorSystem,
    pod_uid: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sPodCostTrend(pod_uid, q)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}
