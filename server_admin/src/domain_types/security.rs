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
        super::AdminPermission,
        0,
        { ADMIN_AUTH_COLLECTION_MAX_LEN },
    >,
);
impl utoipa::PartialSchema for AdminPermissions {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::domain_types::vector::BoundedVec<
            super::AdminPermission,
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
        super::AdminRoleName,
        0,
        { ADMIN_AUTH_COLLECTION_MAX_LEN },
    >,
);
impl utoipa::PartialSchema for AdminRoleNames {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::domain_types::vector::BoundedVec<
            super::AdminRoleName,
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
impl TryFrom<Vec<super::AdminPermission>> for AdminPermissions {
    type Error = AdminAuthCollectionError;
    fn try_from(value: Vec<super::AdminPermission>) -> Result<Self, Self::Error> {
        bounded_types::domain_types::vector::BoundedVec::try_from(value)
            .map(Self)
            .map_err(AdminAuthCollectionError::from)
    }
}
impl TryFrom<Vec<super::AdminRoleName>> for AdminRoleNames {
    type Error = AdminAuthCollectionError;
    fn try_from(value: Vec<super::AdminRoleName>) -> Result<Self, Self::Error> {
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
#[allow(
    dead_code,
    reason = "the owned permit is held for its drop semantics while password hashing runs"
)]
pub(crate) struct TokioAdminOwnedSemaphorePermit(tokio::sync::OwnedSemaphorePermit);
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
impl SqlxAdminError {
    pub(crate) fn into_inner(self) -> sqlx::Error {
        self.0
    }
}
impl From<super::AdminIdTryFromI64Error> for SqlxAdminError {
    fn from(value: super::AdminIdTryFromI64Error) -> Self {
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
pub struct AdminPassword(super::SecrecyAdminString);
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
        super::SecrecyAdminString::try_from(value)
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
    pub fn new(value: super::SecrecyAdminString) -> Self {
        Self::from(value)
    }
    pub(crate) fn into_inner(self) -> super::SecrecyAdminString {
        self.0
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugRedacted, newtype::FromInner,
)]
pub struct AdminPasswordHash(pg_types_text_misc::StringAsNonNullTextSecret);
impl AdminPasswordHash {
    #[must_use]
    pub(crate) fn expose(&self) -> super::StdAdminStrRef<'_> {
        super::StdAdminStrRef::from(self.0.as_ref())
    }

    #[must_use]
    pub fn new(value: pg_types_text_misc::StringAsNonNullTextSecret) -> Self {
        Self::from(value)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugRedacted, newtype::FromInner,
)]
pub struct AdminJwtSecret(super::SecrecyAdminString);
impl AdminJwtSecret {
    #[must_use]
    pub fn new(value: super::SecrecyAdminString) -> Self {
        Self::from(value)
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugRedacted, newtype::FromInner,
)]
pub struct AdminOpaqueToken(super::SecrecyAdminString);
impl AdminOpaqueToken {
    #[must_use]
    pub fn new(value: super::SecrecyAdminString) -> Self {
        Self::from(value)
    }
    #[must_use]
    pub(crate) fn expose(&self) -> super::StdAdminStrRef<'_> {
        super::StdAdminStrRef::from(secrecy::ExposeSecret::expose_secret(self.0.as_ref()).as_str())
    }
    pub(crate) fn clone_secret(&self) -> super::SecrecyAdminString {
        super::SecrecyAdminString::from(secrecy::SecretBox::new(Box::new(
            secrecy::ExposeSecret::expose_secret(&self.0).clone(),
        )))
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
    pub fn expose(&self) -> super::StdAdminStrRef<'_> {
        super::StdAdminStrRef::from(
            secrecy::ExposeSecret::expose_secret(self.0.0.as_ref()).as_str(),
        )
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugRedacted, newtype::FromInner,
)]
pub struct AdminTokenHash(super::SecrecyAdminString);
impl AdminTokenHash {
    #[must_use]
    #[allow(
        clippy::single_call_fn,
        reason = "the crate-private constructor is the invariant boundary for SHA-256 token hashes"
    )]
    pub(crate) fn new(value: super::SecrecyAdminString) -> Self {
        Self::from(value)
    }
    #[must_use]
    pub fn expose(&self) -> super::StdAdminStrRef<'_> {
        super::StdAdminStrRef::from(secrecy::ExposeSecret::expose_secret(self.0.as_ref()).as_str())
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct AdminGeneratedToken {
    hash: AdminTokenHash,
    token: AdminOpaqueToken,
}
impl AdminGeneratedToken {
    pub fn generate() -> Result<Self, AdminSecretTextError> {
        let token = super::SecrecyAdminString::try_from(format!(
            "{}.{}",
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4()
        ))
        .map(AdminOpaqueToken::new)?;
        super::hash_opaque_token::hash_opaque_token(&token).map(|hash| Self { hash, token })
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
pub fn token(token: &AdminOpaqueToken) -> Result<AdminTokenHash, AdminSecretTextError> {
    super::hash_opaque_token::hash_opaque_token(token)
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
impl<'headers_lt> HttpAdminHeaderMapRef<'headers_lt> {
    pub(crate) const fn get(self) -> &'headers_lt http::HeaderMap {
        self.0
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdminCookieKind {
    Access,
    Csrf,
    Refresh,
}
impl AdminCookieKind {
    fn name(self) -> super::StdAdminStrRef<'static> {
        super::StdAdminStrRef::from(match self {
            Self::Access => constants_str::SERVER_ADMIN_ACCESS_COOKIE_NAME,
            Self::Csrf => constants_str::ADMIN_CSRF_TOKEN,
            Self::Refresh => constants_str::ADMIN_REFRESH_TOKEN,
        })
    }
}
#[must_use]
pub fn build_admin_cookie(
    kind: AdminCookieKind,
    value: super::StdAdminStrRef<'_>,
    max_age: AdminCookieMaxAgeSeconds,
    secure: AdminCookieSecure,
) -> StdAdminCookie {
    let http_only = if matches!(kind, AdminCookieKind::Csrf) {
        constants_str::PG_CRUD_EMPTY_SQL_SUFFIX
    } else {
        constants_str::HTTPONLY
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
        super::StdAdminStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        AdminCookieMaxAgeSeconds::from(0),
        secure,
    )
}
#[must_use]
pub fn find_admin_cookie(
    headers: HttpAdminHeaderMapRef<'_>,
    kind: AdminCookieKind,
) -> Option<super::StdAdminStrRef<'_>> {
    match server_runtime_http::domain_types::resolve_unique_cookie(
        server_runtime_http::domain_types::HttpCookieHeadersRef::from(headers.0),
        server_runtime_http::domain_types::HttpCookieNameRef::from(kind.name().as_ref()),
    ) {
        server_runtime_http::domain_types::CookieResolution::Resolved(value) => {
            Some(super::StdAdminStrRef::from(<&str>::from(value)))
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
pub struct AdminPasswordHashConcurrency(super::AdminNonZeroUsize);
impl AdminPasswordHashConcurrency {
    pub(crate) const fn get(self) -> super::AdminNonZeroUsize {
        self.0
    }
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
    newtype::FromInner,
)]
#[serde(from = "u64")]
pub struct AdminUnixTokenStream(u64);
impl AdminUnixTokenStream {
    pub(crate) const fn get(self) -> u64 {
        self.0
    }
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
    newtype::FromInner,
)]
#[serde(from = "super::UuidAdminValue")]
pub struct AdminSessionId(super::UuidAdminValue);
impl AdminSessionId {
    pub(crate) const fn get(self) -> super::UuidAdminValue {
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
    sub: super::AdminUserId,
    jti: AdminSessionId,
}
impl AdminAccessClaims {
    #[must_use]
    pub const fn new(
        user_id: super::AdminUserId,
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
    pub const fn user_id(&self) -> super::AdminUserId {
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
#[allow(
    clippy::multiple_inherent_impl,
    reason = "semaphore acquisition stays with the security-owned wrapper while hashing behavior stays in the password module"
)]
impl AdminPasswordHasher {
    #[allow(
        clippy::single_call_fn,
        reason = "the constructor keeps the semaphore field private across the password module boundary"
    )]
    pub(crate) const fn from_semaphore(semaphore: AdminSharedSemaphoreArc) -> Self {
        Self { semaphore }
    }
    pub(crate) async fn acquire(
        &self,
    ) -> Result<TokioAdminOwnedSemaphorePermit, AdminPasswordHashError> {
        std::sync::Arc::<tokio::sync::Semaphore>::clone(&self.semaphore.0)
            .acquire_owned()
            .await
            .map(TokioAdminOwnedSemaphorePermit::from)
            .map_err(|error| {
                AdminPasswordHashError::SemaphoreClosed(TokioAdminAcquireError::from(error))
            })
    }
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
impl From<StdAdminAccessTokenTryFromStringError> for AdminSecretTextError {
    fn from(value: StdAdminAccessTokenTryFromStringError) -> Self {
        match value {
            StdAdminAccessTokenTryFromStringError::ContainsNul => Self::ContainsNul,
            StdAdminAccessTokenTryFromStringError::InvalidBounds { .. } => Self::InvalidBounds,
            StdAdminAccessTokenTryFromStringError::InvalidValue => Self::InvalidValue,
            StdAdminAccessTokenTryFromStringError::TooLong { .. } => Self::TooLong,
            StdAdminAccessTokenTryFromStringError::TooShort { .. } => Self::TooShort,
        }
    }
}
impl std::fmt::Debug for StdAdminAccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::REDACTED_ALT_3)
    }
}
pub fn encode_access_token(
    claims: &AdminAccessClaims,
    secret: &AdminJwtSecret,
) -> Result<StdAdminAccessToken, AdminAccessTokenError> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        claims,
        &jsonwebtoken::EncodingKey::from_secret(
            secrecy::ExposeSecret::expose_secret(secret.0.as_ref()).as_bytes(),
        ),
    )
    .map(StdAdminAccessToken)
    .map_err(JsonwebtokenAdminError::from)
    .map_err(AdminAccessTokenError::from)
}
pub fn decode_access_token(
    token: &StdAdminAccessToken,
    secret: &AdminJwtSecret,
    issuer: &config_lib::domain_types::AdminTokenIssuer,
    audience: &config_lib::domain_types::AdminTokenAudience,
) -> Result<AdminAccessClaims, AdminAccessTokenError> {
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[issuer.as_ref()]);
    validation.set_audience(&[audience.as_ref()]);
    jsonwebtoken::decode::<AdminAccessClaims>(
        token.as_ref(),
        &jsonwebtoken::DecodingKey::from_secret(
            secrecy::ExposeSecret::expose_secret(secret.0.as_ref()).as_bytes(),
        ),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(JsonwebtokenAdminError::from)
    .map_err(AdminAccessTokenError::from)
}

#[cfg(test)]
mod tests {
    #[test]
    fn opaque_token_debug_is_redacted() {
        let token = super::AdminOpaqueToken::new(
            crate::domain_types::SecrecyAdminString::try_from(
                constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES.to_owned(),
            )
            .expect("4f0db163 bounded test secret must be valid"),
        );
        let debug = format!("{token:?}");
        assert!(debug.contains(constants_str::REDACTED_ALT_3));
        assert!(!debug.contains(constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES));
    }
}
