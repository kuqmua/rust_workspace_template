#![allow(clippy::arbitrary_source_item_ordering)] // domain declarations are grouped by authentication and authorization responsibility
#[path = "application/auth.rs"]
pub mod auth;
mod generated_auth;
pub mod generated_tables;
mod password;
mod rbac;
mod token;
pub use generated_auth::{AdminGeneratedAuthLayer, AdminGeneratedAuthService};
pub use pg_table::domain_types::CombinationOfAppStateLogicTraits;
pub use server_admin_contract::domain_types::{
    AdminDisplayName, AdminLogin, AdminPermission, AdminPermissionTryFromStrError, AdminRoleName,
};
pub use server_admin_core::domain_types::{
    AdminAuditLogId, AdminIdTryFromI64Error, AdminNonZeroUsize, AdminPermissionId,
    AdminPermissionName, AdminRoleId, AdminSocketAddr, AdminUserId, SecrecyAdminString,
    StdAdminBool, StdAdminStrRef, StdAdminString, UuidAdminValue,
};
const ADMIN_AUTH_COLLECTION_MAX_LEN: usize = 10_000usize;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Serialize,
    newtype::DerefInner,
    newtype::FromInner,
)]
#[serde(transparent)]
pub(crate) struct AdminPasswordChangeRequired(bool);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
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
impl From<server_admin_core::domain_types::StdAdminStringTryFromStringError>
    for AdminSecretTextError
{
    fn from(value: server_admin_core::domain_types::StdAdminStringTryFromStringError) -> Self {
        match value {
            server_admin_core::domain_types::StdAdminStringTryFromStringError::InvalidBounds {
                ..
            } => Self::InvalidBounds,
            server_admin_core::domain_types::StdAdminStringTryFromStringError::TooShort {
                ..
            } => Self::TooShort,
            server_admin_core::domain_types::StdAdminStringTryFromStringError::TooLong {
                ..
            } => Self::TooLong,
            server_admin_core::domain_types::StdAdminStringTryFromStringError::ContainsNul => {
                Self::ContainsNul
            }
            server_admin_core::domain_types::StdAdminStringTryFromStringError::InvalidValue => {
                Self::InvalidValue
            }
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    newtype::AsRefTarget,
    newtype::IntoInnerFrom,
)]
#[serde(transparent)]
pub(crate) struct AdminPermissions(
    bounded_types::domain_types::vector::BoundedVec<
        AdminPermission,
        0,
        { ADMIN_AUTH_COLLECTION_MAX_LEN },
    >,
);
impl utoipa::PartialSchema for AdminPermissions {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::domain_types::vector::BoundedVec<
            AdminPermission,
            0,
            { ADMIN_AUTH_COLLECTION_MAX_LEN },
        > as utoipa::PartialSchema>::schema()
    }
}
impl utoipa::ToSchema for AdminPermissions {}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    newtype::AsRefTarget,
    newtype::IntoInnerFrom,
)]
#[serde(transparent)]
pub(crate) struct AdminRoleNames(
    bounded_types::domain_types::vector::BoundedVec<
        AdminRoleName,
        0,
        { ADMIN_AUTH_COLLECTION_MAX_LEN },
    >,
);
impl utoipa::PartialSchema for AdminRoleNames {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::domain_types::vector::BoundedVec<
            AdminRoleName,
            0,
            { ADMIN_AUTH_COLLECTION_MAX_LEN },
        > as utoipa::PartialSchema>::schema()
    }
}
impl utoipa::ToSchema for AdminRoleNames {}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("administrator authorization collection exceeds maximum length")]
pub(crate) struct AdminAuthCollectionError;
impl TryFrom<Vec<AdminPermission>> for AdminPermissions {
    type Error = AdminAuthCollectionError;
    fn try_from(value: Vec<AdminPermission>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from(value)
            .map(Self)
            .map_err(AdminAuthCollectionError::from)
    }
}
impl TryFrom<Vec<AdminRoleName>> for AdminRoleNames {
    type Error = AdminAuthCollectionError;
    fn try_from(value: Vec<AdminRoleName>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from(value)
            .map(Self)
            .map_err(AdminAuthCollectionError::from)
    }
}
impl From<bounded_types::domain_types::BoundedValueError> for AdminAuthCollectionError {
    fn from(_value: bounded_types::domain_types::BoundedValueError) -> Self {
        Self
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub struct AdminSharedSemaphoreArc(std::sync::Arc<tokio::sync::Semaphore>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct TokioAdminJoinError(tokio::task::JoinError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct TokioAdminAcquireError(tokio::sync::AcquireError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::DebugTransparent,
    newtype::FromInner,
)]
pub struct Argon2AdminPasswordHashError(argon2::password_hash::Error);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DebugTransparent,
    thiserror::Error,
    newtype::FromInner,
)]
#[error(transparent)]
pub struct SqlxAdminError(sqlx::Error);
impl From<AdminIdTryFromI64Error> for SqlxAdminError {
    fn from(value: AdminIdTryFromI64Error) -> Self {
        Self::from(sqlx::Error::Decode(Box::new(value)))
    }
}
impl From<server_admin_contract::domain_types::AdminIdTryFromI64Error> for SqlxAdminError {
    fn from(value: server_admin_contract::domain_types::AdminIdTryFromI64Error) -> Self {
        Self::from(sqlx::Error::Decode(Box::new(value)))
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DebugRedacted,
    newtype::FromInner,
    serde::Deserialize,
)]
#[serde(try_from = "String")]
pub struct AdminPassword(SecrecyAdminString);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum AdminPasswordTryFromStringError {
    #[error("{}", constants_str::ADMINISTRATOR_PASSWORD_LENGTH_IS_INVALID)]
    InvalidLength,
}
impl utoipa::PartialSchema for AdminPassword {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .min_length(Some(
                server_admin_contract::domain_types::ADMIN_PASSWORD_MIN_CHARS,
            ))
            .max_length(Some(
                server_admin_contract::domain_types::ADMIN_PASSWORD_MAX_CHARS,
            ))
            .write_only(Some(true))
            .build()
            .into()
    }
}
impl utoipa::ToSchema for AdminPassword {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::ADMINPASSWORD)
    }
}
impl TryFrom<String> for AdminPassword {
    type Error = AdminPasswordTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let len = value.chars().count();
        if !(server_admin_contract::domain_types::ADMIN_PASSWORD_MIN_CHARS
            ..=server_admin_contract::domain_types::ADMIN_PASSWORD_MAX_CHARS)
            .contains(&len)
        {
            return Err(AdminPasswordTryFromStringError::InvalidLength);
        }
        SecrecyAdminString::try_from(value)
            .map(Self::from)
            .map_err(|error| match error {
                server_admin_core::domain_types::StdAdminStringTryFromStringError::InvalidBounds { .. }
                | server_admin_core::domain_types::StdAdminStringTryFromStringError::TooShort { .. }
                | server_admin_core::domain_types::StdAdminStringTryFromStringError::TooLong { .. }
                | server_admin_core::domain_types::StdAdminStringTryFromStringError::ContainsNul
                | server_admin_core::domain_types::StdAdminStringTryFromStringError::InvalidValue => {
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugRedacted, newtype::FromInner,
)]
pub struct AdminPasswordHash(pg_types_text_misc::StringAsNonNullTextSecret);
impl AdminPasswordHash {
    #[must_use]
    pub(crate) fn expose(&self) -> StdAdminStrRef<'_> {
        StdAdminStrRef::from(self.0.as_ref())
    }

    #[must_use]
    pub fn new(value: pg_types_text_misc::StringAsNonNullTextSecret) -> Self {
        Self::from(value)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugRedacted, newtype::FromInner,
)]
pub struct AdminJwtSecret(SecrecyAdminString);
impl AdminJwtSecret {
    #[must_use]
    pub fn new(value: SecrecyAdminString) -> Self {
        Self::from(value)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugRedacted, newtype::FromInner,
)]
pub struct AdminOpaqueToken(SecrecyAdminString);
impl AdminOpaqueToken {
    #[must_use]
    pub fn new(value: SecrecyAdminString) -> Self {
        Self::from(value)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugRedacted, newtype::FromInner,
)]
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugRedacted, newtype::FromInner,
)]
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub struct AdminCookieSecure(bool);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub struct AdminCookieMaxAgeSeconds(u64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    newtype::BoundedString,
    newtype::AsRefOwned,
    newtype::IntoInner,
)]
#[bounded_string(max = 8192, description = "administrator cookie")]
pub struct StdAdminCookie(String);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct HttpAdminHeaderMapRef<'headers_lt>(&'headers_lt http::HeaderMap);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
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
            Self::Access => constants_str::SERVER_ADMIN_ACCESS_COOKIE_NAME,
            Self::Csrf => constants_str::ADMIN_CSRF_TOKEN,
            Self::Refresh => constants_str::ADMIN_REFRESH_TOKEN,
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
        constants_str::HTTPONLY
    } else {
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
    };
    let secure_attr = if secure.0 {
        constants_str::SECURE
    } else {
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
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
        StdAdminStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        AdminCookieMaxAgeSeconds::from(0),
        secure,
    )
}
#[must_use]
pub fn find_admin_cookie(
    headers: HttpAdminHeaderMapRef<'_>,
    kind: AdminCookieKind,
) -> Option<StdAdminStrRef<'_>> {
    match server_runtime_http::domain_types::resolve_unique_cookie(
        server_runtime_http::domain_types::HttpCookieHeadersRef::from(headers.0),
        server_runtime_http::domain_types::HttpCookieNameRef::from(kind.name().as_ref()),
    ) {
        server_runtime_http::domain_types::CookieResolution::Resolved(value) => {
            Some(StdAdminStrRef::from(<&str>::from(value)))
        }
        server_runtime_http::domain_types::CookieResolution::Invalid
        | server_runtime_http::domain_types::CookieResolution::Missing => None,
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub struct AdminPasswordHashConcurrency(AdminNonZeroUsize);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::FromInner,
)]
#[serde(from = "u64")]
pub struct AdminUnixTokenStream(u64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
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
impl AdminSessionId {
    pub(crate) const fn get(self) -> UuidAdminValue {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct AdminAccessClaims {
    aud: config_lib::domain_types::AdminTokenAudience,
    exp: AdminUnixTokenStream,
    iat: AdminUnixTokenStream,
    iss: config_lib::domain_types::AdminTokenIssuer,
    sub: AdminUserId,
    jti: AdminSessionId,
}
impl AdminAccessClaims {
    #[must_use]
    pub const fn new(
        user_id: AdminUserId,
        session_id: AdminSessionId,
        issued_at: AdminUnixTokenStream,
        expires_at: AdminUnixTokenStream,
        issuer: config_lib::domain_types::AdminTokenIssuer,
        audience: config_lib::domain_types::AdminTokenAudience,
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminPasswordHashError {
    #[error("administrator password hashing task failed: {0:?}")]
    Join(TokioAdminJoinError),
    #[error("administrator password hashing failed: {0:?}")]
    PasswordHash(Argon2AdminPasswordHashError),
    #[error("administrator password hashing concurrency limiter was closed: {0:?}")]
    SemaphoreClosed(TokioAdminAcquireError),
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct AdminPasswordHasher {
    semaphore: AdminSharedSemaphoreArc,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct JsonwebtokenAdminError(jsonwebtoken::errors::Error);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("administrator access token operation failed: {0:?}")]
#[derive(newtype::FromInner)]
pub struct AdminAccessTokenError(JsonwebtokenAdminError);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    PartialEq,
    Eq,
    newtype::BoundedString,
    newtype::AsRefOwned,
    newtype::IntoInner,
)]
#[bounded_string(max = 8192, description = "administrator access token")]
pub struct StdAdminAccessToken(String);
impl std::fmt::Debug for StdAdminAccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::REDACTED_ALT_3)
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
    issuer: &config_lib::domain_types::AdminTokenIssuer,
    audience: &config_lib::domain_types::AdminTokenAudience,
) -> Result<AdminAccessClaims, AdminAccessTokenError> {
    token::decode_access_token(token, secret, issuer, audience)
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
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
    optimal_memory_layout::OptimalMemoryLayout,
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct SqlxAdminMigrateError(sqlx::migrate::MigrateError);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
enum AdminMigrateErrorInner {
    #[error("migration failed: {0:?}")]
    Migration(SqlxAdminMigrateError),
    #[error("permission reconciliation failed: {0:?}")]
    Reconciliation(SqlxAdminError),
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
#[error("failed to prepare administrator schema: {0}")]
#[derive(newtype::FromInner)]
pub struct AdminMigrateError(AdminMigrateErrorInner);
impl From<SqlxAdminMigrateError> for AdminMigrateError {
    fn from(error: SqlxAdminMigrateError) -> Self {
        Self(AdminMigrateErrorInner::Migration(error))
    }
}
impl From<SqlxAdminError> for AdminMigrateError {
    fn from(error: SqlxAdminError) -> Self {
        Self(AdminMigrateErrorInner::Reconciliation(error))
    }
}
pub async fn prep_pg(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
) -> Result<(), AdminMigrateError> {
    crate::adapters::migrations::prep_pg(pool).await
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupBatchSize(i64);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupRetentionSeconds(i64);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupCfg {
    audit_retention: AdminCleanupRetentionSeconds,
    auth_retention: AdminCleanupRetentionSeconds,
    batch_size: AdminCleanupBatchSize,
    idempotency_completed_retention: AdminCleanupRetentionSeconds,
    idempotency_pending_retention: AdminCleanupRetentionSeconds,
    rate_limit_retention: AdminCleanupRetentionSeconds,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdminCleanupReport {
    access_sessions: AdminCleanupRows,
    audit_log: AdminCleanupRows,
    idempotency: AdminCleanupRows,
    login_attempts: AdminCleanupRows,
    rate_limits: AdminCleanupRows,
    refresh_tokens: AdminCleanupRows,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::Display,
    newtype::FromInner,
)]
pub struct AdminCleanupRows(u64);
impl std::ops::Add for AdminCleanupRows {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0.saturating_add(rhs.0))
    }
}
impl AdminCleanupRows {
    pub(crate) const fn get(self) -> u64 {
        self.0
    }

    fn saturating_add(self, rhs: Self) -> Self {
        Self::from(self.0.saturating_add(rhs.0))
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum AdminCleanupCfgError {
    #[error("{}", constants_str::CLEANUP_BATCH_SIZE_MUST_BE_BETWEEN_1_AND_10000)]
    BatchSizeOutOfRange,
    #[error("{}", constants_str::CLEANUP_RETENTION_MUST_BE_GREATER_THAN_ZERO)]
    RetentionMustBePositive,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminCleanupError {
    #[error("{}", constants_str::ADMIN_CLEANUP_ROWS_EXCEED_I64)]
    Count,
    #[error("idempotency cleanup failed: {0}")]
    Idempotency(#[source] pg_table::domain_types::SqlxPgTableIdempotencyError),
    #[error(transparent)]
    IdempotencyConfig(
        #[from] pg_table::domain_types::PgTableIdempotencyCleanupValueTryFromI64Error,
    ),
    #[error("administrator table cleanup failed: {0:?}")]
    Pg(#[source] SqlxAdminError),
}
impl TryFrom<i64> for AdminCleanupBatchSize {
    type Error = AdminCleanupCfgError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if (constants_i64::ONE..=10_000i64).contains(&value) {
            Ok(Self(value))
        } else {
            Err(AdminCleanupCfgError::BatchSizeOutOfRange)
        }
    }
}
impl TryFrom<i64> for AdminCleanupRetentionSeconds {
    type Error = AdminCleanupCfgError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value > constants_i64::ZERO {
            Ok(Self(value))
        } else {
            Err(AdminCleanupCfgError::RetentionMustBePositive)
        }
    }
}
impl AdminCleanupCfg {
    pub(crate) const fn audit_retention(self) -> AdminCleanupRetentionSeconds {
        self.audit_retention
    }

    pub(crate) const fn auth_retention(self) -> AdminCleanupRetentionSeconds {
        self.auth_retention
    }

    pub(crate) const fn batch_size(self) -> AdminCleanupBatchSize {
        self.batch_size
    }

    pub(crate) const fn idempotency_completed_retention(self) -> AdminCleanupRetentionSeconds {
        self.idempotency_completed_retention
    }

    pub(crate) const fn idempotency_pending_retention(self) -> AdminCleanupRetentionSeconds {
        self.idempotency_pending_retention
    }

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

    pub(crate) const fn rate_limit_retention(self) -> AdminCleanupRetentionSeconds {
        self.rate_limit_retention
    }
}
impl AdminCleanupBatchSize {
    pub(crate) const fn get(self) -> i64 {
        self.0
    }
}
impl AdminCleanupRetentionSeconds {
    pub(crate) const fn get(self) -> i64 {
        self.0
    }
}
impl AdminCleanupReport {
    #[allow(
        clippy::single_call_fn,
        reason = "cleanup adapter constructs the complete typed report through one invariant boundary"
    )]
    pub(crate) const fn new(
        access_sessions: AdminCleanupRows,
        audit_log: AdminCleanupRows,
        idempotency: AdminCleanupRows,
        login_attempts: AdminCleanupRows,
        rate_limits: AdminCleanupRows,
        refresh_tokens: AdminCleanupRows,
    ) -> Self {
        Self {
            access_sessions,
            audit_log,
            idempotency,
            login_attempts,
            rate_limits,
            refresh_tokens,
        }
    }

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
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    cfg: AdminCleanupCfg,
) -> Result<AdminCleanupReport, AdminCleanupError> {
    crate::adapters::cleanup::cleanup_admin_tables(pool, cfg).await
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
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
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    login: AdminLogin,
    display_name: AdminDisplayName,
    password: server_admin_contract::domain_types::AdminNewPassword,
    password_hasher: &AdminPasswordHasher,
) -> Result<AdminUserId, AdminBootstrapError> {
    crate::adapters::migrations::bootstrap_admin(
        pool,
        login,
        display_name,
        password,
        password_hasher,
    )
    .await
}
pub async fn reset_admin_password(
    pool: app_state::domain_types::SqlxPgPoolRef<'_>,
    login: AdminLogin,
    password: server_admin_contract::domain_types::AdminNewPassword,
    password_hasher: &AdminPasswordHasher,
) -> Result<AdminUserId, AdminPasswordResetError> {
    crate::adapters::migrations::reset_admin_password(pool, login, password, password_hasher).await
}
#[cfg(test)]
#[allow(
    clippy::needless_for_each,
    clippy::single_call_fn,
    reason = "repository policy forbids for loops and compact fixtures keep secret setup deterministic"
)]
mod tests;
