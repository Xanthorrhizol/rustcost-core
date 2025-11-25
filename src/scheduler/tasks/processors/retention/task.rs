use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use crate::scheduler::tasks::processors::retention;
use crate::core::persistence::info::fixed::setting::info_setting_retention_repository_trait::InfoSettingRetentionRepository;

pub struct RetentionTask<R: InfoSettingRetentionRepository> {
    pub settings_repo: R,
}

impl<R: InfoSettingRetentionRepository> RetentionTask<R> {
    pub fn new(repo: R) -> Self {
        Self { settings_repo: repo }
    }

    pub async fn run(&self, now: DateTime<Utc>) -> Result<()> {
        let settings = self.settings_repo.read()?;  // Load config

        let minute_before = now - Duration::days(settings.minute_retention_days.into());
        let hour_before   = now - Duration::days((settings.hour_retention_months * 30).into());
        let day_before    = now - Duration::days((settings.day_retention_years * 365).into());

        retention::pod::task::run(minute_before, hour_before, day_before).await?;
        retention::node::task::run(minute_before, hour_before, day_before).await?;
        retention::container::task::run(minute_before, hour_before, day_before).await?;

        Ok(())
    }
}
