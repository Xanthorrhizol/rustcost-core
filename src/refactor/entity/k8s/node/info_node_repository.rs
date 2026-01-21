use crate::core::persistence::info::k8s::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use crate::core::persistence::info::k8s::node::info_node_api_repository_trait::InfoNodeApiRepository;
use crate::core::persistence::info::k8s::node::info_node_collector_repository_trait::InfoNodeCollectorRepository;
use crate::core::persistence::info::k8s::node::info_node_entity::InfoNodeEntity;
use crate::core::persistence::info::k8s::node::info_node_fs_adapter::InfoNodeFsAdapter;
use anyhow::Result;
use tracing::error;

/// Repository for node info that delegates to the filesystem adapter.
pub struct InfoNodeRepository {
    adapter: InfoNodeFsAdapter,
}

impl InfoNodeRepository {
    pub fn new() -> Self {
        Self {
            adapter: InfoNodeFsAdapter,
        }
    }
}

impl Default for InfoNodeRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InfoNodeApiRepository for InfoNodeRepository {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoNodeEntity> {
        &self.adapter
    }

    fn read(&self, node_name: &str) -> Result<InfoNodeEntity> {
        self.adapter.read(node_name).map_err(|err| {
            error!(error = %err, node_name, "Failed to read node info");
            err
        })
    }

    fn update(&self, data: &InfoNodeEntity) -> Result<()> {
        self.adapter.update(data).map_err(|err| {
            error!(error = %err, node_name = ?data.node_name, "Failed to update node info");
            err
        })
    }
}

impl InfoNodeCollectorRepository for InfoNodeRepository {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoNodeEntity> {
        &self.adapter
    }

    fn exists(&self, node_name: &str) -> Result<bool> {
        self.adapter.exists(node_name).map_err(|err| {
            error!(error = %err, node_name, "Failed to check node info existence");
            err
        })
    }

    fn create_if_missing(&self, node_name: &str, data: &InfoNodeEntity) -> Result<bool> {
        if self.adapter.exists(node_name)? {
            return Ok(false);
        }

        self.adapter.insert(data).map_err(|err| {
            error!(error = %err, node_name, "Failed to create node info if missing");
            err
        })?;

        Ok(true)
    }
}
