use axum::extract::State;
use axum::Json;
use serde_json::Value;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::InfoSettingUpsertRequest;
use crate::refactor::entity::InfoSettingEntity;
use crate::refactor::state::{InfoServiceMessage, StateActor, StateActorMessage, StateActorResult};

pub struct InfoSettingController;

impl InfoSettingController {
    pub async fn get_info_settings(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<InfoSettingEntity>>, AppError> {
        to_json(get_info_settings(&mut state).await)
    }

    pub async fn upsert_info_settings(
        State(mut state): State<ActorSystem>,
        Json(payload): Json<InfoSettingUpsertRequest>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(upsert_info_settings(&mut state, payload).await)
    }
}

async fn get_info_settings(state: &mut ActorSystem) -> anyhow::Result<InfoSettingEntity> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Info(InfoServiceMessage::GetInfoSettings),
        )
        .await?
    {
        StateActorResult::InfoSettingEntity(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn upsert_info_settings(
    state: &mut ActorSystem,
    payload: InfoSettingUpsertRequest,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Info(InfoServiceMessage::UpsertInfoSettings(payload)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}
