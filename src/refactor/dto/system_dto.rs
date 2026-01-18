//! System API DTOs
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct LogQuery {
    pub cursor: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedLogResponse {
    pub date: String,
    pub lines: Vec<String>,
    pub next_cursor: Option<usize>,
}
