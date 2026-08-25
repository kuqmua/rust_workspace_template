#![allow(clippy::single_call_fn)] // shared repository boundary types support thematic SQL owner modules

pub(crate) mod audit;
pub(crate) mod cleanup;
pub(crate) mod data_tables;
pub(crate) mod permissions;
pub(crate) mod rate_limits;
pub(crate) mod roles;
pub(crate) mod sessions;
pub(crate) mod settings;
pub(crate) mod users;

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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AdminRateLimitOutcome {
    Allowed,
    Limited,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) enum AdminRateLimitRepositoryError {
    InvalidPolicy,
    Sqlx(crate::domain_types::SqlxAdminError),
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum AdminRepositoryDbRef<'connection_lt, 'pool_lt> {
    Connection(SqlxAdminRepositoryConnectionMutRef<'connection_lt>),
    Pool(SqlxAdminRepositoryPoolRef<'pool_lt>),
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub(crate) struct AdminAuthenticatedRecord {
    display_name: server_admin_contract::domain_types::AdminDisplayName,
    login: server_admin_contract::domain_types::AdminLogin,
    permissions: crate::domain_types::AdminPermissions,
    roles: crate::domain_types::AdminRoleNames,
    password_change_required: crate::domain_types::AdminPasswordChangeRequired,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug)]
pub(crate) struct AdminCleanupRepositoryReport {
    access_sessions: crate::domain_types::AdminCleanupRows,
    audit_log: crate::domain_types::AdminCleanupRows,
    login_attempts: crate::domain_types::AdminCleanupRows,
    rate_limits: crate::domain_types::AdminCleanupRows,
    refresh_tokens: crate::domain_types::AdminCleanupRows,
}
impl AdminCleanupRepositoryReport {
    pub(crate) const fn into_parts(
        self,
    ) -> (
        crate::domain_types::AdminCleanupRows,
        crate::domain_types::AdminCleanupRows,
        crate::domain_types::AdminCleanupRows,
        crate::domain_types::AdminCleanupRows,
        crate::domain_types::AdminCleanupRows,
    ) {
        (
            self.access_sessions,
            self.audit_log,
            self.login_attempts,
            self.rate_limits,
            self.refresh_tokens,
        )
    }
}
impl AdminAuthenticatedRecord {
    pub(crate) fn into_parts(
        self,
    ) -> (
        server_admin_contract::domain_types::AdminDisplayName,
        server_admin_contract::domain_types::AdminLogin,
        crate::domain_types::AdminPasswordChangeRequired,
        crate::domain_types::AdminPermissions,
        crate::domain_types::AdminRoleNames,
    ) {
        (
            self.display_name,
            self.login,
            self.password_change_required,
            self.permissions,
            self.roles,
        )
    }
}
impl From<crate::domain_types::SqlxAdminError> for AdminRepositoryError {
    fn from(error: crate::domain_types::SqlxAdminError) -> Self {
        Self::Sqlx(error)
    }
}
pub(super) fn invalid_stored_value<Error>(_error: Error) -> AdminRepositoryError {
    AdminRepositoryError::InvalidStoredValue
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct SqlxAdminRepositoryConnectionMutRef<'connection_lt>(
    &'connection_lt mut sqlx::PgConnection,
);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
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
        .map_err(invalid_stored_value)
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
impl AdminSignInUser {
    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::domain_types::AdminUserId,
        crate::domain_types::AdminPasswordHash,
        crate::domain_types::StdAdminBool,
    ) {
        (self.id, self.password_hash, self.is_banned)
    }
}
