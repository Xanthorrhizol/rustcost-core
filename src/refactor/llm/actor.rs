use super::*;
use xan_actor::prelude::*;

pub struct LlmActor {
    actor_system: ActorSystem,
}

#[async_trait::async_trait]
impl Actor for LlmActor {
    type Message = LlmActorMessage;
    type Result = LlmActorResult; // serde_json::Value
    type Error = anyhow::Error;

    fn address(&self) -> &str {
        llm_actor_addr!()
    }

    async fn actor(&mut self, msg: Self::Message) -> Result<Self::Result, Self::Error> {
        match msg {
            LlmActorMessage::Chat(payload) => self.chat(payload).await,
            LlmActorMessage::ChatWithContext(payload) => self.chat_with_context(payload).await,
        }
    }
}

impl LlmActor {
    pub fn new(actor_system: ActorSystem) -> Self {
        Self { actor_system }
    }

    pub async fn chat(&mut self, payload: LlmChatRequest) -> anyhow::Result<serde_json::Value> {
        // TODO
        Ok(serde_json::Value::Null)
    }

    pub async fn chat_with_context(
        &mut self,
        payload: LlmChatWithContextRequest,
    ) -> anyhow::Result<serde_json::Value> {
        // TODO
        Ok(serde_json::Value::Null)
    }
}
