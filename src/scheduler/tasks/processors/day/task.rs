use anyhow::Result;
use chrono::{DateTime, Utc};
use tracing::{debug};
use crate::scheduler::tasks::processors::day::pod::task::process_pod_hour_to_day;
use crate::scheduler::tasks::processors::day::node::task::process_node_hour_to_day;
use crate::scheduler::tasks::processors::day::container::task::process_container_hour_to_day;

pub async fn run(now: DateTime<Utc>) -> Result<()> {
    debug!("Running day aggregation task...");

    process_pod_hour_to_day(now)
        .await
        .expect("Failed to process pod hour-to-day aggregation");
    process_container_hour_to_day(now)
        .await
        .expect("Failed to process container hour-to-day aggregation");
    process_node_hour_to_day(now)
        .await
        .expect("Failed to process node hour-to-day aggregation");

    Ok(())
}
