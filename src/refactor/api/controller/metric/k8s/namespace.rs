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

pub struct K8sNamespaceMetricsController;

impl K8sNamespaceMetricsController {
    pub async fn get_metric_k8s_namespaces_raw(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let ns_names = get_namespaces(&mut state).await;
        to_json(get_metric_k8s_namespaces_raw(&mut state, q, ns_names).await)
    }

    pub async fn get_metric_k8s_namespaces_raw_summary(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let ns_names = get_namespaces(&mut state).await;
        to_json(get_metric_k8s_namespaces_raw_summary(&mut state, q, ns_names).await)
    }

    pub async fn get_metric_k8s_namespaces_raw_efficiency(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let ns_names = get_namespaces(&mut state).await;
        to_json(get_metric_k8s_namespaces_raw_efficiency(&mut state, q, ns_names).await)
    }

    pub async fn get_metric_k8s_namespace_raw(
        State(mut state): State<ActorSystem>,
        Path(namespace): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_namespace_raw(&mut state, namespace, q).await)
    }

    pub async fn get_metric_k8s_namespace_raw_summary(
        State(mut state): State<ActorSystem>,
        Path(namespace): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_namespace_raw_summary(&mut state, namespace, q).await)
    }

    pub async fn get_metric_k8s_namespace_raw_efficiency(
        State(mut state): State<ActorSystem>,
        Path(namespace): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_namespace_raw_efficiency(&mut state, namespace, q).await)
    }

    pub async fn get_metric_k8s_namespaces_cost(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let ns_names = get_namespaces(&mut state).await;
        to_json(get_metric_k8s_namespaces_cost(&mut state, q, ns_names).await)
    }

    pub async fn get_metric_k8s_namespaces_cost_summary(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let ns_names = get_namespaces(&mut state).await;
        to_json(get_metric_k8s_namespaces_cost_summary(&mut state, q, ns_names).await)
    }

    pub async fn get_metric_k8s_namespaces_cost_trend(
        State(mut state): State<ActorSystem>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let ns_names = get_namespaces(&mut state).await;
        to_json(get_metric_k8s_namespaces_cost_trend(&mut state, q, ns_names).await)
    }

    pub async fn get_metric_k8s_namespace_cost(
        State(mut state): State<ActorSystem>,
        Path(namespace): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_namespace_cost(&mut state, namespace, q).await)
    }

    pub async fn get_metric_k8s_namespace_cost_summary(
        State(mut state): State<ActorSystem>,
        Path(namespace): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_namespace_cost_summary(&mut state, namespace, q).await)
    }

    pub async fn get_metric_k8s_namespace_cost_trend(
        State(mut state): State<ActorSystem>,
        Path(namespace): Path<String>,
        Query(q): Query<RangeQuery>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(get_metric_k8s_namespace_cost_trend(&mut state, namespace, q).await)
    }
}

async fn get_namespaces(state: &mut ActorSystem) -> Vec<String> {
    state
        .send_and_recv::<K8sActor>(
            k8s_actor_addr!().to_string(),
            K8sActorMessage::Namespaces(Target::All, By::None),
        )
        .await
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect()
}

async fn get_metric_k8s_namespaces_raw(
    state: &mut ActorSystem,
    q: RangeQuery,
    ns_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNamespacesRaw(q, ns_names)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_namespaces_raw_summary(
    state: &mut ActorSystem,
    q: RangeQuery,
    ns_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNamespacesRawSummary(
                q, ns_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_namespaces_raw_efficiency(
    state: &mut ActorSystem,
    q: RangeQuery,
    ns_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNamespacesRawEfficiency(
                q, ns_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_namespace_raw(
    state: &mut ActorSystem,
    namespace: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNamespaceRaw(namespace, q)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_namespace_raw_summary(
    state: &mut ActorSystem,
    namespace: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNamespaceRawSummary(
                namespace, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_namespace_raw_efficiency(
    state: &mut ActorSystem,
    namespace: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNamespaceRawEfficiency(
                namespace, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_namespaces_cost(
    state: &mut ActorSystem,
    q: RangeQuery,
    ns_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNamespacesCost(
                q, ns_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_namespaces_cost_summary(
    state: &mut ActorSystem,
    q: RangeQuery,
    ns_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNamespacesCostSummary(
                q, ns_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_namespaces_cost_trend(
    state: &mut ActorSystem,
    q: RangeQuery,
    ns_names: Vec<String>,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNamespacesCostTrend(
                q, ns_names,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_namespace_cost(
    state: &mut ActorSystem,
    namespace: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNamespaceCost(
                namespace, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_namespace_cost_summary(
    state: &mut ActorSystem,
    namespace: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNamespaceCostSummary(
                namespace, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_metric_k8s_namespace_cost_trend(
    state: &mut ActorSystem,
    namespace: String,
    q: RangeQuery,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Metric(MetricServiceMessage::GetMetricK8sNamespaceCostTrend(
                namespace, q,
            )),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}
