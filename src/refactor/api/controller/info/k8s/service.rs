use axum::extract::{Path, Query, State};
use axum::Json;
use k8s_openapi::api::core::v1::Service;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::{PaginatedResponse, PaginationQuery};
use crate::refactor::state::{
    InfoK8sServiceMessage, StateActor, StateActorMessage, StateActorResult,
};

pub struct InfoK8sServiceController;

impl InfoK8sServiceController {
    pub async fn get_k8s_services(
        State(mut state): State<ActorSystem>,
        Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<Service>>>, AppError> {
        to_json(get_k8s_services_paginated(&mut state, pagination.limit, pagination.offset).await)
    }

    pub async fn get_k8s_service(
        Path((namespace, name)): Path<(String, String)>,
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Service>>, AppError> {
        to_json(get_k8s_service(&mut state, namespace, name).await)
    }
}

async fn get_k8s_services_paginated(
    state: &mut ActorSystem,
    limit: Option<usize>,
    offset: Option<usize>,
) -> anyhow::Result<PaginatedResponse<Service>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sServicesPaginated(
                limit, offset,
            )),
        )
        .await?
    {
        StateActorResult::PaginatedService(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_k8s_service(
    state: &mut ActorSystem,
    namespace: String,
    name: String,
) -> anyhow::Result<Service> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sService(namespace, name)),
        )
        .await?
    {
        StateActorResult::Service(v) => Ok(v),
        _ => unreachable!(),
    }
}
