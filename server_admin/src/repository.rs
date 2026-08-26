#![allow(clippy::single_call_fn)] // shared repository boundary types support thematic SQL owner modules

#[path = "adapters_repository_data_tables.rs"]
pub(crate) mod data_tables;
#[path = "insert_audit_success.rs"]
pub(crate) mod insert_audit_success;
#[path = "insert_user.rs"]
pub(crate) mod insert_user;
#[path = "query_audit_log.rs"]
pub(crate) mod query_audit_log;
#[path = "read_settings.rs"]
pub(crate) mod read_settings;
#[path = "revoke_access_session.rs"]
pub(crate) mod revoke_access_session;
#[path = "revoke_refresh_token.rs"]
pub(crate) mod revoke_refresh_token;
#[path = "revoke_user_sessions.rs"]
pub(crate) mod revoke_user_sessions;
#[path = "adapters_repository_roles.rs"]
pub(crate) mod roles;
#[path = "update_user_password.rs"]
pub(crate) mod update_user_password;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminRepositoryError {
    #[error("stored admin value does not satisfy its contract")]
    InvalidStoredValue,
    #[error("admin repository query failed: {0:?}")]
    Sqlx(crate::domain_types::SqlxAdminError),
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReplaceRolePermissionsOutcome {
    MissingRole,
    SystemRole,
    StaleAssignment,
    UnknownPermission,
    Updated,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReplaceUserRolesOutcome {
    LastActiveAdministrator,
    MissingUser,
    StaleAssignment,
    UnknownRole,
    Updated,
}
impl From<crate::domain_types::SqlxAdminError> for AdminRepositoryError {
    fn from(error: crate::domain_types::SqlxAdminError) -> Self {
        Self::Sqlx(error)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub(crate) struct SqlxAdminRepositoryConnectionMutRef<'connection_lt>(
    &'connection_lt mut sqlx::PgConnection,
);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(crate) struct SqlxAdminRepositoryPoolRef<'pool_lt>(&'pool_lt sqlx::PgPool);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct AdminRecentLoginFailureCount(i64);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct AdminPageTotalCount(i64);
impl AdminPageTotalCount {
    pub(crate) const fn get(self) -> i64 {
        self.0
    }
}
pub(crate) fn page_total(
    value: AdminPageTotalCount,
) -> Result<server_admin_contract::domain_types::AdminPageTotal, AdminRepositoryError> {
    u64::try_from(value.get())
        .map(server_admin_contract::domain_types::AdminPageTotal::from)
        .map_err(|_error| AdminRepositoryError::InvalidStoredValue)
}
impl AdminRecentLoginFailureCount {
    pub(crate) fn reached(
        self,
        threshold: crate::domain_types::auth::StdAdminFailureThreshold,
    ) -> crate::domain_types::StdAdminBool {
        crate::domain_types::StdAdminBool::from(self.0 >= i64::from(threshold))
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct AdminSignInUser {
    id: crate::domain_types::AdminUserId,
    password_hash: crate::domain_types::AdminPasswordHash,
    is_banned: crate::domain_types::StdAdminBool,
}
impl TryFrom<(i64, String, bool)> for AdminSignInUser {
    type Error = crate::domain_types::SqlxAdminError;
    fn try_from((id, password_hash, is_banned): (i64, String, bool)) -> Result<Self, Self::Error> {
        Ok(Self {
            id: crate::domain_types::AdminUserId::try_from(id)?,
            password_hash: crate::domain_types::AdminPasswordHash::new(
                pg_types_text_misc::StringAsNonNullTextSecret::from(password_hash),
            ),
            is_banned: crate::domain_types::StdAdminBool::from(is_banned),
        })
    }
}
impl From<AdminSignInUser>
    for (
        crate::domain_types::AdminUserId,
        crate::domain_types::AdminPasswordHash,
        crate::domain_types::StdAdminBool,
    )
{
    fn from(value: AdminSignInUser) -> Self {
        (value.id, value.password_hash, value.is_banned)
    }
}
