#![allow(clippy::wildcard_imports)] // split security implementation modules share a private facade vocabulary
pub use admin_access_claims::*;
pub use admin_access_token_error::*;
pub(crate) use admin_auth_collection_error::*;
use admin_auth_collection_max_len::ADMIN_AUTH_COLLECTION_MAX_LEN;
pub(crate) use admin_auth_permissions::*;
pub use admin_cookie_kind::*;
pub use admin_cookie_max_age_seconds::*;
pub use admin_cookie_secure::*;
pub use admin_generated_token::*;
pub use admin_jwt_secret::*;
pub use admin_opaque_token::*;
pub use admin_password::*;
pub(crate) use admin_password_change_required::*;
pub use admin_password_hash::*;
pub use admin_password_hash_concurrency::*;
pub use admin_password_hash_error::*;
pub use admin_password_hasher::*;
pub use admin_password_try_from_string_error::*;
pub use admin_refresh_token::*;
pub(crate) use admin_role_names::*;
pub use admin_secret_text_error::*;
pub use admin_session_id::*;
pub use admin_shared_semaphore_arc::*;
pub use admin_token_hash::*;
pub use admin_unix_token_stream::*;
pub use argon2_admin_password_hash_error::*;
pub use build_admin_cookie::*;
pub use clear_admin_cookie::*;
pub use decode_access_token::*;
pub use encode_access_token::*;
pub use find_admin_cookie::*;
pub use http_admin_header_map_ref::*;
pub use jsonwebtoken_admin_error::*;
pub use sqlx_admin_error::*;
pub use std_admin_access_token::*;
pub use std_admin_cookie::*;
pub use token::*;
pub use tokio_admin_acquire_error::*;
pub use tokio_admin_join_error::*;
pub(crate) use tokio_admin_owned_semaphore_permit::*;

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
        <bounded_types::domain_types::vector::BoundedVec<
            crate::AdminPermission,
            0,
            { ADMIN_AUTH_COLLECTION_MAX_LEN },
        > as utoipa::PartialSchema>::schema()
    }
}
impl utoipa::ToSchema for AdminAuthPermissions {}
impl utoipa::PartialSchema for AdminRoleNames {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::domain_types::vector::BoundedVec<
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
        bounded_types::domain_types::vector::BoundedVec::try_from(value)
            .map(Self)
            .map_err(AdminAuthCollectionError::from)
    }
}
impl TryFrom<Vec<crate::AdminRoleName>> for AdminRoleNames {
    type Error = AdminAuthCollectionError;
    fn try_from(value: Vec<crate::AdminRoleName>) -> Result<Self, Self::Error> {
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
    pub(crate) const fn get(self) -> crate::AdminNonZeroUsize {
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
    pub use crate::admin_auth_collection_max_len::*;
}
mod admin_password_change_required {
    pub use crate::admin_password_change_required::*;
}
mod admin_secret_text_error {
    pub use crate::admin_secret_text_error::*;
}
mod admin_auth_permissions {
    pub use crate::admin_auth_permissions::*;
}
mod admin_role_names {
    pub use crate::admin_role_names::*;
}
mod admin_auth_collection_error {
    pub use crate::admin_auth_collection_error::*;
}
mod admin_shared_semaphore_arc {
    pub use crate::admin_shared_semaphore_arc::*;
}
mod tokio_admin_join_error {
    pub use crate::tokio_admin_join_error::*;
}
mod tokio_admin_acquire_error {
    pub use crate::tokio_admin_acquire_error::*;
}
mod tokio_admin_owned_semaphore_permit {
    pub use crate::tokio_admin_owned_semaphore_permit::*;
}
mod argon2_admin_password_hash_error {
    pub use crate::argon2_admin_password_hash_error::*;
}
mod sqlx_admin_error {
    pub use crate::sqlx_admin_error::*;
}
mod admin_password {
    pub use crate::admin_password::*;
}
mod admin_password_try_from_string_error {
    pub use crate::admin_password_try_from_string_error::*;
}
mod admin_password_hash {
    pub use crate::admin_password_hash::*;
}
mod admin_jwt_secret {
    pub use crate::admin_jwt_secret::*;
}
mod admin_opaque_token {
    pub use crate::admin_opaque_token::*;
}
mod admin_refresh_token {
    pub use crate::admin_refresh_token::*;
}
mod admin_token_hash {
    pub use crate::admin_token_hash::*;
}
mod admin_generated_token {
    pub use crate::admin_generated_token::*;
}
mod token {
    pub use crate::token::*;
}
mod admin_cookie_secure {
    pub use crate::admin_cookie_secure::*;
}
mod admin_cookie_max_age_seconds {
    pub use crate::admin_cookie_max_age_seconds::*;
}
mod std_admin_cookie {
    pub use crate::std_admin_cookie::*;
}
mod http_admin_header_map_ref {
    pub use crate::http_admin_header_map_ref::*;
}
mod admin_cookie_kind {
    pub use crate::admin_cookie_kind::*;
}
mod build_admin_cookie {
    pub use crate::build_admin_cookie::*;
}
mod clear_admin_cookie {
    pub use crate::clear_admin_cookie::*;
}
mod find_admin_cookie {
    pub use crate::find_admin_cookie::*;
}
mod admin_password_hash_concurrency {
    pub use crate::admin_password_hash_concurrency::*;
}
mod admin_unix_token_stream {
    pub use crate::admin_unix_token_stream::*;
}
mod admin_session_id {
    pub use crate::admin_session_id::*;
}
mod admin_access_claims {
    pub use crate::admin_access_claims::*;
}
mod admin_password_hash_error {
    pub use crate::admin_password_hash_error::*;
}
mod admin_password_hasher {
    pub use crate::admin_password_hasher::*;
}
mod jsonwebtoken_admin_error {
    pub use crate::jsonwebtoken_admin_error::*;
}
mod admin_access_token_error {
    pub use crate::admin_access_token_error::*;
}
mod std_admin_access_token {
    pub use crate::std_admin_access_token::*;
}
mod encode_access_token {
    pub use crate::encode_access_token::*;
}
mod decode_access_token {
    pub use crate::decode_access_token::*;
}
