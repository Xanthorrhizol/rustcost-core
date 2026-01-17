use crate::core::persistence::info::fixed::version::info_version_api_repository_trait::InfoVersionApiRepository;
use crate::core::persistence::info::fixed::version::info_version_entity::InfoVersionEntity;
use crate::core::persistence::info::fixed::version::info_version_repository::InfoVersionRepositoryImpl;
use anyhow::Result;

pub async fn get_info_versions() -> Result<InfoVersionEntity> {
    let repo = InfoVersionRepositoryImpl::new();
    get_info_versions_with_repo(&repo).await
}

async fn get_info_versions_with_repo<R: InfoVersionApiRepository>(
    repo: &R,
) -> Result<InfoVersionEntity> {
    let entity = repo.read()?;
    Ok(entity)
}
