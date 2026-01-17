use anyhow::Result;
use k8s_openapi::api::core::v1::PersistentVolume;

use crate::api::dto::paginated_response::PaginatedResponse;
use crate::core::client::k8s::client_k8s_persistent_volume;
use crate::core::client::k8s::util::{build_client, read_token};

pub async fn get_k8s_persistent_volumes_paginated(
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<PaginatedResponse<PersistentVolume>> {
    const DEFAULT_LIMIT: usize = 50;

    let token = read_token()?;
    let client = build_client()?;

    let volumes = client_k8s_persistent_volume::fetch_persistent_volumes(&token, &client).await?;
    let total = volumes.len();

    let offset = offset.unwrap_or(0).min(total);
    let limit = limit.unwrap_or(DEFAULT_LIMIT);
    let end = (offset + limit).min(total);

    let items = volumes
        .into_iter()
        .skip(offset)
        .take(end.saturating_sub(offset))
        .collect();

    Ok(PaginatedResponse {
        items,
        total,
        limit: end.saturating_sub(offset),
        offset,
    })
}

pub async fn get_k8s_persistent_volume(name: String) -> Result<PersistentVolume> {
    let token = read_token()?;
    let client = build_client()?;

    client_k8s_persistent_volume::fetch_persistent_volume_by_name(&token, &client, &name).await
}
