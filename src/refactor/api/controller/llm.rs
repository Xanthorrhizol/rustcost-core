use axum::extract::State;
use axum::Json;
use serde_json::Value;
use xan_actor::ActorSystem;

use crate::api::dto::ApiResponse;
use crate::api::util::json::to_json;
use crate::errors::AppError;
use crate::refactor::dto::{LlmChatRequest, LlmChatWithContextRequest};
use crate::refactor::llm::{LlmActor, LlmActorMessage};

pub struct LlmController;

impl LlmController {
    pub async fn chat(
        State(mut state): State<ActorSystem>,
        Json(payload): Json<LlmChatRequest>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(chat(&mut state, payload).await)
    }

    pub async fn chat_with_context(
        State(mut state): State<ActorSystem>,
        Json(payload): Json<LlmChatWithContextRequest>,
    ) -> Result<Json<ApiResponse<Value>>, AppError> {
        to_json(chat_with_context(&mut state, payload).await)
    }
}

async fn chat(state: &mut ActorSystem, message: LlmChatRequest) -> anyhow::Result<Value> {
    Ok(state
        .send_and_recv::<LlmActor>(
            llm_actor_addr!().to_string(),
            LlmActorMessage::Chat(message),
        )
        .await?)
}

async fn chat_with_context(
    state: &mut ActorSystem,
    message: LlmChatWithContextRequest,
) -> anyhow::Result<Value> {
    Ok(state
        .send_and_recv::<LlmActor>(
            llm_actor_addr!().to_string(),
            LlmActorMessage::ChatWithContext(message),
        )
        .await?)
}
