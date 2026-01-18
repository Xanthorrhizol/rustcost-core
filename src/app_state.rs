use std::sync::Arc;

//
// SHORT IMPORTS
//

// system
use crate::domain::system::service::backup_service::backup;
use crate::domain::system::service::health_service::health;
use crate::domain::system::service::resync_service::resync;
use crate::domain::system::service::status_service::status_internal;

// info
use crate::domain::info::service::info_alerts_service::{get_info_alerts, upsert_info_alerts};
use crate::domain::info::service::info_llm_service::{get_info_llm, upsert_info_llm};
use crate::domain::info::service::info_settings_service::{
    get_info_settings, upsert_info_settings,
};
use crate::domain::info::service::info_unit_price_service::{
    get_info_unit_prices, upsert_info_unit_prices,
};
use crate::domain::info::service::info_version_service::get_info_versions;
use crate::domain::llm::service::llm_chat_service::chat as llm_chat;
use crate::domain::llm::service::llm_chat_service::chat_with_context as llm_chat_with_context;

// info k8s
use crate::domain::info::service::info_k8s_cronjob_service::{
    get_k8s_cronjob, get_k8s_cronjobs_paginated,
};
use crate::domain::info::service::info_k8s_daemonset_service::{
    get_k8s_daemonset, get_k8s_daemonsets_paginated,
};
use crate::domain::info::service::info_k8s_deployment_service::{
    get_k8s_deployment, get_k8s_deployments_paginated,
};
use crate::domain::info::service::info_k8s_hpa_service::get_k8s_hpas;
use crate::domain::info::service::info_k8s_ingress_service::{
    get_k8s_ingress, get_k8s_ingresses_paginated,
};
use crate::domain::info::service::info_k8s_job_service::{get_k8s_job, get_k8s_jobs_paginated};
use crate::domain::info::service::info_k8s_limit_range_service::get_k8s_limit_ranges;
use crate::domain::info::service::info_k8s_persistent_volume_claim_service::{
    get_k8s_persistent_volume_claim, get_k8s_persistent_volume_claims_paginated,
};
use crate::domain::info::service::info_k8s_persistent_volume_service::{
    get_k8s_persistent_volume, get_k8s_persistent_volumes_paginated,
};
use crate::domain::info::service::info_k8s_resource_quota_service::get_k8s_resource_quotas;
use crate::domain::info::service::info_k8s_service_service::{
    get_k8s_service, get_k8s_services_paginated,
};
use crate::domain::info::service::info_k8s_statefulset_service::{
    get_k8s_statefulset, get_k8s_statefulsets_paginated,
};
use crate::domain::info::service::info_namespace_service::get_k8s_namespaces;

use crate::domain::info::service::info_k8s_container_service::{
    get_info_k8s_container, list_k8s_containers, patch_info_k8s_container,
};
use crate::domain::info::service::info_k8s_live_container_service::{
    get_k8s_live_container, get_k8s_live_containers_paginated,
};
use crate::domain::info::service::info_k8s_live_node_service::{
    get_k8s_live_node, get_k8s_live_nodes_paginated,
};
use crate::domain::info::service::info_k8s_live_pod_service::{
    get_k8s_live_pod, get_k8s_live_pods_paginated,
};
use crate::domain::info::service::info_k8s_node_service::{
    get_info_k8s_node, list_k8s_nodes, patch_info_k8s_node_filter, patch_info_k8s_node_price,
};
use crate::domain::info::service::info_k8s_pod_service::{
    get_info_k8s_pod, list_k8s_pods, patch_info_k8s_pod,
};

// metrics
use crate::domain::metric::k8s::cluster::service::*;
use crate::domain::metric::k8s::container::service::*;
use crate::domain::metric::k8s::deployment::service::*;
use crate::domain::metric::k8s::namespace::service::*;
use crate::domain::metric::k8s::node::service::*;
use crate::domain::metric::k8s::pod::service::*;

// entities
use crate::core::persistence::info::fixed::alerts::info_alert_entity::InfoAlertEntity;
use crate::core::persistence::info::fixed::llm::info_llm_entity::InfoLlmEntity;
use crate::core::persistence::info::fixed::setting::info_setting_entity::InfoSettingEntity;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_entity::InfoUnitPriceEntity;
use crate::core::persistence::info::fixed::version::info_version_entity::InfoVersionEntity;

use crate::core::persistence::info::k8s::container::info_container_entity::InfoContainerEntity;
use crate::core::persistence::info::k8s::node::info_node_entity::InfoNodeEntity;
use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;

// dtos
use crate::domain::info::dto::info_alert_upsert_request::InfoAlertUpsertRequest;
use crate::domain::info::dto::info_k8s_container_patch_request::InfoK8sContainerPatchRequest;
use crate::domain::info::dto::info_k8s_node_patch_request::{
    InfoK8sNodePatchRequest, InfoK8sNodePricePatchRequest,
};
use crate::domain::info::dto::info_k8s_pod_patch_request::InfoK8sPodPatchRequest;
use crate::domain::info::dto::info_llm_upsert_request::InfoLlmUpsertRequest;
use crate::domain::info::dto::info_setting_upsert_request::InfoSettingUpsertRequest;
use crate::domain::info::dto::info_unit_price_upsert_request::InfoUnitPriceUpsertRequest;
use crate::domain::llm::dto::llm_chat_request::LlmChatRequest;
use crate::domain::llm::dto::llm_chat_with_context_request::LlmChatWithContextRequest;

use crate::api::dto::info_dto::{K8sListNodeQuery, K8sListQuery};
use crate::api::dto::k8s_pod_query_request_dto::K8sPodQueryRequestDto;
use crate::api::dto::metrics_dto::RangeQuery;
use crate::api::dto::paginated_response::PaginatedResponse;

// logs
use crate::core::persistence::logs::log_repository::LogRepositoryImpl;
use crate::core::state::runtime::alerts::alert_runtime_state_manager::AlertRuntimeStateManager;
use crate::core::state::runtime::alerts::alert_runtime_state_repository::AlertRuntimeStateRepository;
use crate::core::state::runtime::k8s::k8s_runtime_state_manager::K8sRuntimeStateManager;
use crate::core::state::runtime::k8s::k8s_runtime_state_repository::K8sRuntimeStateRepository;
use crate::domain::system::service::log_service::LogService;

//
// ============================================================
// APP STATE
// ============================================================
//
#[derive(Clone)]
pub struct AppState {
    pub log_service: Arc<LogService<LogRepositoryImpl>>,
    pub system_service: Arc<SystemService>,
    pub info_service: Arc<InfoService>,
    pub llm_service: Arc<LlmService>,
    pub info_k8s_service: Arc<InfoK8sService>,
    pub metric_service: Arc<MetricService>,

    // runtime state managers
    pub k8s_state: Arc<K8sRuntimeStateManager<K8sRuntimeStateRepository>>,
    pub alerts: Arc<AlertRuntimeStateManager<AlertRuntimeStateRepository>>,
}

pub fn build_app_state() -> AppState {
    // Create repositories
    let k8s_repo = K8sRuntimeStateRepository::new().shared();
    let alert_repo = AlertRuntimeStateRepository::new().shared();

    // Managers wrap repositories
    let k8s_state = Arc::new(K8sRuntimeStateManager::new(k8s_repo));
    let alerts = Arc::new(AlertRuntimeStateManager::new(alert_repo));

    AppState {
        log_service: Arc::new(LogService::new(LogRepositoryImpl::new())),
        system_service: Arc::new(SystemService::new(k8s_state.clone())),
        info_service: Arc::new(InfoService::default()),
        llm_service: Arc::new(LlmService::default()),
        info_k8s_service: Arc::new(InfoK8sService::default()),
        metric_service: Arc::new(MetricService::default()),

        k8s_state,
        alerts,
    }
}

//
// ============================================================
// SYSTEM
// ============================================================
//
#[derive(Clone)]
pub struct SystemService {
    pub k8s_state: Arc<K8sRuntimeStateManager<K8sRuntimeStateRepository>>,
}

impl SystemService {
    pub fn new(k8s_state: Arc<K8sRuntimeStateManager<K8sRuntimeStateRepository>>) -> Self {
        Self { k8s_state }
    }

    pub async fn health(&self) -> anyhow::Result<serde_json::Value> {
        health().await
    }
    pub async fn backup(&self) -> anyhow::Result<serde_json::Value> {
        backup().await
    }
    pub async fn status(&self) -> anyhow::Result<serde_json::Value> {
        status_internal(self.k8s_state.clone()).await
    }
    pub async fn resync(&self) -> anyhow::Result<serde_json::Value> {
        resync(self.k8s_state.clone()).await
    }
}

//
// ============================================================
// INFO
// ============================================================
//
#[derive(Clone, Default)]
pub struct InfoService;

impl InfoService {
    pub async fn get_info_unit_prices(&self) -> anyhow::Result<InfoUnitPriceEntity> {
        get_info_unit_prices().await
    }
    pub async fn upsert_info_unit_prices(
        &self,
        req: InfoUnitPriceUpsertRequest,
    ) -> anyhow::Result<serde_json::Value> {
        upsert_info_unit_prices(req).await
    }

    pub async fn get_info_versions(&self) -> anyhow::Result<InfoVersionEntity> {
        get_info_versions().await
    }

    pub async fn get_info_alerts(&self) -> anyhow::Result<InfoAlertEntity> {
        get_info_alerts().await
    }
    pub async fn upsert_info_alerts(
        &self,
        req: InfoAlertUpsertRequest,
    ) -> anyhow::Result<serde_json::Value> {
        upsert_info_alerts(req).await
    }

    pub async fn get_info_llm(&self) -> anyhow::Result<InfoLlmEntity> {
        get_info_llm().await
    }
    pub async fn upsert_info_llm(
        &self,
        req: InfoLlmUpsertRequest,
    ) -> anyhow::Result<serde_json::Value> {
        upsert_info_llm(req).await
    }

    pub async fn get_info_settings(&self) -> anyhow::Result<InfoSettingEntity> {
        get_info_settings().await
    }
    pub async fn upsert_info_settings(
        &self,
        req: InfoSettingUpsertRequest,
    ) -> anyhow::Result<serde_json::Value> {
        upsert_info_settings(req).await
    }
}

//
// ============================================================
// LLM
// ============================================================
//
#[derive(Clone, Default)]
pub struct LlmService;

impl LlmService {
    pub async fn chat(&self, payload: LlmChatRequest) -> anyhow::Result<serde_json::Value> {
        llm_chat(payload).await
    }
    pub async fn chat_with_context(
        &self,
        payload: LlmChatWithContextRequest,
    ) -> anyhow::Result<serde_json::Value> {
        llm_chat_with_context(payload).await
    }
}

//
// ============================================================
// INFO K8S
// ============================================================
//
#[derive(Clone, Default)]
pub struct InfoK8sService;

impl InfoK8sService {
    pub async fn get_k8s_namespaces(&self) -> anyhow::Result<serde_json::Value> {
        get_k8s_namespaces().await
    }
    pub async fn get_k8s_deployments_paginated(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> anyhow::Result<PaginatedResponse<k8s_openapi::api::apps::v1::Deployment>> {
        get_k8s_deployments_paginated(limit, offset).await
    }
    pub async fn get_k8s_deployment(
        &self,
        namespace: String,
        name: String,
    ) -> anyhow::Result<k8s_openapi::api::apps::v1::Deployment> {
        get_k8s_deployment(namespace, name).await
    }
    pub async fn get_k8s_statefulsets_paginated(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> anyhow::Result<PaginatedResponse<k8s_openapi::api::apps::v1::StatefulSet>> {
        get_k8s_statefulsets_paginated(limit, offset).await
    }
    pub async fn get_k8s_statefulset(
        &self,
        namespace: String,
        name: String,
    ) -> anyhow::Result<k8s_openapi::api::apps::v1::StatefulSet> {
        get_k8s_statefulset(namespace, name).await
    }
    pub async fn get_k8s_daemonsets_paginated(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> anyhow::Result<PaginatedResponse<k8s_openapi::api::apps::v1::DaemonSet>> {
        get_k8s_daemonsets_paginated(limit, offset).await
    }
    pub async fn get_k8s_daemonset(
        &self,
        namespace: String,
        name: String,
    ) -> anyhow::Result<k8s_openapi::api::apps::v1::DaemonSet> {
        get_k8s_daemonset(namespace, name).await
    }

    pub async fn get_k8s_jobs_paginated(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> anyhow::Result<PaginatedResponse<k8s_openapi::api::batch::v1::Job>> {
        get_k8s_jobs_paginated(limit, offset).await
    }
    pub async fn get_k8s_job(
        &self,
        namespace: String,
        name: String,
    ) -> anyhow::Result<k8s_openapi::api::batch::v1::Job> {
        get_k8s_job(namespace, name).await
    }

    pub async fn get_k8s_cronjobs_paginated(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> anyhow::Result<PaginatedResponse<k8s_openapi::api::batch::v1::CronJob>> {
        get_k8s_cronjobs_paginated(limit, offset).await
    }
    pub async fn get_k8s_cronjob(
        &self,
        namespace: String,
        name: String,
    ) -> anyhow::Result<k8s_openapi::api::batch::v1::CronJob> {
        get_k8s_cronjob(namespace, name).await
    }

    pub async fn get_k8s_services_paginated(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> anyhow::Result<PaginatedResponse<k8s_openapi::api::core::v1::Service>> {
        get_k8s_services_paginated(limit, offset).await
    }
    pub async fn get_k8s_service(
        &self,
        namespace: String,
        name: String,
    ) -> anyhow::Result<k8s_openapi::api::core::v1::Service> {
        get_k8s_service(namespace, name).await
    }

    pub async fn get_k8s_ingresses_paginated(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> anyhow::Result<PaginatedResponse<k8s_openapi::api::networking::v1::Ingress>> {
        get_k8s_ingresses_paginated(limit, offset).await
    }
    pub async fn get_k8s_ingress(
        &self,
        namespace: String,
        name: String,
    ) -> anyhow::Result<k8s_openapi::api::networking::v1::Ingress> {
        get_k8s_ingress(namespace, name).await
    }

    pub async fn get_k8s_persistent_volumes_paginated(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> anyhow::Result<PaginatedResponse<k8s_openapi::api::core::v1::PersistentVolume>> {
        get_k8s_persistent_volumes_paginated(limit, offset).await
    }
    pub async fn get_k8s_persistent_volume(
        &self,
        name: String,
    ) -> anyhow::Result<k8s_openapi::api::core::v1::PersistentVolume> {
        get_k8s_persistent_volume(name).await
    }

    pub async fn get_k8s_persistent_volume_claims_paginated(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> anyhow::Result<PaginatedResponse<k8s_openapi::api::core::v1::PersistentVolumeClaim>> {
        get_k8s_persistent_volume_claims_paginated(limit, offset).await
    }
    pub async fn get_k8s_persistent_volume_claim(
        &self,
        namespace: String,
        name: String,
    ) -> anyhow::Result<k8s_openapi::api::core::v1::PersistentVolumeClaim> {
        get_k8s_persistent_volume_claim(namespace, name).await
    }
    pub async fn get_k8s_resource_quotas(&self) -> anyhow::Result<serde_json::Value> {
        get_k8s_resource_quotas().await
    }
    pub async fn get_k8s_limit_ranges(&self) -> anyhow::Result<serde_json::Value> {
        get_k8s_limit_ranges().await
    }
    pub async fn get_k8s_hpas(&self) -> anyhow::Result<serde_json::Value> {
        get_k8s_hpas().await
    }

    pub async fn get_k8s_live_nodes_paginated(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> anyhow::Result<PaginatedResponse<k8s_openapi::api::core::v1::Node>> {
        get_k8s_live_nodes_paginated(limit, offset).await
    }
    pub async fn get_k8s_live_node(
        &self,
        node_name: String,
    ) -> anyhow::Result<k8s_openapi::api::core::v1::Node> {
        get_k8s_live_node(node_name).await
    }

    pub async fn get_k8s_live_pods_paginated(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> anyhow::Result<PaginatedResponse<k8s_openapi::api::core::v1::Pod>> {
        get_k8s_live_pods_paginated(limit, offset).await
    }
    pub async fn get_k8s_live_pod(
        &self,
        pod_uid: String,
    ) -> anyhow::Result<k8s_openapi::api::core::v1::Pod> {
        get_k8s_live_pod(pod_uid).await
    }

    pub async fn get_k8s_live_containers_paginated(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> anyhow::Result<PaginatedResponse<InfoContainerEntity>> {
        get_k8s_live_containers_paginated(limit, offset).await
    }
    pub async fn get_k8s_live_container(&self, id: String) -> anyhow::Result<InfoContainerEntity> {
        get_k8s_live_container(id).await
    }

    pub async fn get_info_k8s_node(&self, node_name: String) -> anyhow::Result<InfoNodeEntity> {
        get_info_k8s_node(node_name).await
    }
    pub async fn list_k8s_nodes(
        &self,
        filter: K8sListNodeQuery,
    ) -> anyhow::Result<Vec<InfoNodeEntity>> {
        list_k8s_nodes(filter).await
    }
    pub async fn patch_info_k8s_node_filter(
        &self,
        id: String,
        patch: InfoK8sNodePatchRequest,
    ) -> anyhow::Result<serde_json::Value> {
        patch_info_k8s_node_filter(id, patch).await
    }
    pub async fn patch_info_k8s_node_price(
        &self,
        id: String,
        patch: InfoK8sNodePricePatchRequest,
    ) -> anyhow::Result<serde_json::Value> {
        patch_info_k8s_node_price(id, patch).await
    }

    pub async fn get_info_k8s_pod(&self, pod_uid: String) -> anyhow::Result<InfoPodEntity> {
        get_info_k8s_pod(pod_uid).await
    }
    pub async fn list_k8s_pods(
        &self,
        state: AppState,
        filter: K8sPodQueryRequestDto,
    ) -> anyhow::Result<PaginatedResponse<InfoPodEntity>> {
        list_k8s_pods(state, filter).await
    }
    pub async fn patch_info_k8s_pod(
        &self,
        id: String,
        payload: InfoK8sPodPatchRequest,
    ) -> anyhow::Result<serde_json::Value> {
        patch_info_k8s_pod(id, payload).await
    }

    pub async fn get_info_k8s_container(&self, id: String) -> anyhow::Result<InfoContainerEntity> {
        get_info_k8s_container(id).await
    }
    pub async fn list_k8s_containers(
        &self,
        filter: K8sListQuery,
    ) -> anyhow::Result<Vec<InfoContainerEntity>> {
        list_k8s_containers(filter).await
    }
    pub async fn patch_info_k8s_container(
        &self,
        id: String,
        payload: InfoK8sContainerPatchRequest,
    ) -> anyhow::Result<serde_json::Value> {
        patch_info_k8s_container(id, payload).await
    }
}

//
// ============================================================
// METRICS
// ============================================================
//
#[derive(Clone, Default)]
pub struct MetricService;

impl MetricService {
    pub async fn get_metric_k8s_pods_raw(
        &self,
        q: RangeQuery,
        pod_uids: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_pods_raw(q, pod_uids).await
    }

    pub async fn get_metric_k8s_pods_raw_summary(
        &self,
        q: RangeQuery,
        pod_uids: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_pods_raw_summary(q, pod_uids).await
    }
    pub async fn get_metric_k8s_pods_raw_efficiency(
        &self,
        q: RangeQuery,
        _pod_uids: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_pods_raw_efficiency(q, _pod_uids).await
    }

    pub async fn get_metric_k8s_pod_raw(
        &self,
        pod_uid: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_pod_raw(pod_uid, q).await
    }
    pub async fn get_metric_k8s_pod_raw_summary(
        &self,
        pod_uid: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_pod_raw_summary(pod_uid, q).await
    }
    pub async fn get_metric_k8s_pod_raw_efficiency(
        &self,
        pod_uid: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_pod_raw_efficiency(pod_uid, q).await
    }

    pub async fn get_metric_k8s_pods_cost(
        &self,
        q: RangeQuery,
        _pod_uids: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_pods_cost(q, _pod_uids).await
    }
    pub async fn get_metric_k8s_pods_cost_summary(
        &self,
        q: RangeQuery,
        _pod_uids: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_pods_cost_summary(q, _pod_uids).await
    }
    pub async fn get_metric_k8s_pods_cost_trend(
        &self,
        q: RangeQuery,
        _pod_uids: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_pods_cost_trend(q, _pod_uids).await
    }

    pub async fn get_metric_k8s_pod_cost(
        &self,
        pod_uid: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_pod_cost(pod_uid, q).await
    }
    pub async fn get_metric_k8s_pod_cost_summary(
        &self,
        pod_uid: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_pod_cost_summary(pod_uid, q).await
    }
    pub async fn get_metric_k8s_pod_cost_trend(
        &self,
        pod_uid: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_pod_cost_trend(pod_uid, q).await
    }

    pub async fn get_metric_k8s_nodes_raw(
        &self,
        q: RangeQuery,
        node_names: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_nodes_raw(q, node_names).await
    }
    pub async fn get_metric_k8s_nodes_raw_summary(
        &self,
        q: RangeQuery,
        node_names: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_nodes_raw_summary(q, node_names).await
    }
    pub async fn get_metric_k8s_nodes_raw_efficiency(
        &self,
        q: RangeQuery,
        node_names: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_nodes_raw_efficiency(q, node_names).await
    }

    pub async fn get_metric_k8s_node_raw(
        &self,
        node_name: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_node_raw(node_name, q).await
    }
    pub async fn get_metric_k8s_node_raw_summary(
        &self,
        node_name: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_node_raw_summary(node_name, q).await
    }
    pub async fn get_metric_k8s_node_raw_efficiency(
        &self,
        node_name: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_node_raw_efficiency(node_name, q).await
    }

    pub async fn get_metric_k8s_nodes_cost(
        &self,
        q: RangeQuery,
        node_names: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_nodes_cost(q, node_names).await
    }
    pub async fn get_metric_k8s_nodes_cost_summary(
        &self,
        q: RangeQuery,
        node_names: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_nodes_cost_summary(q, node_names).await
    }
    pub async fn get_metric_k8s_nodes_cost_trend(
        &self,
        q: RangeQuery,
        node_names: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_nodes_cost_trend(q, node_names).await
    }

    pub async fn get_metric_k8s_node_cost(
        &self,
        node_name: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_node_cost(node_name, q).await
    }
    pub async fn get_metric_k8s_node_cost_summary(
        &self,
        node_name: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_node_cost_summary(node_name, q).await
    }
    pub async fn get_metric_k8s_node_cost_trend(
        &self,
        node_name: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_node_cost_trend(node_name, q).await
    }

    pub async fn get_metric_k8s_namespaces_raw(
        &self,
        q: RangeQuery,
        namespaces: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_namespaces_raw(q, namespaces).await
    }
    pub async fn get_metric_k8s_namespaces_raw_summary(
        &self,
        q: RangeQuery,
        namespaces: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_namespaces_raw_summary(q, namespaces).await
    }
    pub async fn get_metric_k8s_namespaces_raw_efficiency(
        &self,
        q: RangeQuery,
        namespaces: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_namespaces_raw_efficiency(q, namespaces).await
    }

    pub async fn get_metric_k8s_namespace_raw(
        &self,
        ns: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_namespace_raw(ns, q).await
    }
    pub async fn get_metric_k8s_namespace_raw_summary(
        &self,
        ns: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_namespace_raw_summary(ns, q).await
    }
    pub async fn get_metric_k8s_namespace_raw_efficiency(
        &self,
        ns: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_namespace_raw_efficiency(ns, q).await
    }

    pub async fn get_metric_k8s_namespaces_cost(
        &self,
        q: RangeQuery,
        namespaces: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_namespaces_cost(q, namespaces).await
    }
    pub async fn get_metric_k8s_namespaces_cost_summary(
        &self,
        q: RangeQuery,
        namespaces: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_namespaces_cost_summary(q, namespaces).await
    }
    pub async fn get_metric_k8s_namespaces_cost_trend(
        &self,
        q: RangeQuery,
        namespaces: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_namespaces_cost_trend(q, namespaces).await
    }

    pub async fn get_metric_k8s_namespace_cost(
        &self,
        ns: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_namespace_cost(ns, q).await
    }
    pub async fn get_metric_k8s_namespace_cost_summary(
        &self,
        ns: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_namespace_cost_summary(ns, q).await
    }
    pub async fn get_metric_k8s_namespace_cost_trend(
        &self,
        ns: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_namespace_cost_trend(ns, q).await
    }

    pub async fn get_metric_k8s_deployments_raw(
        &self,
        q: RangeQuery,
        deployments: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_deployments_raw(q, deployments).await
    }
    pub async fn get_metric_k8s_deployments_raw_summary(
        &self,
        q: RangeQuery,
        deployments: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_deployments_raw_summary(q, deployments).await
    }
    pub async fn get_metric_k8s_deployments_raw_efficiency(
        &self,
        q: RangeQuery,
        deployments: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_deployments_raw_efficiency(q, deployments).await
    }

    pub async fn get_metric_k8s_deployment_raw(
        &self,
        name: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_deployment_raw(name, q).await
    }
    pub async fn get_metric_k8s_deployment_raw_summary(
        &self,
        name: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_deployment_raw_summary(name, q).await
    }
    pub async fn get_metric_k8s_deployment_raw_efficiency(
        &self,
        name: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_deployment_raw_efficiency(name, q).await
    }

    pub async fn get_metric_k8s_deployments_cost(
        &self,
        q: RangeQuery,
        deployments: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_deployments_cost(q, deployments).await
    }
    pub async fn get_metric_k8s_deployments_cost_summary(
        &self,
        q: RangeQuery,
        deployments: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_deployments_cost_summary(q, deployments).await
    }
    pub async fn get_metric_k8s_deployments_cost_trend(
        &self,
        q: RangeQuery,
        deployments: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_deployments_cost_trend(q, deployments).await
    }

    pub async fn get_metric_k8s_deployment_cost(
        &self,
        name: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_deployment_cost(name, q).await
    }
    pub async fn get_metric_k8s_deployment_cost_summary(
        &self,
        name: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_deployment_cost_summary(name, q).await
    }
    pub async fn get_metric_k8s_deployment_cost_trend(
        &self,
        name: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_deployment_cost_trend(name, q).await
    }

    pub async fn get_metric_k8s_containers_raw(
        &self,
        q: RangeQuery,
        container_keys: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_containers_raw(q, container_keys).await
    }
    pub async fn get_metric_k8s_containers_raw_summary(
        &self,
        q: RangeQuery,
        container_keys: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_containers_raw_summary(q, container_keys).await
    }
    pub async fn get_metric_k8s_containers_raw_efficiency(
        &self,
        q: RangeQuery,
        container_keys: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_containers_raw_efficiency(q, container_keys).await
    }

    pub async fn get_metric_k8s_container_raw(
        &self,
        id: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_container_raw(id, q).await
    }
    pub async fn get_metric_k8s_container_raw_summary(
        &self,
        id: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_container_raw_summary(id, q).await
    }
    pub async fn get_metric_k8s_container_raw_efficiency(
        &self,
        id: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_container_raw_efficiency(id, q).await
    }

    pub async fn get_metric_k8s_containers_cost(
        &self,
        q: RangeQuery,
        container_keys: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_containers_cost(q, container_keys).await
    }
    pub async fn get_metric_k8s_containers_cost_summary(
        &self,
        q: RangeQuery,
        container_keys: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_containers_cost_summary(q, container_keys).await
    }
    pub async fn get_metric_k8s_containers_cost_trend(
        &self,
        q: RangeQuery,
        container_keys: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_containers_cost_trend(q, container_keys).await
    }

    pub async fn get_metric_k8s_container_cost(
        &self,
        id: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_container_cost(id, q).await
    }
    pub async fn get_metric_k8s_container_cost_summary(
        &self,
        id: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_container_cost_summary(id, q).await
    }
    pub async fn get_metric_k8s_container_cost_trend(
        &self,
        id: String,
        q: RangeQuery,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_container_cost_trend(id, q).await
    }
}

//
// ============================================================
// METRIC CLUSTER (manual)
// ============================================================
//
impl MetricService {
    pub async fn get_metric_k8s_cluster_raw(
        &self,
        q: RangeQuery,
        node_names: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_cluster_raw(node_names, q).await
    }

    pub async fn get_metric_k8s_cluster_raw_summary(
        &self,
        q: RangeQuery,
        node_names: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        get_metric_k8s_cluster_raw_summary(node_names, q).await
    }

    pub async fn get_metric_k8s_cluster_raw_efficiency(
        &self,
        q: RangeQuery,
        node_names: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        let nodes = list_k8s_nodes(K8sListNodeQuery::default()).await?;
        get_metric_k8s_cluster_raw_efficiency(nodes, node_names, q).await
    }

    pub async fn get_metric_k8s_cluster_cost(
        &self,
        q: RangeQuery,
        node_names: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        let costs = get_info_unit_prices().await?;
        get_metric_k8s_cluster_cost(node_names, costs, q).await
    }

    pub async fn get_metric_k8s_cluster_cost_summary(
        &self,
        q: RangeQuery,
        node_names: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        let costs = get_info_unit_prices().await?;
        get_metric_k8s_cluster_cost_summary(node_names, costs, q).await
    }

    pub async fn get_metric_k8s_cluster_cost_trend(
        &self,
        q: RangeQuery,
        node_names: Vec<String>,
    ) -> anyhow::Result<serde_json::Value> {
        let costs = get_info_unit_prices().await?;
        get_metric_k8s_cluster_cost_trend(node_names, costs, q).await
    }
}
