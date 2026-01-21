use axum::extract::State;
use axum::Json;
use serde_json::Value;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::InfoAlertUpsertRequest;
use crate::refactor::entity::InfoAlertEntity;
use crate::refactor::state::{InfoServiceMessage, StateActor, StateActorMessage, StateActorResult};

pub struct InfoAlertController;

impl InfoAlertController {
    pub async fn get_info_alerts(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<InfoAlertEntity>>, AppError> {
        to_json(get_info_alerts(&mut state).await)
    }

    pub async fn upsert_info_alerts(
        State(mut state): State<ActorSystem>,
        Json(payload): Json<InfoAlertUpsertRequest>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(upsert_info_alerts(&mut state, payload).await)
    }
}

async fn get_info_alerts(state: &mut ActorSystem) -> anyhow::Result<InfoAlertEntity> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Info(InfoServiceMessage::GetInfoAlerts),
        )
        .await?
    {
        StateActorResult::InfoAlertEntity(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn upsert_info_alerts(
    state: &mut ActorSystem,
    req: InfoAlertUpsertRequest,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Info(InfoServiceMessage::UpsertInfoAlerts(req)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}
