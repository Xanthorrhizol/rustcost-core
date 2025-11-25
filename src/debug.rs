use tracing::info;
use crate::scheduler;

/// Runs only when in RUSTCOST_DEBUG_MODE
pub async fn run_debug() {
    info!("🔧 Debug mode: running debug tasks...");
    scheduler::tasks::hour_task().await.expect("TODO: panic message");
    info!("Debug tasks completed. Exiting...");
}