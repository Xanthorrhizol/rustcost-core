//! Info controller: connects routes to info usecases

use axum::extract::State;
use axum::Json;
use serde_json::Value;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::InfoUnitPriceUpsertRequest;
use crate::refactor::entity::{InfoUnitPriceEntity, InfoVersionEntity};
use crate::refactor::state::{InfoServiceMessage, StateActor, StateActorMessage, StateActorResult};

pub struct InfoController;

impl InfoController {
    pub async fn get_info_unit_prices(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<InfoUnitPriceEntity>>, AppError> {
        to_json(get_info_unit_prices(&mut state).await)
    }

    pub async fn upsert_info_unit_prices(
        State(mut state): State<ActorSystem>,
        Json(payload): Json<InfoUnitPriceUpsertRequest>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(upsert_info_unit_prices(&mut state, payload).await)
    }

    pub async fn get_info_versions(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<InfoVersionEntity>>, AppError> {
        to_json(get_info_versions(&mut state).await)
    }
}

async fn get_info_unit_prices(state: &mut ActorSystem) -> anyhow::Result<InfoUnitPriceEntity> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Info(InfoServiceMessage::GetInfoUnitPrices),
        )
        .await?
    {
        StateActorResult::InfoUnitPriceEntity(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn upsert_info_unit_prices(
    state: &mut ActorSystem,
    payload: InfoUnitPriceUpsertRequest,
) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Info(InfoServiceMessage::UpsertInfoUnitPrices(payload)),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}

async fn get_info_versions(state: &mut ActorSystem) -> anyhow::Result<InfoVersionEntity> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::Info(InfoServiceMessage::GetInfoVersions),
        )
        .await?
    {
        StateActorResult::InfoVersionEntity(v) => Ok(v),
        _ => unreachable!(),
    }
}
