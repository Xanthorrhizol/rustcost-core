use axum::extract::{Path, Query, State};
use axum::Json;
use k8s_openapi::api::batch::v1::Job;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::{PaginatedResponse, PaginationQuery};
use crate::refactor::state::{
    InfoK8sServiceMessage, StateActor, StateActorMessage, StateActorResult,
};

pub struct InfoK8sJobController;

impl InfoK8sJobController {
    pub async fn get_k8s_jobs(
        State(mut state): State<ActorSystem>,
        Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<Job>>>, AppError> {
        to_json(get_k8s_jobs_paginated(&mut state, pagination.limit, pagination.offset).await)
    }

    pub async fn get_k8s_job(
        Path((namespace, name)): Path<(String, String)>,
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Job>>, AppError> {
        to_json(get_k8s_job(&mut state, namespace, name).await)
    }
}

async fn get_k8s_jobs_paginated(
    state: &mut ActorSystem,
    limit: Option<usize>,
    offset: Option<usize>,
) -> anyhow::Result<PaginatedResponse<Job>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sJobsPaginated(limit, offset)),
        )
        .await?
    {
        StateActorResult::PaginatedJob(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_k8s_job(
    state: &mut ActorSystem,
    namespace: String,
    name: String,
) -> anyhow::Result<Job> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sJob(namespace, name)),
        )
        .await?
    {
        StateActorResult::Job(v) => Ok(v),
        _ => unreachable!(),
    }
}
