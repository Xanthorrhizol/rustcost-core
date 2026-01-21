//! Info routes (e.g., /api/v1/info/*)

use super::{info_live_routes, info_stored_routes};
use axum::Router;
use xan_actor::ActorSystem;

pub fn info_routes() -> Router<ActorSystem> {
    Router::new()
        .merge(info_stored_routes::info_stored_routes())
        .merge(info_live_routes::info_live_routes())
}
