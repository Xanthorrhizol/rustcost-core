use super::client::*;
use super::*;
use xan_actor::prelude::*;

pub struct K8sActor {
    actor_system: ActorSystem,
    client: kube::Client,
}

#[async_trait::async_trait]
impl Actor for K8sActor {
    type Message = K8sActorMessage;
    type Result = serde_json::Value;
    type Error = anyhow::Error;

    fn address(&self) -> &str {
        k8s_actor_addr!()
    }

    async fn actor(&mut self, msg: Self::Message) -> Result<Self::Result, Self::Error> {
        match msg {
            K8sActorMessage::Cronjobs(target, by) => todo!(),
            K8sActorMessage::Daemonsets(target, by) => todo!(),
            K8sActorMessage::Deployments(target, by) => todo!(),
            K8sActorMessage::Ingresses(target, by) => todo!(),
            K8sActorMessage::Jobs(target, by) => todo!(),
            K8sActorMessage::Namespaces(target, by) => todo!(),
            K8sActorMessage::Nodes(target, by) => todo!(),
            K8sActorMessage::Pods(target, by) => todo!(),
            K8sActorMessage::Services(target, by) => todo!(),
            K8sActorMessage::Statefulsets(target, by) => todo!(),
        }
    }
}

impl K8sActor {
    pub fn new(actor_system: ActorSystem, config: kube::Config) -> K8sActor {
        let client = kube::Client::try_from(config).expect("failed to create k8s client");
        K8sActor {
            actor_system,
            client,
        }
    }
}
