use super::{
    AdminDisplayName, AdminLogin, AdminPasswordHashError, AdminPasswordHasher, AdminPermission,
    AdminUserId, SqlxAdminError,
};

#[path = "maintenance/admin_audit_action.rs"]
mod admin_audit_action;
pub use admin_audit_action::*;
#[path = "maintenance/admin_audit_resource.rs"]
mod admin_audit_resource;
pub use admin_audit_resource::*;
#[path = "maintenance/sqlx_admin_migrate_error.rs"]
mod sqlx_admin_migrate_error;
pub use sqlx_admin_migrate_error::*;
#[path = "maintenance/admin_migrate_error_inner.rs"]
mod admin_migrate_error_inner;
use admin_migrate_error_inner::*;
#[path = "maintenance/admin_migrate_error.rs"]
mod admin_migrate_error;
pub use admin_migrate_error::*;
#[path = "maintenance/prepare_postgresql.rs"]
mod prepare_postgresql;
pub use prepare_postgresql::*;
#[path = "maintenance/admin_cleanup_batch_size.rs"]
mod admin_cleanup_batch_size;
pub use admin_cleanup_batch_size::*;
#[path = "maintenance/admin_cleanup_retention_seconds.rs"]
mod admin_cleanup_retention_seconds;
pub use admin_cleanup_retention_seconds::*;
#[path = "maintenance/admin_cleanup_cfg.rs"]
mod admin_cleanup_cfg;
pub use admin_cleanup_cfg::*;
#[path = "maintenance/admin_cleanup_report.rs"]
mod admin_cleanup_report;
pub use admin_cleanup_report::*;
#[path = "maintenance/admin_cleanup_rows.rs"]
mod admin_cleanup_rows;
pub use admin_cleanup_rows::*;
#[path = "maintenance/admin_cleanup_cfg_error.rs"]
mod admin_cleanup_cfg_error;
pub use admin_cleanup_cfg_error::*;
#[path = "maintenance/admin_cleanup_error.rs"]
mod admin_cleanup_error;
pub use admin_cleanup_error::*;
#[path = "maintenance/cleanup_admin_tables.rs"]
mod cleanup_admin_tables;
pub use cleanup_admin_tables::*;
#[path = "maintenance/initial_administrator_creation_error.rs"]
mod initial_administrator_creation_error;
pub use initial_administrator_creation_error::*;
#[path = "maintenance/admin_password_reset_error.rs"]
mod admin_password_reset_error;
pub use admin_password_reset_error::*;
#[path = "maintenance/create_initial_administrator.rs"]
mod create_initial_administrator;
pub use create_initial_administrator::*;
#[path = "maintenance/reset_admin_password.rs"]
mod reset_admin_password;
pub use reset_admin_password::*;

#[cfg(test)]
mod tests {
    #[test]
    fn cleanup_batch_rejects_zero() {
        assert_eq!(
            super::AdminCleanupBatchSize::try_from(constants_i64::ZERO),
            Err(super::AdminCleanupCfgError::BatchSizeOutOfRange),
        );
    }
}
