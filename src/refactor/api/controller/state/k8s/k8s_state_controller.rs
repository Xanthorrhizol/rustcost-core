use axum::{extract::State, Json};
use serde_json::Value;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::runtime::{
    K8sRuntimeMessage, RuntimeActor, RuntimeActorMessage, RuntimeActorResult,
};

pub struct K8sStateController;

impl K8sStateController {
    pub async fn get_full(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let s = get_full(&mut state).await;
        to_json(Ok(s))
    }

    pub async fn get_summary(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        let s = get_summary(&mut state).await;
        to_json(Ok(s))
    }
}

async fn get_full(state: &mut ActorSystem) -> Value {
    match state
        .send_and_recv::<RuntimeActor>(
            runtime_actor_addr!().to_string(),
            RuntimeActorMessage::K8s(K8sRuntimeMessage::GetFull),
        )
        .await
        .expect("failed to get k8s state")
    {
        RuntimeActorResult::K8s(v) => v,
        _ => unreachable!(),
    }
}

async fn get_summary(state: &mut ActorSystem) -> Value {
    match state
        .send_and_recv::<RuntimeActor>(
            runtime_actor_addr!().to_string(),
            RuntimeActorMessage::K8s(K8sRuntimeMessage::GetSummary),
        )
        .await
        .expect("failed to get k8s state")
    {
        RuntimeActorResult::K8s(v) => v,
        _ => unreachable!(),
    }
}
