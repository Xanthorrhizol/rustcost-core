use super::client::*;
use super::*;
use xan_actor::prelude::*;

const DEFAULT_LIMIT: usize = 50;

pub struct K8sActor {
    actor_system: ActorSystem,
    client: kube::Client,
}

#[async_trait::async_trait]
impl Actor for K8sActor {
    type Message = K8sActorMessage;
    type Result = K8sActorResult;
    type Error = anyhow::Error;

    fn address(&self) -> &str {
        k8s_actor_addr!()
    }

    async fn actor(&mut self, msg: Self::Message) -> Result<Self::Result, Self::Error> {
        let client = &self.client;
        match msg {
            K8sActorMessage::Cronjobs(target, by) => match (target, by) {
                (Target::All, By::None) => Ok(K8sActorResult::CronJobs(
                    cronjobs::fetch_cronjobs(client).await?,
                )),
                (Target::All, By::ByNamespace(ns)) => Ok(K8sActorResult::CronJobs(
                    cronjobs::fetch_cronjobs_by_namespace(client, &ns).await?,
                )),
                (Target::One, By::ByNameAndNamespace(ns, name)) => Ok(K8sActorResult::CronJob(
                    cronjobs::fetch_cronjob_by_name_and_namespace(client, &name, &ns).await?,
                )),
                (Target::All, By::ByLabel(label_selector)) => Ok(K8sActorResult::CronJobs(
                    cronjobs::fetch_cronjobs_by_label(client, &label_selector).await?,
                )),
                _ => unreachable!(),
            },
            K8sActorMessage::Daemonsets(target, by) => match (target, by) {
                (Target::All, By::None) => Ok(K8sActorResult::DaemonSets(
                    daemonsets::fetch_daemonsets(client).await?,
                )),
                (Target::All, By::ByNamespace(ns)) => Ok(K8sActorResult::DaemonSets(
                    daemonsets::fetch_daemonsets_by_namespace(client, &ns).await?,
                )),
                (Target::One, By::ByNameAndNamespace(ns, name)) => Ok(K8sActorResult::DaemonSet(
                    daemonsets::fetch_daemonset_by_name_and_namespace(client, &name, &ns).await?,
                )),
                (Target::All, By::ByLabel(label_selector)) => Ok(K8sActorResult::DaemonSets(
                    daemonsets::fetch_daemonsets_by_label(client, &label_selector).await?,
                )),
                _ => unreachable!(),
            },
            K8sActorMessage::Deployments(target, by) => match (target, by) {
                (Target::All, By::None) => Ok(K8sActorResult::Deployments(
                    deployments::fetch_deployments(client).await?,
                )),
                (Target::All, By::ByNamespace(ns)) => Ok(K8sActorResult::Deployments(
                    deployments::fetch_deployments_by_namespace(client, &ns).await?,
                )),
                (Target::One, By::ByNameAndNamespace(ns, name)) => Ok(K8sActorResult::Deployment(
                    deployments::fetch_deployment_by_name_and_namespace(client, &name, &ns).await?,
                )),
                (Target::All, By::ByLabel(label_selector)) => Ok(K8sActorResult::Deployments(
                    deployments::fetch_deployments_by_label(client, &label_selector).await?,
                )),
                _ => unreachable!(),
            },
            K8sActorMessage::Ingresses(target, by) => match (target, by) {
                (Target::All, By::None) => Ok(K8sActorResult::Ingresses(
                    ingresses::fetch_ingresses(client).await?,
                )),
                (Target::All, By::ByNamespace(ns)) => Ok(K8sActorResult::Ingresses(
                    ingresses::fetch_ingresses_by_namespace(client, &ns).await?,
                )),
                (Target::One, By::ByNameAndNamespace(ns, name)) => Ok(K8sActorResult::Ingress(
                    ingresses::fetch_ingress_by_name_and_namespace(client, &name, &ns).await?,
                )),
                (Target::All, By::ByLabel(label_selector)) => Ok(K8sActorResult::Ingresses(
                    ingresses::fetch_ingresses_by_label(client, &label_selector).await?,
                )),
                _ => unreachable!(),
            },
            K8sActorMessage::Jobs(target, by) => match (target, by) {
                (Target::All, By::None) => {
                    Ok(K8sActorResult::Jobs(jobs::fetch_jobs(client).await?))
                }
                (Target::All, By::ByNamespace(ns)) => Ok(K8sActorResult::Jobs(
                    jobs::fetch_jobs_by_namespace(client, &ns).await?,
                )),
                (Target::One, By::ByNameAndNamespace(ns, name)) => Ok(K8sActorResult::Job(
                    jobs::fetch_job_by_name_and_namespace(client, &name, &ns).await?,
                )),
                (Target::All, By::ByLabel(label_selector)) => Ok(K8sActorResult::Jobs(
                    jobs::fetch_jobs_by_label(client, &label_selector).await?,
                )),
                _ => unreachable!(),
            },
            K8sActorMessage::Namespaces(target, by) => match (target, by) {
                (Target::All, By::None) => Ok(K8sActorResult::Namespaces(
                    namespaces::fetch_namespaces(client).await?,
                )),
                (Target::One, By::ByName(name)) => Ok(K8sActorResult::Namespace(
                    namespaces::fetch_namespace_by_name(client, &name).await?,
                )),
                (Target::Names, By::None) => Ok(K8sActorResult::Strings(
                    namespaces::fetch_namespace_names(client).await?,
                )),
                _ => unreachable!(),
            },
            K8sActorMessage::Nodes(target, by) => match (target, by) {
                (Target::All, By::None) => {
                    Ok(K8sActorResult::Nodes(nodes::fetch_nodes(client).await?))
                }
                (Target::One, By::ByName(name)) => Ok(K8sActorResult::Node(
                    nodes::fetch_node_by_name(client, &name).await?,
                )),
                (Target::Names, By::None) => Ok(K8sActorResult::Strings(
                    nodes::fetch_node_names(client).await?,
                )),
                (Target::Summary, By::ByName(name)) => Ok(K8sActorResult::NodeSummary(
                    nodes::fetch_node_summary(client, &name).await?,
                )),
                _ => unreachable!(),
            },
            K8sActorMessage::Pods(target, by) => match (target, by) {
                (Target::All, By::None) => {
                    Ok(K8sActorResult::Pods(pods::fetch_pods(client).await?))
                }
                (Target::All, By::ByNamespace(ns)) => Ok(K8sActorResult::Pods(
                    pods::fetch_pods_by_namespace(client, &ns).await?,
                )),
                (Target::One, By::ByNameAndNamespace(ns, name)) => Ok(K8sActorResult::Pod(
                    pods::fetch_pod_by_name_and_namespace(client, &name, &ns).await?,
                )),
                (Target::All, By::ByLabel(label_selector)) => Ok(K8sActorResult::Pods(
                    pods::fetch_pods_by_label(client, &label_selector).await?,
                )),
                (Target::All, By::ByNode(node_name)) => Ok(K8sActorResult::Pods(
                    pods::fetch_pods_by_node(client, &node_name).await?,
                )),
                (Target::One, By::ByUid(uid)) => Ok(K8sActorResult::Pod(
                    pods::fetch_pod_by_uid(client, &uid).await?,
                )),
                (Target::Names, By::None) => Ok(K8sActorResult::Strings(
                    pods::fetch_pod_names(client).await?,
                )),
                (Target::Names, By::ByNamespace(ns)) => Ok(K8sActorResult::Strings(
                    pods::fetch_pod_names_by_namespace(client, &ns).await?,
                )),
                (Target::Names, By::ByLabel(label_selector)) => Ok(K8sActorResult::Strings(
                    pods::fetch_pod_names_by_label(client, &label_selector).await?,
                )),
                (Target::Names, By::ByNode(node_name)) => Ok(K8sActorResult::Strings(
                    pods::fetch_pod_names_by_node(client, &node_name).await?,
                )),
                _ => unreachable!(),
            },
            K8sActorMessage::Services(target, by) => match (target, by) {
                (Target::All, By::None) => Ok(K8sActorResult::Services(
                    services::fetch_services(client).await?,
                )),
                (Target::All, By::ByNamespace(ns)) => Ok(K8sActorResult::Services(
                    services::fetch_services_by_namespace(client, &ns).await?,
                )),
                (Target::One, By::ByNameAndNamespace(ns, name)) => Ok(K8sActorResult::Service(
                    services::fetch_service_by_name_and_namespace(client, &name, &ns).await?,
                )),
                (Target::All, By::ByLabel(label_selector)) => Ok(K8sActorResult::Services(
                    services::fetch_services_by_label(client, &label_selector).await?,
                )),
                _ => unreachable!(),
            },
            K8sActorMessage::Statefulsets(target, by) => match (target, by) {
                (Target::All, By::None) => Ok(K8sActorResult::StatefulSets(
                    statefulsets::fetch_statefulsets(client).await?,
                )),
                (Target::All, By::ByNamespace(ns)) => Ok(K8sActorResult::StatefulSets(
                    statefulsets::fetch_statefulsets_by_namespace(client, &ns).await?,
                )),
                (Target::One, By::ByNameAndNamespace(ns, name)) => Ok(K8sActorResult::StatefulSet(
                    statefulsets::fetch_statefulset_by_name_and_namespace(client, &name, &ns)
                        .await?,
                )),
                (Target::All, By::ByLabel(label_selector)) => Ok(K8sActorResult::StatefulSets(
                    statefulsets::fetch_statefulsets_by_label(client, &label_selector).await?,
                )),
                _ => unreachable!(),
            },
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
