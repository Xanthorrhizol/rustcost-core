use axum::extract::{Path, Query, State};
use axum::Json;
use k8s_openapi::api::batch::v1::CronJob;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::{PaginatedResponse, PaginationQuery};
use crate::refactor::state::{
    InfoK8sServiceMessage, StateActor, StateActorMessage, StateActorResult,
};

pub struct InfoK8sCronJobController;

impl InfoK8sCronJobController {
    pub async fn get_k8s_cronjobs(
        State(mut state): State<ActorSystem>,
        Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<CronJob>>>, AppError> {
        to_json(get_k8s_cronjobs_paginated(&mut state, pagination.limit, pagination.offset).await)
    }

    pub async fn get_k8s_cronjob(
        Path((namespace, name)): Path<(String, String)>,
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<CronJob>>, AppError> {
        to_json(get_k8s_cronjob(&mut state, namespace, name).await)
    }
}

async fn get_k8s_cronjobs_paginated(
    state: &mut ActorSystem,
    limit: Option<usize>,
    offset: Option<usize>,
) -> anyhow::Result<PaginatedResponse<CronJob>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sCronJobsPaginated(
                limit, offset,
            )),
        )
        .await?
    {
        StateActorResult::PaginatedCronJob(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_k8s_cronjob(
    state: &mut ActorSystem,
    namespace: String,
    name: String,
) -> anyhow::Result<CronJob> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sCronJob(namespace, name)),
        )
        .await?
    {
        StateActorResult::CronJob(v) => Ok(v),
        _ => unreachable!(),
    }
}
