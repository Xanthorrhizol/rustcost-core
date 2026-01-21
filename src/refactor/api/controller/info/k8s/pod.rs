use axum::extract::{Path, Query, State};
use axum::Json;
use k8s_openapi::api::core::v1::Pod;
use serde_json::Value;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::InfoK8sPodPatchRequest;
use crate::refactor::dto::{K8sPodQueryRequest, PaginatedResponse, PaginationQuery};
use crate::refactor::entity::InfoPodEntity;
use crate::refactor::state::{
    InfoK8sServiceMessage, StateActor, StateActorMessage, StateActorResult,
};

pub struct InfoK8sPodController;
pub struct InfoK8sLivePodController;

impl InfoK8sPodController {
    pub async fn get_info_k8s_pod(
        State(mut state): State<ActorSystem>,
        Path(pod_uid): Path<String>,
    ) -> Result<Json<ApiResponse<InfoPodEntity>>, AppError> {
        to_json(get_info_k8s_pod(&mut state, pod_uid).await)
    }

    /// List pods – optionally filter by `namespace`, `labelSelector`, or `nodeName`
    pub async fn list_k8s_pods(
        State(mut state): State<ActorSystem>,
        Query(filter): Query<K8sPodQueryRequest>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<InfoPodEntity>>>, AppError> {
        to_json(list_k8s_pods(&mut state, filter).await)
    }

    pub async fn patch_info_k8s_pod(
        State(mut state): State<ActorSystem>,
        Path(id): Path<String>,
        Json(payload): Json<InfoK8sPodPatchRequest>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(patch_info_k8s_pod(&mut state, id, payload).await)
    }
}

impl InfoK8sLivePodController {
    pub async fn list_k8s_pods(
        State(mut state): State<ActorSystem>,
        Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<Pod>>>, AppError> {
        to_json(get_k8s_live_pods_paginated(&mut state, pagination.limit, pagination.offset).await)
    }

    pub async fn get_k8s_pod(
        Path(pod_uid): Path<String>,
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Pod>>, AppError> {
        to_json(get_k8s_live_pod(&mut state, pod_uid).await)
    }
}

async fn get_info_k8s_pod(
    state: &mut ActorSystem,
    pod_uid: String,
) -> anyhow::Result<InfoPodEntity> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetInfoK8sPod(pod_uid)),
        )
        .await?
    {
        StateActorResult::InfoPodEntity(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn list_k8s_pods(
    state: &mut ActorSystem,
    filter: K8sPodQueryRequest,
) -> anyhow::Result<PaginatedResponse<InfoPodEntity>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::ListK8sPods(filter)),
        )
        .await?
    {
        StateActorResult::PaginatedInfoPodEntity(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn patch_info_k8s_pod(
    state: &mut ActorSystem,
    id: String,
    payload: InfoK8sPodPatchRequest,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::PatchInfoK8sPod(id, payload)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_k8s_live_pods_paginated(
    state: &mut ActorSystem,
    limit: Option<usize>,
    offset: Option<usize>,
) -> anyhow::Result<PaginatedResponse<Pod>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sLivePodsPaginated(
                limit, offset,
            )),
        )
        .await?
    {
        StateActorResult::PaginatedPod(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_k8s_live_pod(state: &mut ActorSystem, pod_uid: String) -> anyhow::Result<Pod> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sLivePod(pod_uid)),
        )
        .await?
    {
        StateActorResult::Pod(v) => Ok(v),
        _ => unreachable!(),
    }
}
