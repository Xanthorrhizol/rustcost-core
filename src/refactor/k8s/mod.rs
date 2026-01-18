pub mod actor;
mod client;

pub use actor::K8sActor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum K8sActorMessage {
    Cronjobs(Target, By),
    Daemonsets(Target, By),
    Deployments(Target, By),
    Ingresses(Target, By),
    Jobs(Target, By),
    Namespaces(Target, By),
    Nodes(Target, By),
    Pods(Target, By),
    Services(Target, By),
    Statefulsets(Target, By),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Target {
    All,
    One,
    Names,
    Summary,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum By {
    None,
    /// name
    ByName(String),
    /// namespace
    ByNamespace(String),
    /// namespace, name
    ByNameAndNamespace(String, String),
    /// label_selector
    ByLabel(String),
    /// node_name
    ByNode(String),
    /// uid
    ByUid(String),
}
