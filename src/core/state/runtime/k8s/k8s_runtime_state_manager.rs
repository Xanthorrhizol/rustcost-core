use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::Utc;
use crate::core::state::runtime::k8s::k8s_runtime_state::{K8sRuntimeState, RuntimePod};
use crate::core::state::runtime::k8s::k8s_runtime_state_repository_trait::K8sRuntimeStateRepositoryTrait;
use crate::errors::AppError;

pub struct K8sRuntimeStateManager<R: K8sRuntimeStateRepositoryTrait> {
    pub(crate) repo: Arc<R>,
    pub(crate) is_resyncing: AtomicBool,
}

impl<R: K8sRuntimeStateRepositoryTrait> K8sRuntimeStateManager<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self {
            repo,
            is_resyncing: AtomicBool::new(false),
        }
    }
    /// Replace the entire K8s runtime state.
    pub async fn set_state(&self, state: K8sRuntimeState) {
        self.repo.set(state).await;
    }

    /// Update the discovery snapshot based on fresh K8s data.
    ///
    /// This expects a list of fully constructed RuntimePod entries.
    pub async fn update_discovery(
        &self,
        nodes: Vec<String>,
        namespaces: Vec<String>,
        deployments: Vec<String>,
        pods: Vec<RuntimePod>,
    ) -> anyhow::Result<()> {
        self.repo
            .update(|state| {
                state.update(nodes.clone(), namespaces.clone(), deployments.clone(), pods.clone());
            })
            .await;

        Ok(())
    }

    /// Record a discovery failure (state remains intact).
    pub async fn mark_error(&self, message: String) {
        self.repo.update(|state| state.mark_error(message)).await;
    }

    pub async fn ensure_resynced(&self) -> Result<(), AppError> {
        let state = self.repo.get().await;

        if let Some(ts) = state.last_discovered_at {
            let hours = (Utc::now() - ts).num_hours();
            if hours < 3 {
                return Ok(());
            }
        }

        Err(AppError::NotResynced(
            "K8s runtime state not resynchronized (older than 3 hours)".into(),
        ))
    }


    // ===============================================
    // 1. Is last discovery recent (< 3 hours)
    // ===============================================
    pub async fn is_fresh(&self) -> bool {
        let state = self.repo.get().await;

        if let Some(ts) = state.last_discovered_at {
            let hours = (Utc::now() - ts).num_hours();
            return hours < 3;
        }
        false
    }

    // ===============================================
    // 2. Get pods by deployment (fast O(n))
    // ===============================================
    pub async fn get_pods_by_deployment(&self, deployment: &str) -> Vec<RuntimePod> {
        let state = self.repo.get().await;

        if let Some(uids) = state.pods_by_deployment.get(deployment) {
            return uids
                .iter()
                .filter_map(|uid| state.pods.get(uid).cloned())
                .collect();
        }
        Vec::new()
    }

    // ===============================================
    // 3. Get pods by namespace
    // ===============================================
    pub async fn get_pods_by_namespace(&self, namespace: &str) -> Vec<RuntimePod> {
        let state = self.repo.get().await;

        if let Some(uids) = state.pods_by_namespace.get(namespace) {
            return uids
                .iter()
                .filter_map(|uid| state.pods.get(uid).cloned())
                .collect();
        }
        Vec::new()
    }

    // ===============================================
    // 4. Get pods by node
    // ===============================================
    pub async fn get_pods_by_node(&self, node: &str) -> Vec<RuntimePod> {
        let state = self.repo.get().await;

        if let Some(uids) = state.pods_by_node.get(node) {
            return uids
                .iter()
                .filter_map(|uid| state.pods.get(uid).cloned())
                .collect();
        }
        Vec::new()
    }

    // ===============================================
    // 5. Get all node names
    // ===============================================
    pub async fn get_nodes(&self) -> Vec<String> {
        let state = self.repo.get().await;
        state.nodes.clone()
    }

    // ===============================================
    // 6b. Get all deployments
    // ===============================================
    pub async fn get_deployments(&self) -> Vec<String> {
        let state = self.repo.get().await;
        state.deployments.clone()
    }

    // ===============================================
    // 7. Get all namespaces
    // ===============================================
    pub async fn get_namespaces(&self) -> Vec<String> {
        let state = self.repo.get().await;
        state.namespaces.clone()
    }

    // ===============================================
    // 8. Get all pods (UIDs)
    // ===============================================
    pub async fn get_pods(&self) -> Vec<String> {
        let state = self.repo.get().await;
        state.pods.keys().cloned().collect()
    }

    // ===============================================
    // 9. Get all containers for a pod UID
    // ===============================================
    pub async fn get_containers(&self, pod_uid: &str) -> Vec<String> {
        let state = self.repo.get().await;
        state
            .pods
            .get(pod_uid)
            .map(|p| p.containers.clone())
            .unwrap_or_default()
    }

    // ===============================================
    // 10. Get all container keys (pod_uid-container_name)
    // ===============================================
    pub async fn get_container_keys(&self) -> Vec<String> {
        let state = self.repo.get().await;
        state
            .pods
            .values()
            .flat_map(|pod| {
                pod.containers
                    .iter()
                    .map(move |c| format!("{}-{}", pod.uid, c))
            })
            .collect()
    }

    pub fn is_resyncing(&self) -> bool {
        self.is_resyncing.load(Ordering::SeqCst)
    }
}
