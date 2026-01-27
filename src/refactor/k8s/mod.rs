pub mod actor;
mod client;

pub use actor::K8sActor;
use k8s_openapi::api::apps::v1::{DaemonSet, Deployment, StatefulSet};
use k8s_openapi::api::batch::v1::{CronJob, Job};
use k8s_openapi::api::core::v1::{Namespace, Node, Pod, Service};
use k8s_openapi::api::networking::v1::Ingress;
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
pub enum K8sActorResult {
    CronJobs(Vec<CronJob>),
    CronJob(CronJob),
    DaemonSets(Vec<DaemonSet>),
    DaemonSet(DaemonSet),
    Deployments(Vec<Deployment>),
    Deployment(Deployment),
    Ingresses(Vec<Ingress>),
    Ingress(Ingress),
    Jobs(Vec<Job>),
    Job(Job),
    Namespaces(Vec<Namespace>),
    Namespace(Namespace),
    Nodes(Vec<Node>),
    Node(Node),
    Pods(Vec<Pod>),
    Pod(Pod),
    Services(Vec<Service>),
    Service(Service),
    StatefulSets(Vec<StatefulSet>),
    StatefulSet(StatefulSet),
    Strings(Vec<String>),
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
