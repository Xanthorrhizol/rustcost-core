use crate::core::persistence::logs::log_repository::LogRepository;
use crate::api::dto::system_dto::PaginatedLogResponse;

pub struct LogService<R: LogRepository> {
    repo: R,
}

impl<R: LogRepository> LogService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn get_system_log_file_list(
        &self,
    ) -> anyhow::Result<Vec<String>> {
        self.repo.get_system_log_file_list()
    }

    pub async fn get_system_log_lines(
        &self,
        date: &str,
        cursor: Option<usize>,
        limit: Option<usize>,
    ) -> anyhow::Result<PaginatedLogResponse> {

        let cursor = cursor.unwrap_or(0);
        let limit = limit.unwrap_or(100);

        let (lines, next_cursor) = self
            .repo
            .get_system_log_lines(date, cursor, limit)
            .await?;

        Ok(PaginatedLogResponse {
            date: date.to_string(),
            lines,
            next_cursor,
        })
    }
}
