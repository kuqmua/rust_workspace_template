pub(crate) use admin_page_total_count::AdminPageTotalCount;
pub(crate) use admin_recent_login_failure_count::AdminRecentLoginFailureCount;
pub(crate) use admin_repository_error::AdminRepositoryError;
pub(crate) use admin_sign_in_user::AdminSignInUser;
pub(crate) use replace_role_permissions_outcome::ReplaceRolePermissionsOutcome;
pub(crate) use replace_user_roles_outcome::ReplaceUserRolesOutcome;
pub(crate) use repository_page_total::repository_page_total;
pub(crate) use sqlx_admin_repository_connection_mut_ref::SqlxAdminRepositoryConnectionMutRef;
pub(crate) use sqlx_admin_repository_pool_ref::SqlxAdminRepositoryPoolRef;

// Root-owned module compatibility wrappers.
mod admin_page_total_count {
    pub use super::super::admin_page_total_count::*;
}
mod admin_recent_login_failure_count {
    pub use super::super::admin_recent_login_failure_count::*;
}
mod admin_repository_error {
    pub use super::super::admin_repository_error::*;
}
mod admin_sign_in_user {
    pub use super::super::admin_sign_in_user::*;
}
mod replace_role_permissions_outcome {
    pub use super::super::replace_role_permissions_outcome::*;
}
mod replace_user_roles_outcome {
    pub use super::super::replace_user_roles_outcome::*;
}
mod repository_page_total {
    pub use super::super::repository_page_total::*;
}
mod sqlx_admin_repository_connection_mut_ref {
    pub use super::super::sqlx_admin_repository_connection_mut_ref::*;
}
mod sqlx_admin_repository_pool_ref {
    pub use super::super::sqlx_admin_repository_pool_ref::*;
}
pub(crate) mod data_tables {
    pub use super::super::adapters_repository_data_tables::*;
}
pub(crate) mod insert_audit_success {
    pub use super::super::insert_audit_success::*;
}
pub(crate) mod insert_user {
    pub use super::super::insert_user::*;
}
pub(crate) mod query_audit_log {
    pub use super::super::query_audit_log::*;
}
pub(crate) mod read_settings {
    pub use super::super::read_settings::*;
}
pub(crate) mod revoke_access_session {
    pub use super::super::revoke_access_session::*;
}
pub(crate) mod revoke_refresh_token {
    pub use super::super::revoke_refresh_token::*;
}
pub(crate) mod revoke_user_sessions {
    pub use super::super::revoke_user_sessions::*;
}
pub(crate) mod roles {
    pub use super::super::adapters_repository_roles::*;
}
pub(crate) mod update_user_password {
    pub use super::super::update_user_password::*;
}
