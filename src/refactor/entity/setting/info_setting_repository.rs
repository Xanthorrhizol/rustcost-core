use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use super::info_setting_entity::InfoSettingEntity;
use super::info_setting_fs_adapter::InfoSettingFsAdapter;
use super::info_setting_api_repository_trait::InfoSettingApiRepository;
use super::info_setting_collector_repository_trait::InfoSettingCollectorRepository;
use super::info_setting_retention_repository_trait::InfoSettingRetentionRepository;

pub struct InfoSettingRepository {
    adapter: InfoSettingFsAdapter,
}

impl InfoSettingRepository {
    pub fn new() -> Self {
        Self {
            adapter: InfoSettingFsAdapter::new(),
        }
    }
}

impl InfoSettingApiRepository for InfoSettingRepository {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoSettingEntity> {
        &self.adapter
    }
}
impl InfoSettingCollectorRepository for InfoSettingRepository {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoSettingEntity> {
        &self.adapter
    }
}

impl InfoSettingRetentionRepository for InfoSettingRepository {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoSettingEntity> {
        &self.adapter
    }
}
