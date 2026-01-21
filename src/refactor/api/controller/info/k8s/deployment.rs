use axum::extract::{Path, Query, State};
use axum::Json;
use k8s_openapi::api::apps::v1::Deployment;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::{PaginatedResponse, PaginationQuery};
use crate::refactor::state::{
    InfoK8sServiceMessage, StateActor, StateActorMessage, StateActorResult,
};

pub struct InfoK8sDeploymentController;

impl InfoK8sDeploymentController {
    pub async fn get_k8s_deployments(
        State(mut state): State<ActorSystem>,
        Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<Deployment>>>, AppError> {
        to_json(
            get_k8s_deployments_paginated(&mut state, pagination.limit, pagination.offset).await,
        )
    }

    pub async fn get_k8s_deployment(
        Path((namespace, name)): Path<(String, String)>,
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Deployment>>, AppError> {
        to_json(get_k8s_deployment(&mut state, namespace, name).await)
    }
}

async fn get_k8s_deployments_paginated(
    state: &mut ActorSystem,
    limit: Option<usize>,
    offset: Option<usize>,
) -> anyhow::Result<PaginatedResponse<Deployment>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sDeploymentsPaginated(
                limit, offset,
            )),
        )
        .await?
    {
        StateActorResult::PaginatedDeployment(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_k8s_deployment(
    state: &mut ActorSystem,
    namespace: String,
    name: String,
) -> anyhow::Result<Deployment> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sDeployment(namespace, name)),
        )
        .await?
    {
        StateActorResult::Deployment(v) => Ok(v),
        _ => unreachable!(),
    }
}
