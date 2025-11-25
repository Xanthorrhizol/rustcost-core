use anyhow::Result;
use chrono::Utc;
use tracing::{debug, error};
use crate::core::persistence::info::fixed::setting::info_setting_repository::InfoSettingRepository;
use crate::scheduler::tasks::processors::retention::task::RetentionTask;

pub async fn run() -> Result<()> {
    let now = Utc::now();
    debug!("Running day task (aggregation + retention)...");

    if let Err(e) = super::processors::day::run(now).await {
        error!(?e, "Daily aggregator failed");
    }

    // Create settings repository DI
    let settings_repo = InfoSettingRepository::new();
    let retention_task = RetentionTask::new(settings_repo);

    if let Err(e) = retention_task.run(now).await {
        error!(?e, "Retention cleanup failed");
    }

    Ok(())
}
