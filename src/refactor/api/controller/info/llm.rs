use axum::extract::State;
use axum::Json;
use serde_json::Value;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::InfoLlmUpsertRequest;
use crate::refactor::entity::InfoLlmEntity;
use crate::refactor::state::{InfoServiceMessage, StateActor, StateActorMessage, StateActorResult};

pub struct InfoLlmController;

impl InfoLlmController {
    pub async fn get_info_llm(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<InfoLlmEntity>>, AppError> {
        to_json(get_info_llm(&mut state).await)
    }

    pub async fn upsert_info_llm(
        State(mut state): State<ActorSystem>,
        Json(payload): Json<InfoLlmUpsertRequest>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(upsert_info_llm(&mut state, payload).await)
    }
}

async fn get_info_llm(state: &mut ActorSystem) -> anyhow::Result<InfoLlmEntity> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Info(InfoServiceMessage::GetInfoLlm),
        )
        .await?
    {
        StateActorResult::InfoLlmEntity(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn upsert_info_llm(
    state: &mut ActorSystem,
    payload: InfoLlmUpsertRequest,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Info(InfoServiceMessage::UpsertInfoLlm(payload)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}
