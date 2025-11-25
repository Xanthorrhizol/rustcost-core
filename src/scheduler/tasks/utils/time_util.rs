use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Timelike, Utc};

pub struct TimeUtils;

impl TimeUtils {

    /// Returns the start and end of the **previous full hour**.
    ///
    /// This function:
    /// - Rounds the supplied `now` timestamp *down* to the beginning of the current hour
    ///   (minute=0, second=0, nanosecond=0)
    /// - Uses that rounded timestamp as the `end` boundary
    /// - Subtracts one hour to compute the `start` boundary
    ///
    /// Example:
    /// If `now = 2025-11-26T17:23:45Z`, this returns:
    /// - `start = 2025-11-26T16:00:00Z`
    /// - `end   = 2025-11-26T17:00:00Z`
    ///
    /// Fails only if `with_minute`, `with_second`, or `with_nanosecond` return `None`,
    /// which should never occur for valid UTC datetimes.
    pub fn previous_hour_window(now: DateTime<Utc>) -> Result<(DateTime<Utc>, DateTime<Utc>)> {
        let end = now
            .with_minute(0).context("failed to set minute to 0")?
            .with_second(0).context("failed to set second to 0")?
            .with_nanosecond(0).context("failed to set nanosecond to 0")?;

        let start = end - Duration::hours(1);

        Ok((start, end))
    }

    /// Returns the start and end of the **previous full day**, aligned to UTC midnight.
    ///
    /// This function:
    /// - Extracts the calendar date from the provided `now` timestamp (UTC)
    /// - Constructs the midnight timestamp for the *current* day (`end`)
    /// - Computes the midnight timestamp for the *previous* day (`start`)
    ///
    /// Example:
    /// If `now = 2025-11-26T17:23:45Z`, this returns:
    /// - `start = 2025-11-25T00:00:00Z`
    /// - `end   = 2025-11-26T00:00:00Z`
    ///
    /// This function never fails and performs no allocations.
    #[inline]
    pub(crate) fn previous_day_window(now: DateTime<Utc>) -> (DateTime<Utc>, DateTime<Utc>) {
        // Extract today's date in UTC (no time component)
        let today = now.date_naive();

        // Construct midnight (start of today) using the modern chrono API
        let end = DateTime::from_naive_utc_and_offset(
            today.and_hms_opt(0, 0, 0).unwrap(),
            Utc,
        );

        // Midnight of the previous day
        let start = end - Duration::days(1);

        (start, end)
    }
}
