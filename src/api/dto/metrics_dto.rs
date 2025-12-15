//! Metrics API DTOs

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::domain::metric::k8s::common::dto::MetricGranularity;

/// Represents the standard query parameters for fetching metrics.
///
/// This DTO handles three main concerns:
/// 1. **Time Range & Resolution**: Defining the window and granularity of data.
/// 2. **Pagination**: Controlling the size and order of the result set.
/// 3. **Filtering**: Narrowing down the scope to specific teams, services, or resources.
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct RangeQuery {
    // --- Time Range Configuration ---

    /// The start timestamp for the query window.
    /// Expected format: ISO 8601 (e.g., `2023-10-27T10:00:00`).
    /// If `None`, behavior depends on implementation (often defaults to a specific lookback window).
    pub start: Option<NaiveDateTime>,

    /// The end timestamp for the query window.
    /// If `None`, usually defaults to the current time.
    pub end: Option<NaiveDateTime>,

    /// Overrides the automatic data resolution.
    ///
    /// If not provided, the system may auto-calculate granularity based on the
    /// duration between `start` and `end`.
    /// Valid values: `minute`, `hour`, `day`.
    pub granularity: Option<MetricGranularity>,

    // --- Pagination & Sorting ---

    /// The maximum number of records to return (page size).
    pub limit: Option<usize>,

    /// The number of records to skip before starting to return results.
    pub offset: Option<usize>,

    /// The sort order string.
    /// Format convention: `field_name` (asc) or `-field_name` (desc).
    pub sort: Option<String>,

    // --- Scope Filters ---

    /// Filter metrics by the owning team.
    pub team: Option<String>,

    /// Filter metrics by specific microservice name.
    pub service: Option<String>,

    /// Filter by deployment environment.
    /// Common values: `"dev"`, `"stage"`, `"prod"`.
    pub env: Option<String>,

    /// Filter by Kubernetes namespace.
    pub namespace: Option<String>,

    /// Filter by resource labels.
    /// Expected format (convention-based):
    /// - `key=value`
    /// - `key1=value1,key2=value2`
    /// Example: `"app=api,tier=backend"`
    pub labels: Option<String>,

    // --- Resource Identification ---

    /// A unique identifier for a specific resource object.
    ///
    /// This is used to fetch metrics for a singular entity rather than an aggregate.
    /// Examples include:
    /// * Pod UID
    /// * Container Name + Pod UID
    /// * Node Name
    pub key: Option<String>
}