use super::info_setting_entity::InfoSettingEntity;
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use anyhow::Result;

/// Collector repository trait for settings.
/// Collector may read and occasionally create/update settings locally.
pub trait InfoSettingRetentionRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoSettingEntity>;
    fn read(&self) -> Result<InfoSettingEntity> {
        self.fs_adapter().read()
    }
}
