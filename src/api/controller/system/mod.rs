//! System controller: connects routes to system usecases

use axum::extract::{Path, Query};
use axum::{Json};
use serde_json::Value;
use axum::extract::State;

use crate::api::dto::ApiResponse;
use crate::api::dto::system_dto::{LogQuery, PaginatedLogResponse};
use crate::core::persistence::logs::log_repository::{LogRepository};
use crate::errors::AppError;
use crate::app_state::AppState;

pub async fn status() -> Json<ApiResponse<Value>> {
    match crate::domain::system::service::status_service::status().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn health() -> Json<ApiResponse<Value>> {
    match crate::domain::system::service::health_service::health().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn backup() -> Json<ApiResponse<Value>> {
    match crate::domain::system::service::backup_service::backup().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn resync() -> Json<ApiResponse<Value>> {
    match crate::domain::system::service::resync_service::resync().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}
pub async fn get_system_log_file_list(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<String>>> {
    match state.log_service.get_system_log_file_list().await {
        Ok(list) => Json(ApiResponse::ok(list)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_system_log_lines(
    State(state): State<AppState>,
    Path(date): Path<String>,
    Query(query): Query<LogQuery>,
) -> Result<Json<ApiResponse<PaginatedLogResponse>>, AppError> {
    let result = state
        .log_service
        .get_system_log_lines(&date, query.cursor, query.limit)
        .await;

    match result {
        Ok(v) => Ok(Json(ApiResponse::ok(v))),
        Err(_) => Err(AppError::InternalServerError),
    }
}

