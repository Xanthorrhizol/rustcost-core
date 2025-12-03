//! Info controller: connects routes to info usecases

use std::sync::Arc;
use serde_json::{json, Value};
use anyhow::Result;
use crate::core::state::runtime::k8s::k8s_runtime_state_manager::K8sRuntimeStateManager;
use crate::core::state::runtime::k8s::k8s_runtime_state_repository::K8sRuntimeStateRepository;
use crate::core::state::runtime::k8s::k8s_runtime_state_repository_trait::K8sRuntimeStateRepositoryTrait;
pub async fn status_internal(
    k8s_state: Arc<K8sRuntimeStateManager<K8sRuntimeStateRepository>>,
) -> Result<Value> {
    let st = k8s_state.repo.get().await;

    Ok(json!({
        "last_discovered_at": st.last_discovered_at,
        "last_error_at": st.last_error_at,
        "last_error_message": st.last_error_message,
        "resync_running": k8s_state.is_resyncing(),
    }))
}
