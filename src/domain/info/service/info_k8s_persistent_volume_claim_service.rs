use anyhow::Result;
use k8s_openapi::api::core::v1::PersistentVolumeClaim;

use crate::api::dto::paginated_response::PaginatedResponse;
use crate::core::client::k8s::client_k8s_persistent_volume_claim;
use crate::core::client::k8s::util::{build_client, read_token};

pub async fn get_k8s_persistent_volume_claims_paginated(
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<PaginatedResponse<PersistentVolumeClaim>> {
    const DEFAULT_LIMIT: usize = 50;

    let token = read_token()?;
    let client = build_client()?;

    let claims =
        client_k8s_persistent_volume_claim::fetch_persistent_volume_claims(&token, &client).await?;
    let total = claims.len();

    let offset = offset.unwrap_or(0).min(total);
    let limit = limit.unwrap_or(DEFAULT_LIMIT);
    let end = (offset + limit).min(total);

    let items = claims
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

pub async fn get_k8s_persistent_volume_claim(
    namespace: String,
    name: String,
) -> Result<PersistentVolumeClaim> {
    let token = read_token()?;
    let client = build_client()?;

    client_k8s_persistent_volume_claim::fetch_persistent_volume_claim_by_name_and_namespace(
        &token, &client, &namespace, &name,
    )
    .await
}
