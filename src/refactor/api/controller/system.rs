//! System controller: connects routes to system usecases

use axum::extract::{Path, Query, State};
use axum::Json;
use serde_json::Value;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::{LogQuery, PaginatedLogResponse};
use crate::refactor::log::{LogActor, LogActorMessage, LogActorResult};
use crate::refactor::state::{
    StateActor, StateActorMessage, StateActorResult, SystemServiceMessage,
};

pub struct SystemController;

impl SystemController {
    pub async fn status(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(status(&mut state).await)
    }

    pub async fn health(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(health(&mut state).await)
    }

    pub async fn backup(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(backup(&mut state).await)
    }

    pub async fn resync(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(resync(&mut state).await)
    }

    pub async fn get_system_log_file_list(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Vec<String>>>, AppError> {
        to_json(get_system_log_file_list(&mut state).await)
    }

    pub async fn get_system_log_lines(
        State(mut state): State<ActorSystem>,
        Path(date): Path<String>,
        Query(query): Query<LogQuery>,
    ) -> Result<Json<ApiResponse<PaginatedLogResponse>>, AppError> {
        to_json(get_system_log_lines(&mut state, date.to_string(), query.cursor, query.limit).await)
    }
}

async fn status(state: &mut ActorSystem) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::System(SystemServiceMessage::Status),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => Err(anyhow::anyhow!("failed to get system status")),
    }
}

async fn health(state: &mut ActorSystem) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::System(SystemServiceMessage::Health),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => Err(anyhow::anyhow!("failed to get system health")),
    }
}

async fn backup(state: &mut ActorSystem) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::System(SystemServiceMessage::Backup),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => Err(anyhow::anyhow!("failed to get system backup")),
    }
}

async fn resync(state: &mut ActorSystem) -> anyhow::Result<Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::System(SystemServiceMessage::Resync),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => Err(anyhow::anyhow!("failed to get system resync")),
    }
}

async fn get_system_log_file_list(state: &mut ActorSystem) -> anyhow::Result<Vec<String>> {
    match state
        .send_and_recv::<LogActor>(
            log_actor_addr!().to_string(),
            LogActorMessage::GetSystemLogFileList,
        )
        .await?
    {
        LogActorResult::VecString(v) => Ok(v),
        _ => Err(anyhow::anyhow!("failed to get system log file list")),
    }
}

async fn get_system_log_lines(
    state: &mut ActorSystem,
    date: String,
    cursor: Option<usize>,
    limit: Option<usize>,
) -> anyhow::Result<PaginatedLogResponse> {
    match state
        .send_and_recv::<LogActor>(
            log_actor_addr!().to_string(),
            LogActorMessage::GetSystemLogLines {
                date,
                cursor,
                limit,
            },
        )
        .await?
    {
        LogActorResult::PaginatedLog(v) => Ok(v),
        _ => Err(anyhow::anyhow!("failed to get system log lines")),
    }
}
