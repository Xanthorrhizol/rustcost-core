use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;
use anyhow::{Context, Result};
use kube::api::{Api, ListParams};
use serde_json::{json, Value};
use tokio::time::sleep;
use tracing::{error};
use crate::core::state::runtime::k8s::k8s_runtime_state_manager::K8sRuntimeStateManager;
use crate::core::state::runtime::k8s::k8s_runtime_state_repository::K8sRuntimeStateRepository;
use crate::scheduler::tasks::info::k8s_refresh::task::refresh_k8s_object_info;

async fn ensure_k8s_available() -> Result<()> {
    let client = crate::core::client::kube_client::build_kube_client()
        .await
        .context("failed to build kube client")?;

    // Lightweight readiness check: list namespaces to verify API is reachable.
    let api: Api<k8s_openapi::api::core::v1::Namespace> = Api::all(client);
    api.list(&ListParams::default())
        .await
        .context("failed to reach Kubernetes API")?;

    Ok(())
}

pub async fn resync(
    k8s_state: Arc<K8sRuntimeStateManager<K8sRuntimeStateRepository>>,
) -> Result<Value> {
    ensure_k8s_available().await?;
    do_resync(k8s_state).await
}

/// Kick off a background refresh of the Kubernetes runtime state.
pub async fn do_resync(
    k8s_state: Arc<K8sRuntimeStateManager<K8sRuntimeStateRepository>>,
) -> Result<Value> {

    // Prevent double-start
    if k8s_state.is_resyncing.swap(true, Ordering::SeqCst) {
        return Ok(json!({ "resync": "already_running" }));
    }

    let mgr = k8s_state.clone();

    tokio::spawn(async move {
        if let Err(e) = refresh_k8s_object_info(&mgr).await {
            error!("K8s resync failed: {e}");
        }
        // ⏳ WAIT 10 SECONDS BEFORE MARKING COMPLETE
        sleep(Duration::from_secs(10)).await;
        // Mark as finished
        mgr.is_resyncing.store(false, Ordering::SeqCst);
    });

    Ok(json!({ "resync": "started" }))
}