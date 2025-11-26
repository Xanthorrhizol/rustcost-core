use anyhow::Result;
use serde_json::{json, Value};
use crate::scheduler;
use crate::scheduler::tasks::info::k8s_refresh::task::refresh_k8s_object_info;

pub async fn resync() -> Result<Value> {
    tokio::spawn(async {
        refresh_k8s_object_info().await;
    });

    Ok(json!({"resync": "started"}))}

