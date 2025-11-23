use anyhow::Result;
use tracing::{debug, error};
use crate::core::persistence::info::fixed::setting::info_setting_repository::InfoSettingRepository;
use crate::scheduler::tasks::processors::retention::task::RetentionTask;

pub async fn run() -> Result<()> {
    debug!("Running day task (aggregation + retention)...");

    if let Err(e) = super::processors::day::run().await {
        error!(?e, "Daily aggregator failed");
    }

    // Create settings repository DI
    let settings_repo = InfoSettingRepository::new();
    let retention_task = RetentionTask::new(settings_repo);

    if let Err(e) = retention_task.run().await {
        error!(?e, "Retention cleanup failed");
    }

    Ok(())
}
