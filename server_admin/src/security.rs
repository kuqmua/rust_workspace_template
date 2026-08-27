#![allow(clippy::wildcard_imports)] // split security implementation modules share a private facade vocabulary
#[path = "security/admin_auth_collection_max_len.rs"]
mod admin_auth_collection_max_len;
use admin_auth_collection_max_len::ADMIN_AUTH_COLLECTION_MAX_LEN;
#[path = "security/admin_password_change_required.rs"]
mod admin_password_change_required;
pub(crate) use admin_password_change_required::*;
#[path = "security/admin_secret_text_error.rs"]
mod admin_secret_text_error;
pub use admin_secret_text_error::*;
#[path = "security/admin_permissions.rs"]
mod admin_permissions;
pub(crate) use admin_permissions::*;
#[path = "security/admin_role_names.rs"]
mod admin_role_names;
pub(crate) use admin_role_names::*;
#[path = "security/admin_auth_collection_error.rs"]
mod admin_auth_collection_error;
pub(crate) use admin_auth_collection_error::*;
#[path = "security/admin_shared_semaphore_arc.rs"]
mod admin_shared_semaphore_arc;
pub use admin_shared_semaphore_arc::*;
#[path = "security/tokio_admin_join_error.rs"]
mod tokio_admin_join_error;
pub use tokio_admin_join_error::*;
#[path = "security/tokio_admin_acquire_error.rs"]
mod tokio_admin_acquire_error;
pub use tokio_admin_acquire_error::*;
#[path = "security/tokio_admin_owned_semaphore_permit.rs"]
mod tokio_admin_owned_semaphore_permit;
pub(crate) use tokio_admin_owned_semaphore_permit::*;
#[path = "security/argon2_admin_password_hash_error.rs"]
mod argon2_admin_password_hash_error;
pub use argon2_admin_password_hash_error::*;
#[path = "security/sqlx_admin_error.rs"]
mod sqlx_admin_error;
pub use sqlx_admin_error::*;
#[path = "security/admin_password.rs"]
mod admin_password;
pub use admin_password::*;
#[path = "security/admin_password_try_from_string_error.rs"]
mod admin_password_try_from_string_error;
pub use admin_password_try_from_string_error::*;
#[path = "security/admin_password_hash.rs"]
mod admin_password_hash;
pub use admin_password_hash::*;
#[path = "security/admin_jwt_secret.rs"]
mod admin_jwt_secret;
pub use admin_jwt_secret::*;
#[path = "security/admin_opaque_token.rs"]
mod admin_opaque_token;
pub use admin_opaque_token::*;
#[path = "security/admin_refresh_token.rs"]
mod admin_refresh_token;
pub use admin_refresh_token::*;
#[path = "security/admin_token_hash.rs"]
mod admin_token_hash;
pub use admin_token_hash::*;
#[path = "security/admin_generated_token.rs"]
mod admin_generated_token;
pub use admin_generated_token::*;
#[path = "security/token.rs"]
mod token;
pub use token::*;
#[path = "security/admin_cookie_secure.rs"]
mod admin_cookie_secure;
pub use admin_cookie_secure::*;
#[path = "security/admin_cookie_max_age_seconds.rs"]
mod admin_cookie_max_age_seconds;
pub use admin_cookie_max_age_seconds::*;
#[path = "security/std_admin_cookie.rs"]
mod std_admin_cookie;
pub use std_admin_cookie::*;
#[path = "security/http_admin_header_map_ref.rs"]
mod http_admin_header_map_ref;
pub use http_admin_header_map_ref::*;
#[path = "security/admin_cookie_kind.rs"]
mod admin_cookie_kind;
pub use admin_cookie_kind::*;
#[path = "security/build_admin_cookie.rs"]
mod build_admin_cookie;
pub use build_admin_cookie::*;
#[path = "security/clear_admin_cookie.rs"]
mod clear_admin_cookie;
pub use clear_admin_cookie::*;
#[path = "security/find_admin_cookie.rs"]
mod find_admin_cookie;
pub use find_admin_cookie::*;
#[path = "security/admin_password_hash_concurrency.rs"]
mod admin_password_hash_concurrency;
pub use admin_password_hash_concurrency::*;
#[path = "security/admin_unix_token_stream.rs"]
mod admin_unix_token_stream;
pub use admin_unix_token_stream::*;
#[path = "security/admin_session_id.rs"]
mod admin_session_id;
pub use admin_session_id::*;
#[path = "security/admin_access_claims.rs"]
mod admin_access_claims;
pub use admin_access_claims::*;
#[path = "security/admin_password_hash_error.rs"]
mod admin_password_hash_error;
pub use admin_password_hash_error::*;
#[path = "security/admin_password_hasher.rs"]
mod admin_password_hasher;
pub use admin_password_hasher::*;
#[path = "security/jsonwebtoken_admin_error.rs"]
mod jsonwebtoken_admin_error;
pub use jsonwebtoken_admin_error::*;
#[path = "security/admin_access_token_error.rs"]
mod admin_access_token_error;
pub use admin_access_token_error::*;
#[path = "security/std_admin_access_token.rs"]
mod std_admin_access_token;
pub use std_admin_access_token::*;
#[path = "security/encode_access_token.rs"]
mod encode_access_token;
pub use encode_access_token::*;
#[path = "security/decode_access_token.rs"]
mod decode_access_token;
pub use decode_access_token::*;

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
impl AdminJwtSecret {
    #[must_use]
    pub fn new(value: super::SecrecyAdminString) -> Self {
        Self::from(value)
    }
}
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
impl<'headers_lt> HttpAdminHeaderMapRef<'headers_lt> {
    pub(crate) const fn get(self) -> &'headers_lt http::HeaderMap {
        self.0
    }
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
impl AdminPasswordHashConcurrency {
    pub(crate) const fn get(self) -> super::AdminNonZeroUsize {
        self.0
    }
}
impl AdminUnixTokenStream {
    pub(crate) const fn get(self) -> u64 {
        self.0
    }
}
impl AdminSessionId {
    pub(crate) const fn get(self) -> super::UuidAdminValue {
        self.0
    }
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
