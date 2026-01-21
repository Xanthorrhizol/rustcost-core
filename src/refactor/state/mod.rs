pub mod actor;

pub use actor::StateActor;

use crate::refactor::dto::*;
use crate::refactor::entity::*;
use k8s_openapi::api::apps::v1::{DaemonSet, Deployment, StatefulSet};
use k8s_openapi::api::batch::v1::{CronJob, Job};
use k8s_openapi::api::core::v1::{Node, PersistentVolume, PersistentVolumeClaim, Pod, Service};
use k8s_openapi::api::networking::v1::Ingress;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum StateActorMessage {
    // {{{
    System(SystemServiceMessage),
    Info(InfoServiceMessage),
    Llm(LlmServiceMessage),
    InfoK8s(InfoK8sServiceMessage),
    Metric(MetricServiceMessage),
    // }}}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StateActorResult {
    // {{{
    Ok,
    Json(serde_json::Value),
    PaginatedLog(PaginatedLogResponse),

    // entity
    InfoAlertEntity(InfoAlertEntity),
    InfoUnitPriceEntity(InfoUnitPriceEntity),
    InfoVersionEntity(InfoVersionEntity),
    InfoLlmEntity(InfoLlmEntity),
    InfoSettingEntity(InfoSettingEntity),
    InfoContainerEntity(InfoContainerEntity),
    InfoNodeEntity(InfoNodeEntity),
    InfoPodEntity(InfoPodEntity),

    // entity vector
    VecInfoAlertEntity(Vec<InfoAlertEntity>),
    VecInfoContainerEntity(Vec<InfoContainerEntity>),
    VecInfoNodeEntity(Vec<InfoNodeEntity>),

    // external type
    DaemonSet(DaemonSet),
    Job(Job),
    Node(Node),
    PersistentVolume(PersistentVolume),
    StatefulSet(StatefulSet),
    CronJob(CronJob),
    Deployment(Deployment),
    Ingress(Ingress),
    PersistentVolumeClaim(PersistentVolumeClaim),
    Pod(Pod),
    Service(Service),

    // paginated response with entity
    PaginatedInfoContainerEntity(PaginatedResponse<InfoContainerEntity>),
    PaginatedInfoPodEntity(PaginatedResponse<InfoPodEntity>),

    // paginated response with external type
    PaginatedDaemonSet(PaginatedResponse<DaemonSet>),
    PaginatedJob(PaginatedResponse<Job>),
    PaginatedNode(PaginatedResponse<Node>),
    PaginatedPersistentVolume(PaginatedResponse<PersistentVolume>),
    PaginatedStatefulSet(PaginatedResponse<StatefulSet>),
    PaginatedCronJob(PaginatedResponse<CronJob>),
    PaginatedDeployment(PaginatedResponse<Deployment>),
    PaginatedIngress(PaginatedResponse<Ingress>),
    PaginatedPersistentVolumeClaim(PaginatedResponse<PersistentVolumeClaim>),
    PaginatedPod(PaginatedResponse<Pod>),
    PaginatedService(PaginatedResponse<Service>),
    // }}}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SystemServiceMessage {
    // {{{
    Health,
    Backup,
    Status,
    Resync,
    // }}}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InfoServiceMessage {
    // {{{
    GetInfoUnitPrices,
    UpsertInfoUnitPrices(InfoUnitPriceUpsertRequest),
    GetInfoVersions,
    GetInfoAlerts,
    UpsertInfoAlerts(InfoAlertUpsertRequest),
    GetInfoLlm,
    UpsertInfoLlm(InfoLlmUpsertRequest),
    GetInfoSettings,
    UpsertInfoSettings(InfoSettingUpsertRequest),
    // }}}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LlmServiceMessage {
    // {{{
    Chat(LlmChatRequest),
    ChatWithContext(LlmChatWithContextRequest),
    // }}}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InfoK8sServiceMessage {
    // {{{
    GetK8sNamespaces,
    /// limit, offset,
    GetK8sDeploymentsPaginated(Option<usize>, Option<usize>),
    /// namespace, name
    GetK8sDeployment(String, String),
    /// limit, offset,
    GetK8sStatefulSetsPaginated(Option<usize>, Option<usize>),
    /// namespace, name
    GetK8sStatefulSet(String, String),
    /// limit, offset,
    GetK8sDaemonsetsPaginated(Option<usize>, Option<usize>),
    /// namespace, name
    GetK8sDaemonset(String, String),

    /// limit, offset,
    GetK8sJobsPaginated(Option<usize>, Option<usize>),
    /// namespace, name
    GetK8sJob(String, String),

    /// limit, offset,
    GetK8sCronJobsPaginated(Option<usize>, Option<usize>),
    /// namespace, name
    GetK8sCronJob(String, String),

    /// limit, offset,
    GetK8sServicesPaginated(Option<usize>, Option<usize>),
    /// namespace, name
    GetK8sService(String, String),

    /// limit, offset,
    GetK8sIngressesPaginated(Option<usize>, Option<usize>),
    /// namespace, name
    GetK8sIngress(String, String),

    /// limit, offset,
    GetK8sPersistentVolumesPaginated(Option<usize>, Option<usize>),
    /// name
    GetK8sPersistentVolume(String),

    /// limit, offset,
    GetK8sPersistentVolumeClaimsPaginated(Option<usize>, Option<usize>),
    /// namespace, name
    GetK8sPersistentVolumeClaim(String, String),

    GetK8sResourceQuotas,
    GetK8sLimitRanges,
    GetK8sHpas,

    /// limit, offset
    GetK8sLiveNodesPaginated(Option<usize>, Option<usize>),
    /// node_name
    GetK8sLiveNode(String),

    /// limit, offset
    GetK8sLivePodsPaginated(Option<usize>, Option<usize>),
    /// pod_uid
    GetK8sLivePod(String),

    /// limit, offset
    GetK8sLiveContainersPaginated(Option<usize>, Option<usize>),
    /// id
    GetK8sLiveContainer(String),

    /// node_name
    GetInfoK8sNode(String),
    /// filter
    ListK8sNodes(K8sListNodeQuery),
    /// id, patch
    PatchInfoK8sNodeFilter(String, InfoK8sNodePatchRequest),
    /// id, patch
    PatchInfoK8sNodePrice(String, InfoK8sNodePricePatchRequest),

    /// pod_uid
    GetInfoK8sPod(String),
    /// filter
    ListK8sPods(K8sPodQueryRequest),
    /// id, payload
    PatchInfoK8sPod(String, InfoK8sPodPatchRequest),

    /// id
    GetInfoK8sContainer(String),
    /// filter
    ListK8sContainers(K8sListQuery),
    /// id, payload
    PatchInfoK8sContainer(String, InfoK8sContainerPatchRequest),
    // }}}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MetricServiceMessage {
    // {{{
    /// q, pod_uids
    GetMetricK8sPodsRaw(RangeQuery, Vec<String>),
    /// q, pod_uids
    GetMetricK8sPodsRawSummary(RangeQuery, Vec<String>),
    /// q, pod_uids
    GetMetricK8sPodsRawEfficiency(RangeQuery, Vec<String>),

    /// pod_uid, q
    GetMetricK8sPodRaw(String, RangeQuery),
    /// pod_uid, q
    GetMetricK8sPodRawSummary(String, RangeQuery),
    /// pod_uid, q
    GetMetricK8sPodRawEfficiency(String, RangeQuery),

    /// q, pod_uids
    GetMetricK8sPodsCost(RangeQuery, Vec<String>),
    /// q, pod_uids
    GetMetricK8sPodsCostSummary(RangeQuery, Vec<String>),
    /// q, pod_uids
    GetMetricK8sPodsCostTrend(RangeQuery, Vec<String>),

    /// pod_uid, q
    GetMetricK8sPodCost(String, RangeQuery),
    /// pod_uid, q
    GetMetricK8sPodCostSummary(String, RangeQuery),
    /// pod_uid, q
    GetMetricK8sPodCostTrend(String, RangeQuery),

    /// q, node_names
    GetMetricK8sNodesRaw(RangeQuery, Vec<String>),
    /// q, node_names
    GetMetricK8sNodesRawSummary(RangeQuery, Vec<String>),
    /// q, node_names
    GetMetricK8sNodesRawEfficiency(RangeQuery, Vec<String>),

    /// node_name, q
    GetMetricK8sNodeRaw(String, RangeQuery),
    /// node_name, q
    GetMetricK8sNodeRawSummary(String, RangeQuery),
    /// node_name, q
    GetMetricK8sNodeRawEfficiency(String, RangeQuery),

    /// q, node_names
    GetMetricK8sNodesCost(RangeQuery, Vec<String>),
    /// q, node_names
    GetMetricK8sNodesCostSummary(RangeQuery, Vec<String>),
    /// q, node_names
    GetMetricK8sNodesCostTrend(RangeQuery, Vec<String>),

    /// node_name, q
    GetMetricK8sNodeCost(String, RangeQuery),
    /// node_name, q
    GetMetricK8sNodeCostSummary(String, RangeQuery),
    /// node_name, q
    GetMetricK8sNodeCostTrend(String, RangeQuery),

    /// q, namespaces
    GetMetricK8sNamespacesRaw(RangeQuery, Vec<String>),
    /// q, namespaces
    GetMetricK8sNamespacesRawSummary(RangeQuery, Vec<String>),
    /// q, namespaces
    GetMetricK8sNamespacesRawEfficiency(RangeQuery, Vec<String>),

    /// ns, q
    GetMetricK8sNamespaceRaw(String, RangeQuery),
    /// ns, q
    GetMetricK8sNamespaceRawSummary(String, RangeQuery),
    /// ns, q
    GetMetricK8sNamespaceRawEfficiency(String, RangeQuery),

    /// q, namespaces
    GetMetricK8sNamespacesCost(RangeQuery, Vec<String>),
    /// q, namespaces
    GetMetricK8sNamespacesCostSummary(RangeQuery, Vec<String>),
    /// q, namespaces
    GetMetricK8sNamespacesCostTrend(RangeQuery, Vec<String>),

    /// ns, q
    GetMetricK8sNamespaceCost(String, RangeQuery),
    /// ns, q
    GetMetricK8sNamespaceCostSummary(String, RangeQuery),
    /// ns, q
    GetMetricK8sNamespaceCostTrend(String, RangeQuery),

    /// q, deployments
    GetMetricK8sDeploymentsRaw(RangeQuery, Vec<String>),
    /// q, deployments
    GetMetricK8sDeploymentsRawSummary(RangeQuery, Vec<String>),
    /// q, deployments
    GetMetricK8sDeploymentsRawEfficiency(RangeQuery, Vec<String>),

    /// name, q
    GetMetricK8sDeploymentRaw(String, RangeQuery),
    /// name, q
    GetMetricK8sDeploymentRawSummary(String, RangeQuery),
    /// name, q
    GetMetricK8sDeploymentRawEfficiency(String, RangeQuery),

    /// q, deployments
    GetMetricK8sDeploymentsCost(RangeQuery, Vec<String>),
    /// q, deployments
    GetMetricK8sDeploymentsCostSummary(RangeQuery, Vec<String>),
    /// q, deployments
    GetMetricK8sDeploymentsCostTrend(RangeQuery, Vec<String>),

    /// name, q
    GetMetricK8sDeploymentCost(String, RangeQuery),
    /// name, q
    GetMetricK8sDeploymentCostSummary(String, RangeQuery),
    /// name, q
    GetMetricK8sDeploymentCostTrend(String, RangeQuery),

    /// q, container_keys
    GetMetricK8sContainersRaw(RangeQuery, Vec<String>),
    /// q, container_keys
    GetMetricK8sContainersRawSummary(RangeQuery, Vec<String>),
    /// q, container_keys
    GetMetricK8sContainersRawEfficiency(RangeQuery, Vec<String>),

    /// id, q
    GetMetricK8sContainerRaw(String, RangeQuery),
    /// id, q
    GetMetricK8sContainerRawSummary(String, RangeQuery),
    /// id, q
    GetMetricK8sContainerRawEfficiency(String, RangeQuery),

    /// q, container_keys
    GetMetricK8sContainersCost(RangeQuery, Vec<String>),
    /// q, container_keys
    GetMetricK8sContainersCostSummary(RangeQuery, Vec<String>),
    /// q, container_keys
    GetMetricK8sContainersCostTrend(RangeQuery, Vec<String>),

    /// id, q
    GetMetricK8sContainerCost(String, RangeQuery),
    /// id, q
    GetMetricK8sContainerCostSummary(String, RangeQuery),
    /// id, q
    GetMetricK8sContainerCostTrend(String, RangeQuery),

    /// q, node_names
    GetMetricsK8sClusterRaw(RangeQuery, Vec<String>),
    /// q, node_names
    GetMetricsK8sClusterRawSummary(RangeQuery, Vec<String>),
    /// q, node_names
    GetMetricsK8sClusterRawEfficiency(RangeQuery, Vec<String>),

    /// q, node_names
    GetMetricsK8sClusterCost(RangeQuery, Vec<String>),
    /// q, node_names
    GetMetricsK8sClusterCostSummary(RangeQuery, Vec<String>),
    /// q, node_names
    GetMetricsK8sClusterCostTrend(RangeQuery, Vec<String>),
    // }}}
}
