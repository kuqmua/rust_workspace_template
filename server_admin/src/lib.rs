#![allow(clippy::arbitrary_source_item_ordering)] // domain declarations are grouped by authentication and authorization responsibility
#![cfg_attr(test, allow(unused_crate_dependencies))] // tower is used by the separate admin_api integration test target
pub mod auth;
mod cleanup;
mod generated_auth;
pub mod generated_tables;
mod migrations;
mod password;
mod rbac;
mod repository;
mod token;
pub use generated_auth::{AdminGeneratedAuthLayer, AdminGeneratedAuthService};
pub use pg_table::CombinationOfAppStateLogicTraits;
pub use server_admin_contract::{
    AdminDisplayName, AdminLogin, AdminPermission, AdminPermissionTryFromStrError, AdminRoleName,
};
pub use server_admin_core::{
    AdminAuditLogId, AdminIdTryFromI64Error, AdminPermissionId, AdminPermissionName, AdminRoleId,
    AdminUserId, SecrecyAdminString, StdAdminBool, StdAdminNonZeroUsize, StdAdminSocketAddr,
    StdAdminStrRef, StdAdminString, UuidAdminValue,
};
const ADMIN_AUTH_COLLECTION_MAX_LEN: usize = 10_000usize;
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, newtype::DerefInner, newtype::FromInner,
)]
#[serde(transparent)]
pub(crate) struct AdminPasswordChangeRequired(bool);
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum AdminSecretTextError {
    #[error("administrator secret text has invalid bounds")]
    InvalidBounds,
    #[error("administrator secret text is too short")]
    TooShort,
    #[error("administrator secret text is too long")]
    TooLong,
    #[error("administrator secret text contains a NUL character")]
    ContainsNul,
    #[error("administrator secret text has an invalid value")]
    InvalidValue,
}
impl From<server_admin_core::StdAdminStringTryFromStringError> for AdminSecretTextError {
    fn from(value: server_admin_core::StdAdminStringTryFromStringError) -> Self {
        match value {
            server_admin_core::StdAdminStringTryFromStringError::InvalidBounds { .. } => {
                Self::InvalidBounds
            }
            server_admin_core::StdAdminStringTryFromStringError::TooShort { .. } => Self::TooShort,
            server_admin_core::StdAdminStringTryFromStringError::TooLong { .. } => Self::TooLong,
            server_admin_core::StdAdminStringTryFromStringError::ContainsNul => Self::ContainsNul,
            server_admin_core::StdAdminStringTryFromStringError::InvalidValue => Self::InvalidValue,
        }
    }
}
#[derive(
    Clone, Debug, serde::Serialize, utoipa::ToSchema, newtype::AsRefTarget, newtype::IntoInnerFrom,
)]
#[serde(transparent)]
pub(crate) struct AdminPermissions(
    bounded_types::BoundedVec<AdminPermission, 0, { ADMIN_AUTH_COLLECTION_MAX_LEN }>,
);
#[derive(
    Clone, Debug, serde::Serialize, utoipa::ToSchema, newtype::AsRefTarget, newtype::IntoInnerFrom,
)]
#[serde(transparent)]
pub(crate) struct AdminRoleNames(
    bounded_types::BoundedVec<AdminRoleName, 0, { ADMIN_AUTH_COLLECTION_MAX_LEN }>,
);
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("administrator authorization collection exceeds maximum length")]
pub(crate) struct AdminAuthCollectionError;
impl TryFrom<Vec<AdminPermission>> for AdminPermissions {
    type Error = AdminAuthCollectionError;
    fn try_from(value: Vec<AdminPermission>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::try_from(value)
            .map(Self)
            .map_err(AdminAuthCollectionError::from)
    }
}
impl TryFrom<Vec<AdminRoleName>> for AdminRoleNames {
    type Error = AdminAuthCollectionError;
    fn try_from(value: Vec<AdminRoleName>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::try_from(value)
            .map(Self)
            .map_err(AdminAuthCollectionError::from)
    }
}
impl From<bounded_types::BoundedValueError> for AdminAuthCollectionError {
    fn from(_value: bounded_types::BoundedValueError) -> Self {
        Self
    }
}
#[derive(Clone, Debug, newtype::FromInner)]
pub struct StdAdminSharedSemaphore(std::sync::Arc<tokio::sync::Semaphore>);
#[derive(newtype::DebugTransparent, newtype::FromInner)]
pub struct TokioAdminJoinError(tokio::task::JoinError);
#[derive(newtype::DebugTransparent, newtype::FromInner)]
pub struct TokioAdminAcquireError(tokio::sync::AcquireError);
#[derive(Clone, Copy, newtype::DebugTransparent, newtype::FromInner)]
pub struct Argon2AdminPasswordHashError(argon2::password_hash::Error);
#[derive(newtype::DebugTransparent, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
pub struct SqlxAdminError(sqlx::Error);
impl From<AdminIdTryFromI64Error> for SqlxAdminError {
    fn from(value: AdminIdTryFromI64Error) -> Self {
        Self::from(sqlx::Error::Decode(Box::new(value)))
    }
}
impl From<server_admin_contract::AdminIdTryFromI64Error> for SqlxAdminError {
    fn from(value: server_admin_contract::AdminIdTryFromI64Error) -> Self {
        Self::from(sqlx::Error::Decode(Box::new(value)))
    }
}
#[derive(newtype::DebugRedacted, newtype::FromInner, serde::Deserialize)]
#[serde(try_from = "String")]
pub struct AdminPassword(SecrecyAdminString);
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum AdminPasswordTryFromStringError {
    #[error("{}", str_constants::ADMINISTRATOR_PASSWORD_LENGTH_IS_INVALID)]
    InvalidLength,
}
impl utoipa::PartialSchema for AdminPassword {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .min_length(Some(server_admin_contract::ADMIN_PASSWORD_MIN_CHARS))
            .max_length(Some(server_admin_contract::ADMIN_PASSWORD_MAX_CHARS))
            .write_only(Some(true))
            .build()
            .into()
    }
}
impl utoipa::ToSchema for AdminPassword {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(str_constants::ADMINPASSWORD)
    }
}
impl TryFrom<String> for AdminPassword {
    type Error = AdminPasswordTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let len = value.chars().count();
        if !(server_admin_contract::ADMIN_PASSWORD_MIN_CHARS
            ..=server_admin_contract::ADMIN_PASSWORD_MAX_CHARS)
            .contains(&len)
        {
            return Err(AdminPasswordTryFromStringError::InvalidLength);
        }
        SecrecyAdminString::try_from(value)
            .map(Self::from)
            .map_err(|error| match error {
                server_admin_core::StdAdminStringTryFromStringError::InvalidBounds { .. }
                | server_admin_core::StdAdminStringTryFromStringError::TooShort { .. }
                | server_admin_core::StdAdminStringTryFromStringError::TooLong { .. }
                | server_admin_core::StdAdminStringTryFromStringError::ContainsNul
                | server_admin_core::StdAdminStringTryFromStringError::InvalidValue => {
                    AdminPasswordTryFromStringError::InvalidLength
                }
            })
    }
}
impl AdminPassword {
    #[must_use]
    pub fn new(value: SecrecyAdminString) -> Self {
        Self::from(value)
    }
    fn into_inner(self) -> SecrecyAdminString {
        self.0
    }
}
#[derive(newtype::DebugRedacted, newtype::FromInner)]
pub struct AdminPasswordHash(pg_types_text_misc::StringAsNonNullTextSecret);
impl AdminPasswordHash {
    #[must_use]
    pub fn new(value: pg_types_text_misc::StringAsNonNullTextSecret) -> Self {
        Self::from(value)
    }
}
#[derive(newtype::DebugRedacted, newtype::FromInner)]
pub struct AdminJwtSecret(SecrecyAdminString);
impl AdminJwtSecret {
    #[must_use]
    pub fn new(value: SecrecyAdminString) -> Self {
        Self::from(value)
    }
}
#[derive(newtype::DebugRedacted, newtype::FromInner)]
pub struct AdminOpaqueToken(SecrecyAdminString);
impl AdminOpaqueToken {
    #[must_use]
    pub fn new(value: SecrecyAdminString) -> Self {
        Self::from(value)
    }
}
#[derive(newtype::DebugRedacted, newtype::FromInner)]
pub struct AdminRefreshToken(AdminOpaqueToken);
impl AdminRefreshToken {
    #[must_use]
    pub fn new(value: AdminOpaqueToken) -> Self {
        Self::from(value)
    }
    #[must_use]
    pub fn expose(&self) -> StdAdminStrRef<'_> {
        StdAdminStrRef::from(secrecy::ExposeSecret::expose_secret(self.0.0.as_ref()).as_str())
    }
}
#[derive(newtype::DebugRedacted, newtype::FromInner)]
pub struct AdminTokenHash(SecrecyAdminString);
impl AdminTokenHash {
    #[must_use]
    #[allow(
        clippy::single_call_fn,
        reason = "the crate-private constructor is the invariant boundary for SHA-256 token hashes"
    )]
    pub(crate) fn new(value: SecrecyAdminString) -> Self {
        Self::from(value)
    }
    #[must_use]
    pub fn expose(&self) -> StdAdminStrRef<'_> {
        StdAdminStrRef::from(secrecy::ExposeSecret::expose_secret(self.0.as_ref()).as_str())
    }
}
#[derive(Debug)]
pub struct AdminGeneratedToken {
    hash: AdminTokenHash,
    token: AdminOpaqueToken,
}
impl AdminGeneratedToken {
    pub fn generate() -> Result<Self, AdminSecretTextError> {
        token::generate_token()
    }
    #[must_use]
    pub const fn hash(&self) -> &AdminTokenHash {
        &self.hash
    }
    #[must_use]
    pub const fn token(&self) -> &AdminOpaqueToken {
        &self.token
    }
}
pub fn hash_opaque_token(token: &AdminOpaqueToken) -> Result<AdminTokenHash, AdminSecretTextError> {
    token::hash_opaque_token(token)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
pub struct AdminCookieSecure(bool);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
pub struct AdminCookieMaxAgeSeconds(u64);
#[derive(
    Debug, Clone, PartialEq, Eq, newtype::BoundedString, newtype::AsRefOwned, newtype::IntoInner,
)]
#[bounded_string(max = 8192, description = "administrator cookie")]
pub struct StdAdminCookie(String);
#[derive(Debug, Clone, Copy, newtype::AsRefInner, newtype::FromInner)]
pub struct HttpAdminHeaderMapRef<'headers_lt>(&'headers_lt http::HeaderMap);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminCookieKind {
    Access,
    Csrf,
    Refresh,
}
impl AdminCookieKind {
    fn is_http_only(self) -> StdAdminBool {
        StdAdminBool::from(!matches!(self, Self::Csrf))
    }
    fn name(self) -> StdAdminStrRef<'static> {
        StdAdminStrRef::from(match self {
            Self::Access => str_constants::SERVER_ADMIN_ACCESS_COOKIE_NAME,
            Self::Csrf => str_constants::ADMIN_CSRF_TOKEN,
            Self::Refresh => str_constants::ADMIN_REFRESH_TOKEN,
        })
    }
}
#[must_use]
pub fn build_admin_cookie(
    kind: AdminCookieKind,
    value: StdAdminStrRef<'_>,
    max_age: AdminCookieMaxAgeSeconds,
    secure: AdminCookieSecure,
) -> StdAdminCookie {
    let http_only = if kind.is_http_only().get() {
        str_constants::HTTPONLY
    } else {
        str_constants::PG_CRUD_EMPTY_SQL_SUFFIX
    };
    let secure_attr = if secure.0 {
        str_constants::SECURE
    } else {
        str_constants::PG_CRUD_EMPTY_SQL_SUFFIX
    };
    StdAdminCookie::try_from(format!(
        "{}={}; Path=/; Max-Age={}; SameSite=Strict{http_only}{secure_attr}",
        kind.name().as_ref(),
        value.as_ref(),
        max_age.0
    ))
    .unwrap_or_else(StdAdminCookie::from)
}
#[must_use]
pub fn clear_admin_cookie(kind: AdminCookieKind, secure: AdminCookieSecure) -> StdAdminCookie {
    build_admin_cookie(
        kind,
        StdAdminStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
        AdminCookieMaxAgeSeconds::from(0),
        secure,
    )
}
#[must_use]
pub fn find_admin_cookie(
    headers: HttpAdminHeaderMapRef<'_>,
    kind: AdminCookieKind,
) -> Option<StdAdminStrRef<'_>> {
    match server_runtime_http::resolve_unique_cookie(
        server_runtime_http::HttpCookieHeadersRef::from(headers.0),
        server_runtime_http::HttpCookieNameRef::from(kind.name().as_ref()),
    ) {
        server_runtime_http::CookieResolution::Resolved(value) => {
            Some(StdAdminStrRef::from(<&str>::from(value)))
        }
        server_runtime_http::CookieResolution::Invalid
        | server_runtime_http::CookieResolution::Missing => None,
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
pub struct AdminPasswordHashConcurrency(StdAdminNonZeroUsize);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, newtype::FromInner,
)]
#[serde(from = "u64")]
pub struct AdminUnixTokenStream(u64);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::FromInner,
)]
#[serde(from = "UuidAdminValue")]
pub struct AdminSessionId(UuidAdminValue);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AdminAccessClaims {
    aud: config_lib::AdminTokenAudience,
    exp: AdminUnixTokenStream,
    iat: AdminUnixTokenStream,
    iss: config_lib::AdminTokenIssuer,
    jti: AdminSessionId,
    sub: AdminUserId,
}
impl AdminAccessClaims {
    #[must_use]
    pub const fn new(
        user_id: AdminUserId,
        session_id: AdminSessionId,
        issued_at: AdminUnixTokenStream,
        expires_at: AdminUnixTokenStream,
        issuer: config_lib::AdminTokenIssuer,
        audience: config_lib::AdminTokenAudience,
    ) -> Self {
        Self {
            aud: audience,
            exp: expires_at,
            iat: issued_at,
            iss: issuer,
            jti: session_id,
            sub: user_id,
        }
    }
    #[must_use]
    pub const fn user_id(&self) -> AdminUserId {
        self.sub
    }
    #[must_use]
    pub const fn session_id(&self) -> AdminSessionId {
        self.jti
    }
}
#[derive(Debug, thiserror::Error)]
pub enum AdminPasswordHashError {
    #[error("administrator password hashing task failed: {0:?}")]
    Join(TokioAdminJoinError),
    #[error("administrator password hashing failed: {0:?}")]
    PasswordHash(Argon2AdminPasswordHashError),
    #[error("administrator password hashing concurrency limiter was closed: {0:?}")]
    SemaphoreClosed(TokioAdminAcquireError),
}
#[derive(Clone, Debug)]
pub struct AdminPasswordHasher {
    semaphore: StdAdminSharedSemaphore,
}
#[derive(newtype::DebugTransparent, newtype::FromInner)]
pub struct JsonwebtokenAdminError(jsonwebtoken::errors::Error);
#[derive(Debug, thiserror::Error)]
#[error("administrator access token operation failed: {0:?}")]
#[derive(newtype::FromInner)]
pub struct AdminAccessTokenError(JsonwebtokenAdminError);
#[derive(Clone, PartialEq, Eq, newtype::BoundedString, newtype::AsRefOwned, newtype::IntoInner)]
#[bounded_string(max = 8192, description = "administrator access token")]
pub struct StdAdminAccessToken(String);
impl std::fmt::Debug for StdAdminAccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::REDACTED_ALT_3)
    }
}
pub fn encode_access_token(
    claims: &AdminAccessClaims,
    secret: &AdminJwtSecret,
) -> Result<StdAdminAccessToken, AdminAccessTokenError> {
    token::encode_access_token(claims, secret)
}
pub fn decode_access_token(
    token: &StdAdminAccessToken,
    secret: &AdminJwtSecret,
    issuer: &config_lib::AdminTokenIssuer,
    audience: &config_lib::AdminTokenAudience,
) -> Result<AdminAccessClaims, AdminAccessTokenError> {
    token::decode_access_token(token, secret, issuer, audience)
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::EnumFromStr,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminAuditAction {
    Create,
    Delete,
    Refresh,
    SignIn,
    SignOut,
    Update,
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::EnumFromStr,
)]
#[serde(rename_all = "snake_case")]
pub enum AdminAuditResource {
    AuditLog,
    Permission,
    Role,
    Session,
    SystemSettings,
    User,
}
#[derive(newtype::DebugTransparent, newtype::FromInner)]
pub struct SqlxAdminMigrateError(sqlx::migrate::MigrateError);
#[derive(Debug, thiserror::Error)]
enum AdminMigrateErrorInner {
    #[error("migration failed: {0:?}")]
    Migration(SqlxAdminMigrateError),
    #[error("permission reconciliation failed: {0:?}")]
    Reconciliation(SqlxAdminError),
}
#[derive(Debug, thiserror::Error)]
#[error("failed to prepare administrator schema: {0}")]
#[derive(newtype::FromInner)]
pub struct AdminMigrateError(AdminMigrateErrorInner);
pub async fn prep_pg(pool: app_state::SqlxPgPoolRef<'_>) -> Result<(), AdminMigrateError> {
    migrations::prep_pg(pool).await
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupBatchSize(i64);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupRetentionSeconds(i64);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupCfg {
    audit_retention: AdminCleanupRetentionSeconds,
    auth_retention: AdminCleanupRetentionSeconds,
    batch_size: AdminCleanupBatchSize,
    idempotency_completed_retention: AdminCleanupRetentionSeconds,
    idempotency_pending_retention: AdminCleanupRetentionSeconds,
    rate_limit_retention: AdminCleanupRetentionSeconds,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupReport {
    access_sessions: AdminCleanupRows,
    audit_log: AdminCleanupRows,
    idempotency: AdminCleanupRows,
    login_attempts: AdminCleanupRows,
    rate_limits: AdminCleanupRows,
    refresh_tokens: AdminCleanupRows,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, newtype::Display, newtype::FromInner)]
pub struct AdminCleanupRows(u64);
impl std::ops::Add for AdminCleanupRows {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0.saturating_add(rhs.0))
    }
}
impl AdminCleanupRows {
    fn saturating_add(self, rhs: Self) -> Self {
        Self::from(self.0.saturating_add(rhs.0))
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum AdminCleanupCfgError {
    #[error("{}", str_constants::CLEANUP_BATCH_SIZE_MUST_BE_BETWEEN_1_AND_10000)]
    BatchSizeOutOfRange,
    #[error("{}", str_constants::CLEANUP_RETENTION_MUST_BE_GREATER_THAN_ZERO)]
    RetentionMustBePositive,
}
#[derive(Debug, thiserror::Error)]
pub enum AdminCleanupError {
    #[error("{}", str_constants::ADMIN_CLEANUP_ROWS_EXCEED_I64)]
    Count,
    #[error("idempotency cleanup failed: {0}")]
    Idempotency(#[source] pg_table::SqlxPgTableIdempotencyError),
    #[error(transparent)]
    IdempotencyConfig(#[from] pg_table::PgTableIdempotencyCleanupValueTryFromI64Error),
    #[error("administrator table cleanup failed: {0:?}")]
    Pg(#[source] SqlxAdminError),
}
impl TryFrom<i64> for AdminCleanupBatchSize {
    type Error = AdminCleanupCfgError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if (1i64..=10_000i64).contains(&value) {
            Ok(Self(value))
        } else {
            Err(AdminCleanupCfgError::BatchSizeOutOfRange)
        }
    }
}
impl TryFrom<i64> for AdminCleanupRetentionSeconds {
    type Error = AdminCleanupCfgError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value > 0i64 {
            Ok(Self(value))
        } else {
            Err(AdminCleanupCfgError::RetentionMustBePositive)
        }
    }
}
impl AdminCleanupCfg {
    #[must_use]
    pub const fn new(
        batch_size: AdminCleanupBatchSize,
        auth_retention: AdminCleanupRetentionSeconds,
        audit_retention: AdminCleanupRetentionSeconds,
        rate_limit_retention: AdminCleanupRetentionSeconds,
        idempotency_completed_retention: AdminCleanupRetentionSeconds,
        idempotency_pending_retention: AdminCleanupRetentionSeconds,
    ) -> Self {
        Self {
            audit_retention,
            auth_retention,
            batch_size,
            idempotency_completed_retention,
            idempotency_pending_retention,
            rate_limit_retention,
        }
    }
}
impl AdminCleanupReport {
    #[must_use]
    pub fn total_rows(self) -> AdminCleanupRows {
        self.access_sessions
            .saturating_add(self.audit_log)
            .saturating_add(self.idempotency)
            .saturating_add(self.login_attempts)
            .saturating_add(self.rate_limits)
            .saturating_add(self.refresh_tokens)
    }
}
pub async fn cleanup_admin_tables(
    pool: app_state::SqlxPgPoolRef<'_>,
    cfg: AdminCleanupCfg,
) -> Result<AdminCleanupReport, AdminCleanupError> {
    cleanup::cleanup_admin_tables(pool, cfg).await
}
#[derive(Debug, thiserror::Error)]
pub enum AdminBootstrapError {
    #[error("administrator bootstrap audit details are invalid")]
    AuditDetails,
    #[error("administrator bootstrap display name is empty")]
    EmptyDisplayName,
    #[error("administrator bootstrap login has an invalid format")]
    InvalidLogin,
    #[error("administrator bootstrap password does not satisfy policy")]
    InvalidPassword,
    #[error("administrator bootstrap has already been completed")]
    AlreadyInitialized,
    #[error("administrator bootstrap password hashing failed: {0}")]
    PasswordHash(AdminPasswordHashError),
    #[error("administrator bootstrap database operation failed: {0:?}")]
    Pg(SqlxAdminError),
}
#[derive(Debug, thiserror::Error)]
pub enum AdminPasswordResetError {
    #[error("administrator password reset audit details are invalid")]
    AuditDetails,
    #[error("administrator password reset login has an invalid format")]
    InvalidLogin,
    #[error("administrator password reset password does not satisfy policy")]
    InvalidPassword,
    #[error("administrator password reset password hashing failed: {0}")]
    PasswordHash(AdminPasswordHashError),
    #[error("administrator password reset database operation failed: {0:?}")]
    Pg(SqlxAdminError),
    #[error("administrator password reset target does not exist")]
    UnknownLogin,
}
pub async fn bootstrap_admin(
    pool: app_state::SqlxPgPoolRef<'_>,
    login: AdminLogin,
    display_name: AdminDisplayName,
    password: server_admin_contract::AdminNewPassword,
    password_hasher: &AdminPasswordHasher,
) -> Result<AdminUserId, AdminBootstrapError> {
    migrations::bootstrap_admin(pool, login, display_name, password, password_hasher).await
}
pub async fn reset_admin_password(
    pool: app_state::SqlxPgPoolRef<'_>,
    login: AdminLogin,
    password: server_admin_contract::AdminNewPassword,
    password_hasher: &AdminPasswordHasher,
) -> Result<AdminUserId, AdminPasswordResetError> {
    migrations::reset_admin_password(pool, login, password, password_hasher).await
}
#[cfg(test)]
#[allow(clippy::needless_for_each, clippy::single_call_fn)] // repository policy forbids for loops and compact fixtures keep secret setup deterministic
mod tests {
    #[test]
    fn cleanup_configuration_enforces_positive_bounded_values() {
        assert_eq!(
            super::AdminCleanupBatchSize::try_from(0i64),
            Err(super::AdminCleanupCfgError::BatchSizeOutOfRange)
        );
        assert_eq!(
            super::AdminCleanupBatchSize::try_from(10_001i64),
            Err(super::AdminCleanupCfgError::BatchSizeOutOfRange)
        );
        assert_eq!(
            super::AdminCleanupRetentionSeconds::try_from(0i64),
            Err(super::AdminCleanupCfgError::RetentionMustBePositive)
        );
        assert_eq!(
            super::AdminCleanupBatchSize::try_from(1_000i64),
            Ok(super::AdminCleanupBatchSize(1_000i64))
        );
        assert_eq!(
            super::AdminCleanupRetentionSeconds::try_from(3_600i64),
            Ok(super::AdminCleanupRetentionSeconds(3_600i64))
        );
    }
    fn secret(value: &str) -> super::SecrecyAdminString {
        super::SecrecyAdminString::try_from(value.to_owned()).expect("c2116874")
    }
    fn password_hasher() -> super::AdminPasswordHasher {
        super::AdminPasswordHasher::new(super::AdminPasswordHashConcurrency::from(
            super::StdAdminNonZeroUsize::from(std::num::NonZeroUsize::new(1).expect("70761471")),
        ))
    }
    fn password(value: &str) -> super::AdminPassword {
        super::AdminPassword::new(secret(value))
    }
    fn jwt_secret() -> super::AdminJwtSecret {
        super::AdminJwtSecret::new(secret(
            str_constants::TEST_ONLY_SECRET_WITH_SUFFICIENT_ENTROPY,
        ))
    }
    #[test]
    fn permission_round_trip_is_exhaustive() {
        super::AdminPermission::ALL
            .into_iter()
            .for_each(|permission| {
                assert_eq!(
                    super::AdminPermission::try_from(permission.as_str().as_ref())
                        .expect("0f53b75c"),
                    permission
                );
            });
    }
    #[test]
    fn permission_serializes_as_public_contract_value() {
        assert_eq!(
            serde_json::to_string(&super::AdminPermission::UsersRead).expect("9a6b413e"),
            "\"users:read\""
        );
    }
    #[test]
    fn unknown_permission_is_rejected() {
        assert_eq!(
            super::AdminPermission::try_from(str_constants::UNKNOWN_READ).err(),
            Some(super::AdminPermissionTryFromStrError)
        );
    }
    #[test]
    fn migration_inventory_is_not_empty() {
        let migrations = super::migrations::migrator().iter().collect::<Vec<_>>();
        assert_eq!(migrations.len(), 12usize);
        assert!(
            migrations
                .iter()
                .any(|migration| migration.description == "admin schema")
        );
    }
    #[test]
    fn permission_seed_contains_the_complete_typed_catalog() {
        assert!(super::AdminPermission::ALL.into_iter().all(|permission| {
            super::migrations::migrator().iter().any(|migration| {
                migration
                    .sql
                    .as_str()
                    .contains(permission.as_str().as_ref())
            })
        }));
    }
    #[tokio::test]
    async fn password_hash_verifies_only_matching_password() {
        let hasher = password_hasher();
        let hash = hasher
            .hash(password(str_constants::CORRECT_PASSWORD_ALT))
            .await
            .expect("174a5d2f");
        assert!(
            hasher
                .verify(password("correct password"), hash)
                .await
                .expect("604f40be")
                .get()
        );
        let other_hash = hasher
            .hash(password(str_constants::CORRECT_PASSWORD_ALT))
            .await
            .expect("38819b94");
        assert!(
            !hasher
                .verify(password("wrong password"), other_hash)
                .await
                .expect("ed6b499a")
                .get()
        );
    }
    #[test]
    fn secrets_are_redacted_in_debug_output() {
        let raw_secret = str_constants::NEVER_PRINT_THIS_VALUE;
        let password = password(raw_secret);
        let jwt_secret = super::AdminJwtSecret::new(secret(raw_secret));
        let access_token =
            super::StdAdminAccessToken::try_from(raw_secret.to_owned()).expect("e295277c");
        assert!(!format!("{password:?}").contains(raw_secret));
        assert!(!format!("{jwt_secret:?}").contains(raw_secret));
        assert!(!format!("{access_token:?}").contains(raw_secret));
    }
    #[test]
    fn generated_token_hash_is_stable_and_does_not_expose_token() {
        let token = super::AdminOpaqueToken::new(secret(str_constants::FIXED_TEST_TOKEN));
        let hash = super::hash_opaque_token(&token).expect("3af32394");
        assert_eq!(
            hash.expose().as_ref(),
            "abae2c734c2b0249ef1d413fdf30c332c6875fde570f9bbeef4295966f0b4943"
        );
        assert!(!format!("{hash:?}").contains("fixed-test-token"));
    }
    #[test]
    fn cookie_policy_marks_only_secret_tokens_http_only() {
        let access = super::build_admin_cookie(
            super::AdminCookieKind::Access,
            super::StdAdminStrRef::from(str_constants::ACCESS),
            super::AdminCookieMaxAgeSeconds::from(60),
            super::AdminCookieSecure::from(true),
        );
        let csrf = super::build_admin_cookie(
            super::AdminCookieKind::Csrf,
            super::StdAdminStrRef::from(str_constants::CSRF),
            super::AdminCookieMaxAgeSeconds::from(60),
            super::AdminCookieSecure::from(true),
        );
        assert!(access.as_ref().contains("HttpOnly"));
        assert!(access.as_ref().contains("Secure"));
        assert!(access.as_ref().contains("SameSite=Strict"));
        assert!(!csrf.as_ref().contains("HttpOnly"));
        assert!(csrf.as_ref().contains("Secure"));
    }
    #[test]
    fn cookie_parser_matches_complete_cookie_name() {
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::header::COOKIE,
            http::HeaderValue::from_static(
                str_constants::OTHER_1_ADMIN_ACCESS_TOKEN_EXPECTED_ADMIN_ACCESS_TOKEN_SUFFIX_WRONG,
            ),
        );
        assert_eq!(
            super::find_admin_cookie(
                super::HttpAdminHeaderMapRef::from(&headers),
                super::AdminCookieKind::Access,
            ),
            Some(super::StdAdminStrRef::from("expected"))
        );
    }
    #[test]
    fn bootstrap_login_format_accepts_only_database_compatible_values() {
        let valid =
            super::AdminLogin::try_from(str_constants::ADMIN_USER_1.to_owned()).expect("078c759d");
        assert_eq!(valid.as_ref(), str_constants::ADMIN_USER_1);
        let _uppercase_error = super::AdminLogin::try_from(str_constants::ADMIN.to_owned())
            .expect_err(str_constants::VALUE_5FA1C6E2);
        let _short_error = super::AdminLogin::try_from(str_constants::AB.to_owned())
            .expect_err(str_constants::VALUE_B78D42A9);
    }
    #[test]
    fn access_token_round_trip_checks_issuer_and_audience() {
        let claims = super::AdminAccessClaims::new(
            super::AdminUserId::try_from(7).expect("d6d3da8a"),
            super::AdminSessionId::from(super::UuidAdminValue::from(
                uuid::Uuid::parse_str(str_constants::B871BD8F_7810_4D4B_94A1_5458D3016907)
                    .expect("05562da0"),
            )),
            super::AdminUnixTokenStream::from(1),
            super::AdminUnixTokenStream::from(4_102_444_800),
            config_lib::AdminTokenIssuer::try_from(str_constants::TEST_ISSUER.to_owned())
                .expect("fd6a65b0"),
            config_lib::AdminTokenAudience::try_from(str_constants::TEST_AUDIENCE.to_owned())
                .expect("6e423e16"),
        );
        let secret = jwt_secret();
        let token = super::encode_access_token(&claims, &secret).expect("b41052bc");
        let issuer = config_lib::AdminTokenIssuer::try_from(str_constants::TEST_ISSUER.to_owned())
            .expect("5edc807f");
        let audience =
            config_lib::AdminTokenAudience::try_from(str_constants::TEST_AUDIENCE.to_owned())
                .expect("0c3975a1");
        let decoded =
            super::decode_access_token(&token, &secret, &issuer, &audience).expect("0ed905ff");
        assert_eq!(
            decoded.user_id(),
            super::AdminUserId::try_from(7).expect("5b88f22a")
        );
        assert_eq!(decoded.session_id(), claims.session_id());
        drop(
            super::decode_access_token(
                &token,
                &secret,
                &issuer,
                &config_lib::AdminTokenAudience::try_from(str_constants::WRONG_AUDIENCE.to_owned())
                    .expect("92f9c5ec"),
            )
            .expect_err(str_constants::A82438CC),
        );
    }
}
