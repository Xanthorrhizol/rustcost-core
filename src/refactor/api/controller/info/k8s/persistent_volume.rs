use axum::extract::{Path, Query, State};
use axum::Json;
use k8s_openapi::api::core::v1::PersistentVolume;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::{PaginatedResponse, PaginationQuery};
use crate::refactor::state::{
    InfoK8sServiceMessage, StateActor, StateActorMessage, StateActorResult,
};

pub struct InfoK8sPersistentVolumeController;

impl InfoK8sPersistentVolumeController {
    pub async fn get_k8s_persistent_volumes(
        State(mut state): State<ActorSystem>,
        Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<PersistentVolume>>>, AppError> {
        to_json(
            get_k8s_persistent_volumes_paginated(&mut state, pagination.limit, pagination.offset)
                .await,
        )
    }

    pub async fn get_k8s_persistent_volume(
        Path(name): Path<String>,
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<PersistentVolume>>, AppError> {
        to_json(get_k8s_persistent_volume(&mut state, name).await)
    }
}

async fn get_k8s_persistent_volumes_paginated(
    state: &mut ActorSystem,
    limit: Option<usize>,
    offset: Option<usize>,
) -> anyhow::Result<PaginatedResponse<PersistentVolume>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sPersistentVolumesPaginated(
                limit, offset,
            )),
        )
        .await?
    {
        StateActorResult::PaginatedPersistentVolume(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_k8s_persistent_volume(
    state: &mut ActorSystem,
    name: String,
) -> anyhow::Result<PersistentVolume> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sPersistentVolume(name)),
        )
        .await?
    {
        StateActorResult::PersistentVolume(v) => Ok(v),
        _ => unreachable!(),
    }
}
