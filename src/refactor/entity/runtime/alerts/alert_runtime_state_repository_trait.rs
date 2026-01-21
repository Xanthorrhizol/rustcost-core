use async_trait::async_trait;
use crate::core::state::runtime::alerts::alert_runtime_state::AlertRuntimeState;

#[async_trait]
pub trait AlertRuntimeStateRepositoryTrait: Send + Sync {
    async fn get(&self) -> AlertRuntimeState;
    async fn set(&self, state: AlertRuntimeState);

    async fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut AlertRuntimeState) + Send + Sync;

    async fn active_count(&self) -> usize {
        let state = self.get().await;
        state.active_alerts().len()
    }
}
