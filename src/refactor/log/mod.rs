pub mod actor;

use crate::refactor::dto::PaginatedLogResponse;

pub use actor::LogActor;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum LogActorMessage {
    GetSystemLogFileList,
    GetSystemLogLines {
        date: String,
        cursor: Option<usize>,
        limit: Option<usize>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LogActorResult {
    VecString(Vec<String>),
    PaginatedLog(PaginatedLogResponse),
}
