use super::*;
use xan_actor::prelude::*;

pub struct LogActor {
    actor_system: ActorSystem,
}

#[async_trait::async_trait]
impl Actor for LogActor {
    type Message = LogActorMessage;
    type Result = LogActorResult;
    type Error = anyhow::Error;

    fn address(&self) -> &str {
        log_actor_addr!()
    }

    async fn actor(&mut self, msg: Self::Message) -> Result<Self::Result, Self::Error> {
        todo!()
    }
}
