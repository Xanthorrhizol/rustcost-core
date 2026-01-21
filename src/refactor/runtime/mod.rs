pub mod actor;

pub use actor::RuntimeActor;

use crate::refactor::entity::AlertEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum RuntimeActorMessage {
    Alert(AlertRuntimeMessage),
    K8s(K8sRuntimeMessage),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AlertRuntimeMessage {
    GetActiveAlerts,
    GetAllAlerts,
    FireAlert {
        id: String,
        message: String,
        severity: String,
    },
    ResolveAlert {
        id: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum K8sRuntimeMessage {
    GetFull,
    GetSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RuntimeActorResult {
    Ok,
    Alerts(Vec<AlertEvent>),
    K8s(serde_json::Value),
}
