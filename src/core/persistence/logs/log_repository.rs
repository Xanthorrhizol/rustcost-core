
use anyhow::Result;
use crate::core::persistence::logs::log_fs_adapter::LogFsAdapter;

pub trait LogRepository: Send + Sync {
    fn fs(&self) -> &LogFsAdapter;

    fn get_system_log_file_list(&self) -> Result<Vec<String>> {
        self.fs().get_system_log_file_list()
    }

    #[allow(async_fn_in_trait)]
    async fn get_system_log_lines(
        &self,
        date: &str,
        cursor: usize,
        limit: usize,
    ) -> Result<(Vec<String>, Option<usize>)> {
        self.fs()
            .get_system_log_lines(date, cursor, limit)
            .await
    }
}

pub struct LogRepositoryImpl {
    adapter: LogFsAdapter,
}

impl LogRepositoryImpl {
    pub fn new() -> Self {
        Self {
            adapter: LogFsAdapter,
        }
    }
}

impl LogRepository for LogRepositoryImpl {
    fn fs(&self) -> &LogFsAdapter {
        &self.adapter
    }
}
