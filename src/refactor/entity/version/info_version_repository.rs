use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use crate::core::persistence::info::fixed::version::info_version_api_repository_trait::InfoVersionApiRepository;
use crate::core::persistence::info::fixed::version::info_version_collector_repository_trait::InfoVersionCollectorRepository;
use crate::core::persistence::info::fixed::version::info_version_entity::InfoVersionEntity;
use crate::core::persistence::info::fixed::version::info_version_fs_adapter::InfoVersionFsAdapter;
use crate::core::persistence::info::fixed::version::info_version_repository_trait::InfoVersionRepository;
use anyhow::Result;
use tracing::error;

/// Unified repository for version metadata backed by the filesystem adapter.
pub struct InfoVersionRepositoryImpl {
    adapter: InfoVersionFsAdapter,
}

impl InfoVersionRepositoryImpl {
    pub fn new() -> Self {
        Self {
            adapter: InfoVersionFsAdapter,
        }
    }
}

impl Default for InfoVersionRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl InfoVersionApiRepository for InfoVersionRepositoryImpl {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoVersionEntity> {
        &self.adapter
    }

    fn read(&self) -> Result<InfoVersionEntity> {
        self.adapter.read().map_err(|err| {
            error!(error = %err, "Failed to read version data from FS");
            err
        })
    }
}

impl InfoVersionCollectorRepository for InfoVersionRepositoryImpl {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoVersionEntity> {
        &self.adapter
    }
}

impl InfoVersionRepository for InfoVersionRepositoryImpl {
    fn get(&self) -> Result<InfoVersionEntity> {
        self.adapter.read()
    }

    fn insert(&self, data: &InfoVersionEntity) -> Result<()> {
        self.adapter.insert(data)
    }

    fn update(&self, data: &InfoVersionEntity) -> Result<()> {
        self.adapter.update(data)
    }
}
