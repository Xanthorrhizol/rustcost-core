use axum::extract::{Path, Query, State};
use axum::Json;
use k8s_openapi::api::apps::v1::DaemonSet;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::{PaginatedResponse, PaginationQuery};
use crate::refactor::state::{
    InfoK8sServiceMessage, StateActor, StateActorMessage, StateActorResult,
};

pub struct InfoK8sDaemonSetController;

impl InfoK8sDaemonSetController {
    pub async fn get_k8s_daemonsets(
        State(mut state): State<ActorSystem>,
        Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<DaemonSet>>>, AppError> {
        to_json(get_k8s_daemonsets_paginated(&mut state, pagination.limit, pagination.offset).await)
    }

    pub async fn get_k8s_daemonset(
        Path((namespace, name)): Path<(String, String)>,
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<DaemonSet>>, AppError> {
        to_json(get_k8s_daemonset(&mut state, namespace, name).await)
    }
}

async fn get_k8s_daemonsets_paginated(
    state: &mut ActorSystem,
    limit: Option<usize>,
    offset: Option<usize>,
) -> anyhow::Result<PaginatedResponse<DaemonSet>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sDaemonsetsPaginated(
                limit, offset,
            )),
        )
        .await?
    {
        StateActorResult::PaginatedDaemonSet(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_k8s_daemonset(
    state: &mut ActorSystem,
    namespace: String,
    name: String,
) -> anyhow::Result<DaemonSet> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sDaemonset(namespace, name)),
        )
        .await?
    {
        StateActorResult::DaemonSet(v) => Ok(v),
        _ => unreachable!(),
    }
}
