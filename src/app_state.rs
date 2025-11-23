use std::sync::Arc;
use crate::domain::system::service::log_service::LogService;
use crate::core::persistence::logs::log_repository::LogRepositoryImpl;

#[derive(Clone)]
pub struct AppState {
    pub log_service: Arc<LogService<LogRepositoryImpl>>,
}

pub fn build_app_state() -> AppState {
    AppState {
        log_service: Arc::new(LogService::new(LogRepositoryImpl::new())),
        // Add other services below when needed
        // user_service: Arc::new(UserService::new(UserRepositoryImpl::new())),
        // audit_service: Arc::new(AuditService::new(AuditRepositoryImpl::new())),
    }
}