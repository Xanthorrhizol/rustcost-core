use axum::extract::{Path, Query, State};
use axum::Json;
use k8s_openapi::api::networking::v1::Ingress;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::{PaginatedResponse, PaginationQuery};
use crate::refactor::state::{
    InfoK8sServiceMessage, StateActor, StateActorMessage, StateActorResult,
};

pub struct InfoK8sIngressController;

impl InfoK8sIngressController {
    pub async fn get_k8s_ingresses(
        State(mut state): State<ActorSystem>,
        Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<Ingress>>>, AppError> {
        to_json(get_k8s_ingresses_paginated(&mut state, pagination.limit, pagination.offset).await)
    }

    pub async fn get_k8s_ingress(
        Path((namespace, name)): Path<(String, String)>,
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Ingress>>, AppError> {
        to_json(get_k8s_ingress(&mut state, namespace, name).await)
    }
}

async fn get_k8s_ingresses_paginated(
    state: &mut ActorSystem,
    limit: Option<usize>,
    offset: Option<usize>,
) -> anyhow::Result<PaginatedResponse<Ingress>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sIngressesPaginated(
                limit, offset,
            )),
        )
        .await?
    {
        StateActorResult::PaginatedIngress(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_k8s_ingress(
    state: &mut ActorSystem,
    namespace: String,
    name: String,
) -> anyhow::Result<Ingress> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sIngress(namespace, name)),
        )
        .await?
    {
        StateActorResult::Ingress(v) => Ok(v),
        _ => unreachable!(),
    }
}
