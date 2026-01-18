pub mod actor;

use crate::refactor::dto::{LlmChatRequest, LlmChatWithContextRequest, LlmMessage};
pub use actor::LlmActor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LlmActorMessage {
    Chat(LlmChatRequest),
    ChatWithContext(LlmChatWithContextRequest),
}

pub type LlmActorResult = serde_json::Value;
