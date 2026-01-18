use super::*;
use crate::app_state::AppState;
use xan_actor::prelude::*;

pub struct StateActor {
    actor_system: ActorSystem,
    state: AppState,
}

#[async_trait::async_trait]
impl Actor for StateActor {
    type Message = StateActorMessage;
    type Result = StateActorResult;
    type Error = anyhow::Error;

    fn address(&self) -> &str {
        state_actor_addr!()
    }

    async fn actor(&mut self, msg: Self::Message) -> Result<Self::Result, Self::Error> {
        todo!()
    }
}
