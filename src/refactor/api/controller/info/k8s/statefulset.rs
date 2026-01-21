use axum::extract::{Path, Query, State};
use axum::Json;
use k8s_openapi::api::apps::v1::StatefulSet;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::{PaginatedResponse, PaginationQuery};
use crate::refactor::state::{
    InfoK8sServiceMessage, StateActor, StateActorMessage, StateActorResult,
};

pub struct InfoK8sStatefulSetController;

impl InfoK8sStatefulSetController {
    pub async fn get_k8s_statefulsets(
        State(mut state): State<ActorSystem>,
        Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<StatefulSet>>>, AppError> {
        to_json(
            get_k8s_statefulsets_paginated(&mut state, pagination.limit, pagination.offset).await,
        )
    }

    pub async fn get_k8s_statefulset(
        Path((namespace, name)): Path<(String, String)>,
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<StatefulSet>>, AppError> {
        to_json(get_k8s_statefulset(&mut state, namespace, name).await)
    }
}

async fn get_k8s_statefulsets_paginated(
    state: &mut ActorSystem,
    limit: Option<usize>,
    offset: Option<usize>,
) -> anyhow::Result<PaginatedResponse<StatefulSet>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sStatefulSetsPaginated(
                limit, offset,
            )),
        )
        .await?
    {
        StateActorResult::PaginatedStatefulSet(v) => Ok(v),
        _ => Err(anyhow::anyhow!("Failed to get k8s statefulsets")),
    }
}

async fn get_k8s_statefulset(
    state: &mut ActorSystem,
    namespace: String,
    name: String,
) -> anyhow::Result<StatefulSet> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sStatefulSet(namespace, name)),
        )
        .await?
    {
        StateActorResult::StatefulSet(v) => Ok(v),
        _ => Err(anyhow::anyhow!("Failed to get k8s statefulset")),
    }
}
