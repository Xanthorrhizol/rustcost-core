use axum::extract::State;
use axum::Json;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::state::{
    InfoK8sServiceMessage, StateActor, StateActorMessage, StateActorResult,
};

pub struct InfoK8sHpaController;

impl InfoK8sHpaController {
    pub async fn get_k8s_hpas(
        State(mut state): State<ActorSystem>,
    ) -> Result<Json<ApiResponse<serde_json::Value>>, AppError> {
        to_json(get_k8s_hpas(&mut state).await)
    }
}

async fn get_k8s_hpas(state: &mut ActorSystem) -> anyhow::Result<serde_json::Value> {
    match state
        .send_and_recv::<StateActor>(
            state_actor_addr!().to_string(),
            StateActorMessage::InfoK8s(InfoK8sServiceMessage::GetK8sHpas),
        )
        .await?
    {
        StateActorResult::Json(v) => Ok(v),
        _ => unreachable!(),
    }
}
