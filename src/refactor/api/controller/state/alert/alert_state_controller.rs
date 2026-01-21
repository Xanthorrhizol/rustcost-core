use axum::extract::{Json, Path, State};
use serde_json::{json, Value};
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::entity::AlertEvent;
use crate::refactor::runtime::{
    AlertRuntimeMessage, RuntimeActor, RuntimeActorMessage, RuntimeActorResult,
};

pub struct AlertStateController;

impl AlertStateController {
    pub async fn get_active(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let alerts = get_active_alerts(&mut state).await;
        to_json(Ok(json!({ "active_alerts": alerts })))
    }

    pub async fn get_all(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let all = get_all_alerts(&mut state).await;
        to_json(Ok(json!({ "alerts": all })))
    }

    pub async fn fire(
        State(mut state): State<ActorSystem>,
        Json(payload): Json<Value>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let id = payload["id"].as_str().unwrap_or("").to_string();
        let message = payload["message"].as_str().unwrap_or("").to_string();
        let severity = payload["severity"].as_str().unwrap_or("info").to_string();

        fire_alert(&mut state, id, message, severity).await;

        to_json(Ok(json!({ "status": "ok" })))
    }

    pub async fn resolve(
        Path(id): Path<String>,
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        resolve_alert(&mut state, id.to_string()).await;

        to_json(Ok(json!({ "resolved": id })))
    }
}

async fn get_active_alerts(state: &mut ActorSystem) -> Vec<AlertEvent> {
    match state
        .send_and_recv::<RuntimeActor>(
            runtime_actor_addr!().to_string(),
            RuntimeActorMessage::Alert(AlertRuntimeMessage::GetActiveAlerts),
        )
        .await
        .expect("failed to get alerts")
    {
        RuntimeActorResult::Alerts(alerts) => alerts,
        _ => vec![],
    }
}

async fn get_all_alerts(state: &mut ActorSystem) -> Vec<AlertEvent> {
    match state
        .send_and_recv::<RuntimeActor>(
            runtime_actor_addr!().to_string(),
            RuntimeActorMessage::Alert(AlertRuntimeMessage::GetAllAlerts),
        )
        .await
        .expect("failed to get alerts")
    {
        RuntimeActorResult::Alerts(alerts) => alerts,
        _ => vec![],
    }
}

async fn fire_alert(state: &mut ActorSystem, id: String, message: String, severity: String) {
    state
        .send_and_recv::<RuntimeActor>(
            runtime_actor_addr!().to_string(),
            RuntimeActorMessage::Alert(AlertRuntimeMessage::FireAlert {
                id,
                message,
                severity,
            }),
        )
        .await
        .expect("failed to fire alerts");
}

async fn resolve_alert(state: &mut ActorSystem, id: String) {
    state
        .send_and_recv::<RuntimeActor>(
            runtime_actor_addr!().to_string(),
            RuntimeActorMessage::Alert(AlertRuntimeMessage::ResolveAlert { id }),
        )
        .await
        .expect("failed to resolve alerts");
}
