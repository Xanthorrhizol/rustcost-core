use axum::extract::{Path, Query, State};
use axum::Json;
use serde_json::Value;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::{
    InfoK8sContainerPatchRequest, K8sListQuery, PaginatedResponse, PaginationQuery,
};
use crate::refactor::entity::InfoContainerEntity;
use crate::refactor::state::{
    InfoK8sServiceMessage, StateActor, StateActorMessage, StateActorResult,
};

pub struct InfoK8sContainerController;
pub struct InfoK8sLiveContainerController;

impl InfoK8sContainerController {
    pub async fn get_info_k8s_container(
        State(mut state): State<ActorSystem>,
        Path(id): Path<String>,
    ) -> Result<Json<ApiResponse<InfoContainerEntity>>, AppError> {
        to_json(get_info_k8s_container(&mut state, id).await)
    }

    pub async fn list_k8s_containers(
        State(mut state): State<ActorSystem>,
        Query(filter): Query<K8sListQuery>,
    ) -> Result<Json<ApiResponse<Vec<InfoContainerEntity>>>, AppError> {
        to_json(list_k8s_containers(&mut state, filter).await)
    }

    pub async fn patch_info_k8s_container(
        State(mut state): State<ActorSystem>,
        Path(id): Path<String>,
        Json(payload): Json<InfoK8sContainerPatchRequest>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(patch_info_k8s_container(&mut state, id, payload).await)
    }
}

impl InfoK8sLiveContainerController {
    pub async fn list_k8s_containers(
        State(mut state): State<ActorSystem>,
        Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<InfoContainerEntity>>>, AppError> {
        to_json(
            get_k8s_live_containers_paginated(&mut state, pagination.limit, pagination.offset)
                .await,
        )
    }

    pub async fn get_k8s_container(
        Path(id): Path<String>,
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<InfoContainerEntity>>, AppError> {
        to_json(get_k8s_live_container(&mut state, id).await)
    }
}

async fn get_info_k8s_container(
    state: &mut ActorSystem,
    id: String,
) -> anyhow::Result<InfoContainerEntity> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetInfoK8sContainer(id)),
        )
        .await?
    {
        StateActorResult::InfoContainerEntity(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn list_k8s_containers(
    state: &mut ActorSystem,
    filter: K8sListQuery,
) -> anyhow::Result<Vec<InfoContainerEntity>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::ListK8sContainers(filter)),
        )
        .await?
    {
        StateActorResult::VecInfoContainerEntity(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn patch_info_k8s_container(
    state: &mut ActorSystem,
    id: String,
    payload: InfoK8sContainerPatchRequest,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::PatchInfoK8sContainer(id, payload)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_k8s_live_containers_paginated(
    state: &mut ActorSystem,
    limit: Option<usize>,
    offset: Option<usize>,
) -> anyhow::Result<PaginatedResponse<InfoContainerEntity>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sLiveContainersPaginated(
                limit, offset,
            )),
        )
        .await?
    {
        StateActorResult::PaginatedInfoContainerEntity(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_k8s_live_container(
    state: &mut ActorSystem,
    id: String,
) -> anyhow::Result<InfoContainerEntity> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sLiveContainer(id)),
        )
        .await?
    {
        StateActorResult::InfoContainerEntity(v) => Ok(v),
        _ => unreachable!(),
    }
}
