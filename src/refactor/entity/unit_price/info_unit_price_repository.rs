use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_api_repository_trait::InfoUnitPriceApiRepository;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_collector_repository_trait::InfoUnitPriceCollectorRepository;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_entity::InfoUnitPriceEntity;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_fs_adapter::InfoUnitPriceFsAdapter;
use anyhow::Result;
use tracing::error;

/// Unified repository for unit price data backed by the filesystem adapter.
/// Implements both API and collector traits to keep wiring consistent.
pub struct InfoUnitPriceRepository {
    adapter: InfoUnitPriceFsAdapter,
}

impl InfoUnitPriceRepository {
    pub fn new() -> Self {
        Self {
            adapter: InfoUnitPriceFsAdapter,
        }
    }
}

impl Default for InfoUnitPriceRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InfoUnitPriceApiRepository for InfoUnitPriceRepository {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoUnitPriceEntity> {
        &self.adapter
    }

    fn read(&self) -> Result<InfoUnitPriceEntity> {
        self.adapter.read().map_err(|err| {
            error!(error = %err, "Failed to read unit price data from FS");
            err
        })
    }

    fn update(&self, data: &InfoUnitPriceEntity) -> Result<()> {
        self.adapter.update(data).map_err(|err| {
            error!(error = %err, "Failed to update unit price data on FS");
            err
        })
    }
}

impl InfoUnitPriceCollectorRepository for InfoUnitPriceRepository {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoUnitPriceEntity> {
        &self.adapter
    }
}
