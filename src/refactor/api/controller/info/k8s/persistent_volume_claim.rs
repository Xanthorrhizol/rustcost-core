use axum::extract::{Path, Query, State};
use axum::Json;
use k8s_openapi::api::core::v1::PersistentVolumeClaim;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::{PaginatedResponse, PaginationQuery};
use crate::refactor::state::{
    InfoK8sServiceMessage, StateActor, StateActorMessage, StateActorResult,
};

pub struct InfoK8sPvcController;

impl InfoK8sPvcController {
    pub async fn get_k8s_persistent_volume_claims(
        State(mut state): State<ActorSystem>,
        Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<PersistentVolumeClaim>>>, AppError> {
        to_json(
            get_k8s_persistent_volume_claims_paginated(
                &mut state,
                pagination.limit,
                pagination.offset,
            )
            .await,
        )
    }

    pub async fn get_k8s_persistent_volume_claim(
        Path((namespace, name)): Path<(String, String)>,
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<PersistentVolumeClaim>>, AppError> {
        to_json(get_k8s_persistent_volume_claim(&mut state, namespace, name).await)
    }
}

async fn get_k8s_persistent_volume_claims_paginated(
    state: &mut ActorSystem,
    limit: Option<usize>,
    offset: Option<usize>,
) -> anyhow::Result<PaginatedResponse<PersistentVolumeClaim>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(
                InfoK8sServiceMessage::GetK8sPersistentVolumeClaimsPaginated(limit, offset),
            ),
        )
        .await?
    {
        StateActorResult::PaginatedPersistentVolumeClaim(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_k8s_persistent_volume_claim(
    state: &mut ActorSystem,
    namespace: String,
    name: String,
) -> anyhow::Result<PersistentVolumeClaim> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sPersistentVolumeClaim(
                namespace, name,
            )),
        )
        .await?
    {
        StateActorResult::PersistentVolumeClaim(v) => Ok(v),
        _ => unreachable!(),
    }
}
