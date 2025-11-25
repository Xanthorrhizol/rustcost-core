use anyhow::Result;
use chrono::Utc;
use tracing::{debug, error};

pub async fn run() -> Result<()> {
    let now = Utc::now();
    debug!("Running hour scheduler at {}", now);

    if let Err(e) = super::processors::hour::run(now).await {
        error!(?e, "hour aggregator failed");
    }

    Ok(())
}
