use axum::{routing::post, Router};

use crate::refactor::api::controller::llm::LlmController;
use xan_actor::ActorSystem;

pub fn llm_routes() -> Router<ActorSystem> {
    Router::new()
        .route("/chat", post(LlmController::chat))
        .route("/chat-with-context", post(LlmController::chat_with_context))
}
