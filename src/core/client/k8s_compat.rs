// Compatibility shim for old k8s client
// TODO: Migrate all services to use new kube-rs client directly

#![allow(dead_code, unused_variables)]

use anyhow::Result;

// Util module compatibility
pub mod util {
    use anyhow::Result;
    use reqwest::Client;
    use std::env;

    pub fn read_token() -> Result<String> {
        if let Ok(token) = std::env::var("KUBE_TOKEN") {
            return Ok(token);
        }

        if let Ok(path) = std::env::var("RUSTCOST_TOKEN_PATH") {
            if let Ok(token) = std::fs::read_to_string(&path) {
                return Ok(token.trim().to_string());
            }
        }

        match std::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/token") {
            Ok(token) => Ok(token.trim().to_string()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn build_client() -> Result<Client> {
        Ok(Client::new())
    }

    pub fn k8s_api_server() -> String {
        env::var("RUSTCOST_K8S_API_URL")
            .unwrap_or_else(|_| "https://kubernetes.default.svc".to_string())
    }

    pub async fn build_kube_client() -> Result<kube::Client> {
        crate::core::client::kube_client::build_kube_client().await
    }
}

// Pod client compatibility
pub mod client_k8s_pod {
    use super::*;
    use crate::core::client::{kube_client, pods};
    use crate::core::client::kube_resources::Pod;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct PodList {
        pub items: Vec<Pod>,
    }

    /// Fetch ALL pods using kube-rs
    pub async fn fetch_pods(_token: &str, _client: &reqwest::Client) -> Result<PodList> {
        let kube = kube_client::build_kube_client().await?;
        let items = pods::fetch_pods(&kube).await?;
        Ok(PodList { items })
    }

    pub async fn fetch_pod_by_uid(_token: &str, _client: &reqwest::Client, uid: &str) -> Result<Pod> {
        let kube = kube_client::build_kube_client().await?;
        pods::fetch_pod_by_uid(&kube, uid).await
    }
    pub async fn fetch_pod_by_name_and_namespace(
        _client: &reqwest::Client,
        namespace: &str,
        name: &str,
    ) -> Result<Pod> {
        let kube = kube_client::build_kube_client().await?;
        pods::fetch_pod_by_name_and_namespace(&kube, namespace, name).await
    }

    pub async fn fetch_pods_by_label(
        _token: &str,
        _client: &reqwest::Client,
        label: &str,
    ) -> Result<PodList> {
        let kube = kube_client::build_kube_client().await?;
        let items = pods::fetch_pods_by_label(&kube, label).await?;
        Ok(PodList { items })
    }

    pub async fn fetch_pods_by_namespace(
        _token: &str,
        _client: &reqwest::Client,
        namespace: &str,
    ) -> Result<PodList> {
        let kube = kube_client::build_kube_client().await?;
        let items = pods::fetch_pods_by_namespace(&kube, namespace).await?;
        Ok(PodList { items })
    }

    pub async fn fetch_pods_by_node(
        _token: &str,
        _client: &reqwest::Client,
        node: &str,
    ) -> Result<PodList> {
        let kube = kube_client::build_kube_client().await?;
        let items = pods::fetch_pods_by_node(&kube, node).await?;
        Ok(PodList { items })
    }
}

// Pod mapper compatibility
pub mod client_k8s_pod_mapper {
    use super::*;
    use crate::core::client::kube_resources::Pod;
    use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;

    pub fn map_pod_to_info_pod_entity(pod: &Pod) -> Result<InfoPodEntity> {
        crate::core::client::mappers::map_pod_to_info_entity(pod)
    }
}

// Deployment compatibility
pub mod client_k8s_deployment {
    use super::*;
    use crate::core::client::{deployments, kube_client};
    use crate::core::client::kube_resources::Deployment;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct DeploymentList {
        pub items: Vec<Deployment>,
    }

    pub async fn fetch_deployments(
        _token: &str,
        _client: &reqwest::Client,
    ) -> Result<DeploymentList> {
        let kube = kube_client::build_kube_client().await?;
        let items = deployments::fetch_deployments(&kube).await?;
        Ok(DeploymentList { items })
    }

    pub async fn fetch_deployment_by_name_and_namespace(
        _token: &str,
        _client: &reqwest::Client,
        namespace: &str,
        name: &str,
    ) -> Result<Deployment> {
        let kube = kube_client::build_kube_client().await?;
        deployments::fetch_deployment_by_name_and_namespace(&kube, namespace, name).await
    }

    pub async fn fetch_deployments_by_namespace(
        _token: &str,
        _client: &reqwest::Client,
        namespace: &str,
    ) -> Result<DeploymentList> {
        let kube = kube_client::build_kube_client().await?;
        let items = deployments::fetch_deployments_by_namespace(&kube, namespace).await?;
        Ok(DeploymentList { items })
    }

    pub async fn fetch_deployments_by_label(
        _token: &str,
        _client: &reqwest::Client,
        label: &str,
    ) -> Result<DeploymentList> {
        let kube = kube_client::build_kube_client().await?;
        let items = deployments::fetch_deployments_by_label(&kube, label).await?;
        Ok(DeploymentList { items })
    }
}

pub mod client_k8s_deployment_mapper {
    use super::*;
    use crate::core::client::kube_resources::Deployment;
    use crate::core::persistence::info::k8s::deployment::info_deployment_entity::InfoDeploymentEntity;

    pub fn map_deployment_to_info_deployment_entity(_d: &Deployment) -> Result<InfoDeploymentEntity> {
        Ok(InfoDeploymentEntity::default())
    }
}

// Namespace compatibility
pub mod client_k8s_namespace {
    use super::*;
    use crate::core::client::kube_resources::Namespace;
    use crate::core::client::{kube_client, namespaces};
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Clone)]
    pub struct NamespaceList {
        pub items: Vec<Namespace>,
    }

    pub async fn fetch_namespaces(_token: &str, _client: &reqwest::Client) -> Result<NamespaceList> {
        let kube = kube_client::build_kube_client().await?;
        let items = namespaces::fetch_namespaces(&kube).await?;
        Ok(NamespaceList { items })
    }

    pub async fn fetch_namespace_by_name(
        _token: &str,
        _client: &reqwest::Client,
        name: &str,
    ) -> Result<Namespace> {
        let kube = kube_client::build_kube_client().await?;
        namespaces::fetch_namespace_by_name(&kube, name).await
    }
}

pub mod client_k8s_namespace_mapper {
    use super::*;
    use crate::core::client::kube_resources::Namespace;
    use crate::core::persistence::info::k8s::namespace::info_namespace_entity::InfoNamespaceEntity;

    pub fn map_namespace_to_info_namespace_entity(_ns: &Namespace) -> Result<InfoNamespaceEntity> {
        Ok(InfoNamespaceEntity::default())
    }
}

// Container compatibility
pub mod client_k8s_container {
    use super::*;

    pub async fn fetch_containers(_token: &str, _client: &reqwest::Client) -> Result<Vec<()>> {
        Ok(Vec::new())
    }
}

pub mod client_k8s_container_mapper {
    use super::*;
    use crate::core::client::kube_resources::ContainerStatus;
    use crate::core::persistence::info::k8s::container::info_container_entity::InfoContainerEntity;

    pub fn map_container_status_to_info_container_entity(_cs: &ContainerStatus) -> Result<InfoContainerEntity> {
        Ok(InfoContainerEntity::default())
    }
}

// HPA compatibility
pub mod client_k8s_hpa {
    use super::*;
    use crate::core::client::{kube_client, other_resources};
    use crate::core::client::kube_resources::HorizontalPodAutoscaler;

    pub async fn fetch_hpas(_token: &str, _client: &reqwest::Client) -> Result<Vec<HorizontalPodAutoscaler>> {
        let kube = kube_client::build_kube_client().await?;
        other_resources::fetch_hpas(&kube).await
    }

    pub async fn fetch_horizontal_pod_autoscalers(_token: &str, _client: &reqwest::Client) -> Result<Vec<HorizontalPodAutoscaler>> {
        fetch_hpas(_token, _client).await
    }
}

pub mod client_k8s_hpa_mapper {}

// LimitRange compatibility
pub mod client_k8s_limit_range {
    use super::*;
    use crate::core::client::{kube_client, other_resources};
    use crate::core::client::kube_resources::LimitRange;

    pub async fn fetch_limit_ranges(_token: &str, _client: &reqwest::Client) -> Result<Vec<LimitRange>> {
        let kube = kube_client::build_kube_client().await?;
        other_resources::fetch_limit_ranges(&kube).await
    }
}

pub mod client_k8s_limit_range_mapper {}

// PV/PVC compatibility
pub mod client_k8s_persistent_volume {
    use super::*;
    use crate::core::client::{kube_client, other_resources};
    use crate::core::client::kube_resources::PersistentVolume;

    pub async fn fetch_persistent_volumes(_token: &str, _client: &reqwest::Client) -> Result<Vec<PersistentVolume>> {
        let kube = kube_client::build_kube_client().await?;
        other_resources::fetch_persistent_volumes(&kube).await
    }
}

pub mod client_k8s_persistent_volume_claim {
    use super::*;
    use crate::core::client::{kube_client, other_resources};
    use crate::core::client::kube_resources::PersistentVolumeClaim;

    pub async fn fetch_persistent_volume_claims(_token: &str, _client: &reqwest::Client) -> Result<Vec<PersistentVolumeClaim>> {
        let kube = kube_client::build_kube_client().await?;
        other_resources::fetch_persistent_volume_claims(&kube).await
    }
}

pub mod client_k8s_persistent_volume_mapper {}
pub mod client_k8s_persistent_volume_claim_mapper {}

// ResourceQuota compatibility
pub mod client_k8s_resource_quota {
    use super::*;
    use crate::core::client::{kube_client, other_resources};
    use crate::core::client::kube_resources::ResourceQuota;

    pub async fn fetch_resource_quotas(_token: &str, _client: &reqwest::Client) -> Result<Vec<ResourceQuota>> {
        let kube = kube_client::build_kube_client().await?;
        other_resources::fetch_resource_quotas(&kube).await
    }
}

pub mod client_k8s_resource_quota_mapper {}
