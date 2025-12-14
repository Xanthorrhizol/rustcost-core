use anyhow::{bail, Result};
use chrono::NaiveDate;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};
use crate::core::persistence::storage_path::get_rustcost_base_path;
use tokio::task;
const LOG_PREFIX: &str = "app.log.";

pub struct LogFsAdapter;

impl LogFsAdapter {
    /// logs directory = $RUSTCOST_BASE_PATH/logs
    fn dir() -> PathBuf {
        get_rustcost_base_path().join("logs")
    }

    /// log file = $BASE/logs/app.log.YYYY-MM-DD
    fn log_path(date: &str) -> PathBuf {
        Self::dir().join(format!("{}{}", LOG_PREFIX, date))
    }

    pub fn get_system_log_file_list(&self) -> Result<Vec<String>> {
        let dir = Self::dir();

        // If logs directory does not exist → return empty list
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => return Ok(vec![]),
        };

        let mut list = Vec::new();

        for entry in entries {
            let entry = entry?;
            let filename = entry.file_name();
            let filename = match filename.to_str() {
                Some(f) => f,
                None => continue, // skip non-UTF8 names
            };

            // Ensure it starts with "app.log."
            let Some(date_part) = filename.strip_prefix(LOG_PREFIX) else {
                continue;
            };

            list.push(date_part.to_string());
        }

        // Sort newest → oldest
        list.sort_by(|a, b| {
            NaiveDate::parse_from_str(b, "%Y-%m-%d")
                .unwrap_or(NaiveDate::MIN)
                .cmp(&NaiveDate::parse_from_str(a, "%Y-%m-%d").unwrap_or(NaiveDate::MIN))
        });

        Ok(list)
    }


    pub async fn get_system_log_lines(
        &self,
        date: &str,
        cursor: usize,
        limit: usize,
    ) -> anyhow::Result<(Vec<String>, Option<usize>)> {
        let path = Self::log_path(&date);

        if !path.exists() {
            bail!("Log file not found for date: {}", date);
        }

        let result = task::spawn_blocking(move || -> anyhow::Result<(Vec<String>, Option<usize>)> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);

            let lines = reader
                .lines()
                .skip(cursor)
                .take(limit)
                .collect::<Result<Vec<_>, _>>()?;

            let next_cursor = if lines.len() < limit {
                None
            } else {
                Some(cursor + lines.len())
            };

            Ok((lines, next_cursor))
        })
            .await?; // join handle

        result
    }
}
