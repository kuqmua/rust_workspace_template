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
    pub use crate::admin_page_total_count::*;
}
mod admin_recent_login_failure_count {
    pub use crate::admin_recent_login_failure_count::*;
}
mod admin_repository_error {
    pub use crate::admin_repository_error::*;
}
mod admin_sign_in_user {
    pub use crate::admin_sign_in_user::*;
}
mod replace_role_permissions_outcome {
    pub use crate::replace_role_permissions_outcome::*;
}
mod replace_user_roles_outcome {
    pub use crate::replace_user_roles_outcome::*;
}
mod repository_page_total {
    pub use crate::repository_page_total::*;
}
mod sqlx_admin_repository_connection_mut_ref {
    pub use crate::sqlx_admin_repository_connection_mut_ref::*;
}
mod sqlx_admin_repository_pool_ref {
    pub use crate::sqlx_admin_repository_pool_ref::*;
}
pub(crate) mod data_tables {
    pub use crate::adapters_repository_data_tables::*;
}
pub(crate) mod insert_audit_success {
    pub use crate::insert_audit_success::*;
}
pub(crate) mod insert_user {
    pub use crate::insert_user::*;
}
pub(crate) mod query_audit_log {
    pub use crate::query_audit_log::*;
}
pub(crate) mod read_settings {
    pub use crate::read_settings::*;
}
pub(crate) mod revoke_access_session {
    pub use crate::revoke_access_session::*;
}
pub(crate) mod revoke_refresh_token {
    pub use crate::revoke_refresh_token::*;
}
pub(crate) mod revoke_user_sessions {
    pub use crate::revoke_user_sessions::*;
}
pub(crate) mod roles {
    pub use crate::adapters_repository_roles::*;
}
pub(crate) mod update_user_password {
    pub use crate::update_user_password::*;
}
