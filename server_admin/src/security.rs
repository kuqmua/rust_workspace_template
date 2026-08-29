pub use super::admin_access_claims::*;
pub use super::admin_access_token_error::*;
pub(crate) use super::admin_auth_collection_error::*;
pub(crate) use super::admin_auth_permissions::*;
pub use super::admin_cookie_kind::*;
pub use super::admin_cookie_max_age_seconds::*;
pub use super::admin_cookie_secure::*;
pub use super::admin_generated_token::*;
pub use super::admin_jwt_secret::*;
pub use super::admin_opaque_token::*;
pub use super::admin_password::*;
pub(crate) use super::admin_password_change_required::*;
pub use super::admin_password_hash::*;
pub use super::admin_password_hash_concurrency::*;
pub use super::admin_password_hash_error::*;
pub use super::admin_password_hasher::*;
pub use super::admin_password_try_from_string_error::*;
pub use super::admin_refresh_token::*;
pub(crate) use super::admin_role_names::*;
pub use super::admin_secret_text_error::*;
pub use super::admin_session_id::*;
pub use super::admin_shared_semaphore_arc::*;
pub use super::admin_token_hash::*;
pub use super::admin_unix_token_stream::*;
pub use super::argon2_admin_password_hash_error::*;
pub use super::build_admin_cookie::*;
pub use super::clear_admin_cookie::*;
pub use super::decode_access_token::*;
pub use super::encode_access_token::*;
pub use super::find_admin_cookie::*;
pub use super::http_admin_header_map_ref::*;
pub use super::jsonwebtoken_admin_error::*;
pub use super::sqlx_admin_error::*;
pub use super::std_admin_access_token::*;
pub use super::std_admin_cookie::*;
pub use super::token::*;
pub use super::tokio_admin_acquire_error::*;
pub use super::tokio_admin_join_error::*;
pub(crate) use super::tokio_admin_owned_semaphore_permit::*;
use admin_auth_collection_max_len::ADMIN_AUTH_COLLECTION_MAX_LEN;
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
impl utoipa::PartialSchema for AdminAuthPermissions {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::BoundedVec<
            crate::AdminPermission,
            0,
            { ADMIN_AUTH_COLLECTION_MAX_LEN },
        > as utoipa::PartialSchema>::schema()
    }
}
impl utoipa::ToSchema for AdminAuthPermissions {}
impl utoipa::PartialSchema for AdminRoleNames {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::BoundedVec<
            crate::AdminRoleName,
            0,
            { ADMIN_AUTH_COLLECTION_MAX_LEN },
        > as utoipa::PartialSchema>::schema()
    }
}
impl utoipa::ToSchema for AdminRoleNames {}
impl TryFrom<Vec<crate::AdminPermission>> for AdminAuthPermissions {
    type Error = AdminAuthCollectionError;
    fn try_from(value: Vec<crate::AdminPermission>) -> Result<Self, Self::Error> {
        bounded_types::BoundedVec::try_from(value)
            .map(Self)
            .map_err(AdminAuthCollectionError::from)
    }
}
impl TryFrom<Vec<crate::AdminRoleName>> for AdminRoleNames {
    type Error = AdminAuthCollectionError;
    fn try_from(value: Vec<crate::AdminRoleName>) -> Result<Self, Self::Error> {
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
impl SqlxAdminError {
    pub(crate) fn into_inner(self) -> sqlx::Error {
        self.0
    }
}
impl From<crate::AdminIdTryFromI64Error> for SqlxAdminError {
    fn from(value: crate::AdminIdTryFromI64Error) -> Self {
        Self::from(sqlx::Error::Decode(Box::new(value)))
    }
}
impl From<server_admin_contract::domain_types::AdminIdTryFromI64Error> for SqlxAdminError {
    fn from(value: server_admin_contract::domain_types::AdminIdTryFromI64Error) -> Self {
        Self::from(sqlx::Error::Decode(Box::new(value)))
    }
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
        crate::SecrecyAdminString::try_from(value)
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
    pub fn new(value: crate::SecrecyAdminString) -> Self {
        Self::from(value)
    }
    pub(crate) fn into_inner(self) -> crate::SecrecyAdminString {
        self.0
    }
}
impl AdminPasswordHash {
    #[must_use]
    pub(crate) fn expose(&self) -> crate::StdAdminStrRef<'_> {
        crate::StdAdminStrRef::from(self.0.as_ref())
    }

    #[must_use]
    pub fn new(value: pg_types_text_misc::StringAsNonNullTextSecret) -> Self {
        Self::from(value)
    }
}
impl AdminJwtSecret {
    #[must_use]
    pub fn new(value: crate::SecrecyAdminString) -> Self {
        Self::from(value)
    }
}
impl AdminOpaqueToken {
    #[must_use]
    pub fn new(value: crate::SecrecyAdminString) -> Self {
        Self::from(value)
    }
    #[must_use]
    pub(crate) fn expose(&self) -> crate::StdAdminStrRef<'_> {
        crate::StdAdminStrRef::from(secrecy::ExposeSecret::expose_secret(self.0.as_ref()).as_str())
    }
    pub(crate) fn clone_secret(&self) -> crate::SecrecyAdminString {
        crate::SecrecyAdminString::from(secrecy::SecretBox::new(Box::new(
            secrecy::ExposeSecret::expose_secret(&self.0).clone(),
        )))
    }
}
impl AdminRefreshToken {
    #[must_use]
    pub fn new(value: AdminOpaqueToken) -> Self {
        Self::from(value)
    }
    #[must_use]
    pub fn expose(&self) -> crate::StdAdminStrRef<'_> {
        crate::StdAdminStrRef::from(
            secrecy::ExposeSecret::expose_secret(self.0.0.as_ref()).as_str(),
        )
    }
}
impl AdminTokenHash {
    #[must_use]
    pub(crate) fn new(value: crate::SecrecyAdminString) -> Self {
        Self::from(value)
    }
    #[must_use]
    pub fn expose(&self) -> crate::StdAdminStrRef<'_> {
        crate::StdAdminStrRef::from(secrecy::ExposeSecret::expose_secret(self.0.as_ref()).as_str())
    }
}
impl AdminGeneratedToken {
    pub fn generate() -> Result<Self, AdminSecretTextError> {
        let token = crate::SecrecyAdminString::try_from(format!(
            "{}.{}",
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4()
        ))
        .map(AdminOpaqueToken::new)?;
        crate::hash_opaque_token::hash_opaque_token(&token).map(|hash| Self { hash, token })
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
impl<'headers_lt> HttpAdminHeaderMapRef<'headers_lt> {
    pub(crate) const fn get(self) -> &'headers_lt http::HeaderMap {
        self.0
    }
}
impl AdminCookieKind {
    pub(crate) fn name(self) -> crate::StdAdminStrRef<'static> {
        crate::StdAdminStrRef::from(match self {
            Self::Access => constants_str::SERVER_ADMIN_ACCESS_COOKIE_NAME,
            Self::Csrf => constants_str::ADMIN_CSRF_TOKEN,
            Self::Refresh => constants_str::ADMIN_REFRESH_TOKEN,
        })
    }
}
impl AdminPasswordHashConcurrency {
    pub(crate) const fn get(self) -> std::num::NonZeroUsize {
        self.0
    }
}
impl AdminUnixTokenStream {
    pub(crate) const fn get(self) -> u64 {
        self.0
    }
}
impl AdminSessionId {
    pub(crate) const fn get(self) -> crate::UuidAdminValue {
        self.0
    }
}
impl AdminAccessClaims {
    #[must_use]
    pub const fn new(
        user_id: crate::AdminUserId,
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
    pub const fn user_id(&self) -> crate::AdminUserId {
        self.sub
    }
    #[must_use]
    pub const fn session_id(&self) -> AdminSessionId {
        self.jti
    }
}
#[allow(
    clippy::multiple_inherent_impl,
    reason = "semaphore acquisition stays with the security-owned wrapper while hashing behavior stays in the password module"
)]
impl AdminPasswordHasher {
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

#[cfg(test)]
mod tests {
    #[test]
    fn opaque_token_debug_is_redacted() {
        let token = crate::AdminOpaqueToken::new(
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

// Root-owned module compatibility wrappers.
mod admin_auth_collection_max_len {
    pub use super::super::admin_auth_collection_max_len::*;
}
mod admin_password_change_required {
    pub use super::super::admin_password_change_required::*;
}
mod admin_secret_text_error {
    pub use super::super::admin_secret_text_error::*;
}
mod admin_auth_permissions {
    pub use super::super::admin_auth_permissions::*;
}
mod admin_role_names {
    pub use super::super::admin_role_names::*;
}
mod admin_auth_collection_error {
    pub use super::super::admin_auth_collection_error::*;
}
mod admin_shared_semaphore_arc {
    pub use super::super::admin_shared_semaphore_arc::*;
}
mod tokio_admin_join_error {
    pub use super::super::tokio_admin_join_error::*;
}
mod tokio_admin_acquire_error {
    pub use super::super::tokio_admin_acquire_error::*;
}
mod tokio_admin_owned_semaphore_permit {
    pub use super::super::tokio_admin_owned_semaphore_permit::*;
}
mod argon2_admin_password_hash_error {
    pub use super::super::argon2_admin_password_hash_error::*;
}
mod sqlx_admin_error {
    pub use super::super::sqlx_admin_error::*;
}
mod admin_password {
    pub use super::super::admin_password::*;
}
mod admin_password_try_from_string_error {
    pub use super::super::admin_password_try_from_string_error::*;
}
mod admin_password_hash {
    pub use super::super::admin_password_hash::*;
}
mod admin_jwt_secret {
    pub use super::super::admin_jwt_secret::*;
}
mod admin_opaque_token {
    pub use super::super::admin_opaque_token::*;
}
mod admin_refresh_token {
    pub use super::super::admin_refresh_token::*;
}
mod admin_token_hash {
    pub use super::super::admin_token_hash::*;
}
mod admin_generated_token {
    pub use super::super::admin_generated_token::*;
}
mod token {
    pub use super::super::token::*;
}
mod admin_cookie_secure {
    pub use super::super::admin_cookie_secure::*;
}
mod admin_cookie_max_age_seconds {
    pub use super::super::admin_cookie_max_age_seconds::*;
}
mod std_admin_cookie {
    pub use super::super::std_admin_cookie::*;
}
mod http_admin_header_map_ref {
    pub use super::super::http_admin_header_map_ref::*;
}
mod admin_cookie_kind {
    pub use super::super::admin_cookie_kind::*;
}
mod build_admin_cookie {
    pub use super::super::build_admin_cookie::*;
}
mod clear_admin_cookie {
    pub use super::super::clear_admin_cookie::*;
}
mod find_admin_cookie {
    pub use super::super::find_admin_cookie::*;
}
mod admin_password_hash_concurrency {
    pub use super::super::admin_password_hash_concurrency::*;
}
mod admin_unix_token_stream {
    pub use super::super::admin_unix_token_stream::*;
}
mod admin_session_id {
    pub use super::super::admin_session_id::*;
}
mod admin_access_claims {
    pub use super::super::admin_access_claims::*;
}
mod admin_password_hash_error {
    pub use super::super::admin_password_hash_error::*;
}
mod admin_password_hasher {
    pub use super::super::admin_password_hasher::*;
}
mod jsonwebtoken_admin_error {
    pub use super::super::jsonwebtoken_admin_error::*;
}
mod admin_access_token_error {
    pub use super::super::admin_access_token_error::*;
}
mod std_admin_access_token {
    pub use super::super::std_admin_access_token::*;
}
mod encode_access_token {
    pub use super::super::encode_access_token::*;
}
mod decode_access_token {
    pub use super::super::decode_access_token::*;
}
