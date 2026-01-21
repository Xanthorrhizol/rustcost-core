use super::*;
use crate::refactor::entity::{AlertRuntimeState, K8sRuntimeState};
use xan_actor::prelude::*;

pub struct RuntimeActor {
    actor_system: ActorSystem,
    alert: AlertRuntimeState,
    k8s: K8sRuntimeState,
}

#[async_trait::async_trait]
impl Actor for RuntimeActor {
    type Message = RuntimeActorMessage;
    type Result = RuntimeActorResult;
    type Error = anyhow::Error;

    fn address(&self) -> &str {
        state_actor_addr!()
    }

    async fn actor(&mut self, msg: Self::Message) -> Result<Self::Result, Self::Error> {
        // // GetSummary
        // let s = state.k8s_state.repo.get().await;

        // // calculate container count
        // let container_count: usize = s.pods.values().map(|p| p.containers.len()).sum();
        //
        // to_json(Ok(json!({
        //     "nodes": s.nodes.len(),
        //     "namespaces": s.namespaces.len(),
        //     "deployments": s.deployments.len(),
        //     "pods": s.pods.len(),
        //     "containers": container_count,
        //     "last_discovered_at": s.last_discovered_at,
        //     "last_error_at": s.last_error_at,
        //     "last_error_message": s.last_error_message,
        // })))
        todo!()
    }
}
