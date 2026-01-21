use std::sync::Arc;
use tokio::sync::RwLock;

use crate::core::state::runtime::alerts::alert_runtime_state::AlertRuntimeState;
use crate::core::state::runtime::alerts::alert_runtime_state_repository_trait::AlertRuntimeStateRepositoryTrait;

pub struct AlertRuntimeStateRepository {
    inner: Arc<RwLock<AlertRuntimeState>>,
}

impl AlertRuntimeStateRepository {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(AlertRuntimeState::default())),
        }
    }

    pub fn shared(self) -> Arc<Self> {
        Arc::new(self)
    }
}

#[async_trait::async_trait]
impl AlertRuntimeStateRepositoryTrait for AlertRuntimeStateRepository {
    async fn get(&self) -> AlertRuntimeState {
        self.inner.read().await.clone()
    }

    async fn set(&self, new_state: AlertRuntimeState) {
        let mut state = self.inner.write().await;
        *state = new_state;
    }

    async fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut AlertRuntimeState) + Send + Sync,
    {
        let mut state = self.inner.write().await;
        f(&mut state);
    }
}
