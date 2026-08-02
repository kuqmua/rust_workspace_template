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

#[derive(optml::Optml, Debug, thiserror::Error)]
pub(crate) enum AdminRepositoryError {
    #[error("stored admin value does not satisfy its contract")]
    InvalidStoredValue,
    #[error("admin repository query failed: {0:?}")]
    Sqlx(super::SqlxAdminError),
}
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReplaceRolePermissionsOutcome {
    MissingRole,
    SystemRole,
    StaleAssignment,
    UnknownPermission,
    Updated,
}
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReplaceUserRolesOutcome {
    LastActiveAdministrator,
    MissingUser,
    StaleAssignment,
    UnknownRole,
    Updated,
}
#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AdminRateLimitOutcome {
    Allowed,
    Limited,
}
#[derive(optml::Optml, Debug)]
pub(crate) enum AdminRateLimitRepositoryError {
    InvalidPolicy,
    Sqlx(super::SqlxAdminError),
}
#[derive(optml::Optml)]
pub(crate) enum AdminRepositoryDbRef<'connection_lt, 'pool_lt> {
    Connection(SqlxAdminRepositoryConnectionMutRef<'connection_lt>),
    Pool(SqlxAdminRepositoryPoolRef<'pool_lt>),
}
#[derive(optml::Optml, Debug)]
pub(crate) struct AdminAuthenticatedRecord {
    display_name: server_admin_contract::AdminDisplayName,
    login: server_admin_contract::AdminLogin,
    permissions: super::AdminPermissions,
    roles: super::AdminRoleNames,
    password_change_required: super::AdminPasswordChangeRequired,
}
#[derive(optml::Optml, Clone, Copy, Debug)]
pub(crate) struct AdminCleanupRepositoryReport {
    access_sessions: super::AdminCleanupRows,
    audit_log: super::AdminCleanupRows,
    login_attempts: super::AdminCleanupRows,
    rate_limits: super::AdminCleanupRows,
    refresh_tokens: super::AdminCleanupRows,
}
impl AdminCleanupRepositoryReport {
    pub(crate) const fn into_parts(
        self,
    ) -> (
        super::AdminCleanupRows,
        super::AdminCleanupRows,
        super::AdminCleanupRows,
        super::AdminCleanupRows,
        super::AdminCleanupRows,
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
        server_admin_contract::AdminDisplayName,
        server_admin_contract::AdminLogin,
        super::AdminPasswordChangeRequired,
        super::AdminPermissions,
        super::AdminRoleNames,
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
impl From<super::SqlxAdminError> for AdminRepositoryError {
    fn from(error: super::SqlxAdminError) -> Self {
        Self::Sqlx(error)
    }
}
pub(super) fn invalid_stored_value<Error>(_error: Error) -> AdminRepositoryError {
    AdminRepositoryError::InvalidStoredValue
}

#[derive(optml::Optml, Debug, newtype::FromInner)]
pub(crate) struct SqlxAdminRepositoryConnectionMutRef<'connection_lt>(
    &'connection_lt mut sqlx::PgConnection,
);

#[derive(optml::Optml, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct SqlxAdminRepositoryPoolRef<'pool_lt>(&'pool_lt sqlx::PgPool);
#[derive(optml::Optml, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct AdminRecentLoginFailureCount(i64);
#[derive(optml::Optml, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct AdminPageTotalCount(i64);
impl AdminPageTotalCount {
    pub(crate) const fn get(self) -> i64 {
        self.0
    }
}
pub(crate) fn page_total(
    value: AdminPageTotalCount,
) -> Result<server_admin_contract::AdminPageTotal, AdminRepositoryError> {
    u64::try_from(value.get())
        .map(server_admin_contract::AdminPageTotal::from)
        .map_err(invalid_stored_value)
}
impl AdminRecentLoginFailureCount {
    pub(crate) fn reached(
        self,
        threshold: super::auth::StdAdminFailureThreshold,
    ) -> super::StdAdminBool {
        super::StdAdminBool::from(self.0 >= i64::from(threshold))
    }
}
#[derive(optml::Optml, Debug)]
pub(crate) struct AdminSignInUser {
    id: super::AdminUserId,
    password_hash: super::AdminPasswordHash,
    is_banned: super::StdAdminBool,
}
impl AdminSignInUser {
    pub(crate) fn into_parts(
        self,
    ) -> (
        super::AdminUserId,
        super::AdminPasswordHash,
        super::StdAdminBool,
    ) {
        (self.id, self.password_hash, self.is_banned)
    }
}
