use crate::core::persistence::info::k8s::container::info_container_api_repository_trait::InfoContainerApiRepository;
use crate::core::persistence::info::k8s::container::info_container_collector_repository_trait::InfoContainerCollectorRepository;
use crate::core::persistence::info::k8s::container::info_container_entity::InfoContainerEntity;
use crate::core::persistence::info::k8s::container::info_container_fs_adapter::InfoContainerFsAdapter;
use crate::core::persistence::info::k8s::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use anyhow::Result;
use tracing::error;

/// Repository for container info that wires the API/collector traits to the FS adapter.
pub struct InfoContainerRepository {
    adapter: InfoContainerFsAdapter,
}

impl InfoContainerRepository {
    pub fn new() -> Self {
        Self {
            adapter: InfoContainerFsAdapter,
        }
    }
}

impl Default for InfoContainerRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InfoContainerApiRepository for InfoContainerRepository {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoContainerEntity> {
        &self.adapter
    }

    fn read(&self, container_key: &str) -> Result<InfoContainerEntity> {
        self.adapter.read(container_key).map_err(|err| {
            error!(error = %err, container_key, "Failed to read container info");
            err
        })
    }

    fn update(&self, data: &InfoContainerEntity) -> Result<()> {
        self.adapter.update(data).map_err(|err| {
            error!(error = %err, container_name = ?data.container_name, "Failed to update container info");
            err
        })
    }
}

impl InfoContainerCollectorRepository for InfoContainerRepository {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoContainerEntity> {
        &self.adapter
    }

    fn exists(&self, container_key: &str) -> Result<bool> {
        self.adapter.exists(container_key).map_err(|err| {
            error!(error = %err, container_key, "Failed to check container info existence");
            err
        })
    }

    fn create_if_missing(&self, container_key: &str, data: &InfoContainerEntity) -> Result<bool> {
        if self.adapter.exists(container_key)? {
            return Ok(false);
        }

        self.adapter.insert(data).map_err(|err| {
            error!(error = %err, container_key, "Failed to create container info if missing");
            err
        })?;

        Ok(true)
    }
}
