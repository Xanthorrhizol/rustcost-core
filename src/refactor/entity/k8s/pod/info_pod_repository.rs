use crate::core::persistence::info::k8s::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use crate::core::persistence::info::k8s::pod::info_pod_api_repository_trait::InfoPodApiRepository;
use crate::core::persistence::info::k8s::pod::info_pod_collector_repository_trait::InfoPodCollectorRepository;
use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;
use crate::core::persistence::info::k8s::pod::info_pod_fs_adapter::InfoPodFsAdapter;
use anyhow::Result;
use tracing::error;

/// Repository for pod info bridging traits to the filesystem adapter.
pub struct InfoPodRepository {
    adapter: InfoPodFsAdapter,
}

impl InfoPodRepository {
    pub fn new() -> Self {
        Self {
            adapter: InfoPodFsAdapter,
        }
    }
}

impl Default for InfoPodRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InfoPodApiRepository for InfoPodRepository {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoPodEntity> {
        &self.adapter
    }

    fn read(&self, pod_name: &str) -> Result<InfoPodEntity> {
        self.adapter.read(pod_name).map_err(|err| {
            error!(error = %err, pod_name, "Failed to read pod info");
            err
        })
    }

    fn update(&self, data: &InfoPodEntity) -> Result<()> {
        self.adapter.update(data).map_err(|err| {
            error!(error = %err, pod_name = ?data.pod_name, "Failed to update pod info");
            err
        })
    }
}

impl InfoPodCollectorRepository for InfoPodRepository {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoPodEntity> {
        &self.adapter
    }

    fn exists(&self, pod_name: &str) -> Result<bool> {
        self.adapter.exists(pod_name).map_err(|err| {
            error!(error = %err, pod_name, "Failed to check pod info existence");
            err
        })
    }

    fn create_if_missing(&self, pod_name: &str, data: &InfoPodEntity) -> Result<bool> {
        if self.adapter.exists(pod_name)? {
            return Ok(false);
        }

        self.adapter.insert(data).map_err(|err| {
            error!(error = %err, pod_name, "Failed to create pod info if missing");
            err
        })?;

        Ok(true)
    }
}
