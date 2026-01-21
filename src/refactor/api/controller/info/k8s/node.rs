use axum::extract::{Path, Query, State};
use axum::Json;
use k8s_openapi::api::core::v1::Node;
use serde_json::Value;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::{InfoK8sNodePatchRequest, InfoK8sNodePricePatchRequest};
use crate::refactor::dto::{K8sListNodeQuery, PaginatedResponse, PaginationQuery};
use crate::refactor::entity::InfoNodeEntity;
use crate::refactor::state::{
    InfoK8sServiceMessage, StateActor, StateActorMessage, StateActorResult,
};

pub struct InfoK8sNodeController;
pub struct InfoK8sLiveNodeController;

impl InfoK8sNodeController {
    pub async fn get_info_k8s_node(
        State(mut state): State<ActorSystem>,
        Path(node_name): Path<String>,
    ) -> Result<Json<ApiResponse<InfoNodeEntity>>, AppError> {
        to_json(get_info_k8s_node(&mut state, node_name).await)
    }

    pub async fn list_k8s_nodes(
        State(mut state): State<ActorSystem>,
        Query(filter): Query<K8sListNodeQuery>,
    ) -> Result<Json<ApiResponse<Vec<InfoNodeEntity>>>, AppError> {
        to_json(list_k8s_nodes(&mut state, filter).await)
    }

    pub async fn patch_info_k8s_node_filter(
        State(mut state): State<ActorSystem>,
        Path(id): Path<String>,
        Json(payload): Json<InfoK8sNodePatchRequest>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(patch_info_k8s_node_filter(&mut state, id, payload).await)
    }

    pub async fn patch_info_k8s_node_price(
        State(mut state): State<ActorSystem>,
        Path(id): Path<String>,
        Json(payload): Json<InfoK8sNodePricePatchRequest>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(patch_info_k8s_node_price(&mut state, id, payload).await)
    }
}

impl InfoK8sLiveNodeController {
    pub async fn list_k8s_nodes(
        State(mut state): State<ActorSystem>,
        Query(pagination): Query<PaginationQuery>,
    ) -> Result<Json<ApiResponse<PaginatedResponse<Node>>>, AppError> {
        to_json(get_k8s_live_nodes_paginated(&mut state, pagination.limit, pagination.offset).await)
    }

    pub async fn get_k8s_node(
        Path(node_name): Path<String>,
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Node>>, AppError> {
        to_json(get_k8s_live_node(&mut state, node_name).await)
    }
}

async fn get_info_k8s_node(
    state: &mut ActorSystem,
    node_name: String,
) -> anyhow::Result<InfoNodeEntity> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetInfoK8sNode(node_name)),
        )
        .await?
    {
        StateActorResult::InfoNodeEntity(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn list_k8s_nodes(
    state: &mut ActorSystem,
    filter: K8sListNodeQuery,
) -> anyhow::Result<Vec<InfoNodeEntity>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::ListK8sNodes(filter)),
        )
        .await?
    {
        StateActorResult::VecInfoNodeEntity(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn patch_info_k8s_node_filter(
    state: &mut ActorSystem,
    id: String,
    payload: InfoK8sNodePatchRequest,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::PatchInfoK8sNodeFilter(id, payload)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn patch_info_k8s_node_price(
    state: &mut ActorSystem,
    id: String,
    payload: InfoK8sNodePricePatchRequest,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::PatchInfoK8sNodePrice(id, payload)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_k8s_live_nodes_paginated(
    state: &mut ActorSystem,
    limit: Option<usize>,
    offset: Option<usize>,
) -> anyhow::Result<PaginatedResponse<Node>> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sLiveNodesPaginated(
                limit, offset,
            )),
        )
        .await?
    {
        StateActorResult::PaginatedNode(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_k8s_live_node(state: &mut ActorSystem, node_name: String) -> anyhow::Result<Node> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sLiveNode(node_name)),
        )
        .await?
    {
        StateActorResult::Node(v) => Ok(v),
        _ => unreachable!(),
    }
}
