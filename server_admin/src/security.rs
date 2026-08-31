impl From<server_admin_core::std_admin_string::StdAdminStringTryFromStringError>
    for crate::admin_secret_text_error::AdminSecretTextError
{
    fn from(value: server_admin_core::std_admin_string::StdAdminStringTryFromStringError) -> Self {
        match value {
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::InvalidBounds {
                ..
            } => Self::InvalidBounds,
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::TooShort {
                ..
            } => Self::TooShort,
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::TooLong {
                ..
            } => Self::TooLong,
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::ContainsNul => {
                Self::ContainsNul
            }
            server_admin_core::std_admin_string::StdAdminStringTryFromStringError::InvalidValue => {
                Self::InvalidValue
            }
        }
    }
}
impl utoipa::PartialSchema for crate::admin_auth_permissions::AdminAuthPermissions {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::bounded_vec::BoundedVec<
            server_admin_contract::admin_permission::AdminPermission,
            0,
            { crate::admin_auth_collection_max_len::ADMIN_AUTH_COLLECTION_MAX_LEN },
        > as utoipa::PartialSchema>::schema()
    }
}
impl utoipa::ToSchema for crate::admin_auth_permissions::AdminAuthPermissions {}
impl utoipa::PartialSchema for crate::runtime_admin_role_names::RuntimeAdminRoleNames {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        <bounded_types::bounded_vec::BoundedVec<
            server_admin_contract::admin_role_name::AdminRoleName,
            0,
            { crate::admin_auth_collection_max_len::ADMIN_AUTH_COLLECTION_MAX_LEN },
        > as utoipa::PartialSchema>::schema()
    }
}
impl utoipa::ToSchema for crate::runtime_admin_role_names::RuntimeAdminRoleNames {}
impl TryFrom<Vec<server_admin_contract::admin_permission::AdminPermission>>
    for crate::admin_auth_permissions::AdminAuthPermissions
{
    type Error = crate::admin_auth_collection_error::AdminAuthCollectionError;
    fn try_from(
        value: Vec<server_admin_contract::admin_permission::AdminPermission>,
    ) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(value)
            .map(Self)
            .map_err(crate::admin_auth_collection_error::AdminAuthCollectionError::from)
    }
}
impl TryFrom<Vec<server_admin_contract::admin_role_name::AdminRoleName>>
    for crate::runtime_admin_role_names::RuntimeAdminRoleNames
{
    type Error = crate::admin_auth_collection_error::AdminAuthCollectionError;
    fn try_from(
        value: Vec<server_admin_contract::admin_role_name::AdminRoleName>,
    ) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(value)
            .map(Self)
            .map_err(crate::admin_auth_collection_error::AdminAuthCollectionError::from)
    }
}
impl From<bounded_types::bounded_value_error::BoundedValueError>
    for crate::admin_auth_collection_error::AdminAuthCollectionError
{
    fn from(_value: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        Self::TooLarge
    }
}
impl crate::sqlx_admin_error::SqlxAdminError {
    pub(crate) fn into_inner(self) -> sqlx::Error {
        self.0
    }
}
impl From<server_admin_core::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error>
    for crate::sqlx_admin_error::SqlxAdminError
{
    fn from(
        value: server_admin_core::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error,
    ) -> Self {
        Self::from(sqlx::Error::Decode(Box::new(value)))
    }
}
impl From<server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error>
    for crate::sqlx_admin_error::SqlxAdminError
{
    fn from(
        value: server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error,
    ) -> Self {
        Self::from(sqlx::Error::Decode(Box::new(value)))
    }
}
impl utoipa::PartialSchema for crate::runtime_admin_password::RuntimeAdminPassword {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .min_length(Some(
                server_admin_contract::identity::ADMIN_PASSWORD_MIN_CHARS,
            ))
            .max_length(Some(
                server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS,
            ))
            .write_only(Some(true))
            .build()
            .into()
    }
}
impl utoipa::ToSchema for crate::runtime_admin_password::RuntimeAdminPassword {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::ADMINPASSWORD)
    }
}
impl TryFrom<String> for crate::runtime_admin_password::RuntimeAdminPassword {
    type Error = crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let len = value.chars().count();
        if !(server_admin_contract::identity::ADMIN_PASSWORD_MIN_CHARS
            ..=server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS)
            .contains(&len)
        {
            return Err(crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError::InvalidLength);
        }
        server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(value)
            .map(Self::from)
            .map_err(|error| match error {
                server_admin_core::std_admin_string::StdAdminStringTryFromStringError::InvalidBounds { .. }
                | server_admin_core::std_admin_string::StdAdminStringTryFromStringError::TooShort { .. }
                | server_admin_core::std_admin_string::StdAdminStringTryFromStringError::TooLong { .. }
                | server_admin_core::std_admin_string::StdAdminStringTryFromStringError::ContainsNul
                | server_admin_core::std_admin_string::StdAdminStringTryFromStringError::InvalidValue => {
                    crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError::InvalidLength
                }
            })
    }
}
impl crate::runtime_admin_password::RuntimeAdminPassword {
    #[must_use]
    pub fn new(value: server_admin_core::secrecy_admin_string::SecrecyAdminString) -> Self {
        Self::from(value)
    }
    pub(crate) fn into_inner(self) -> server_admin_core::secrecy_admin_string::SecrecyAdminString {
        self.0
    }
}
impl crate::admin_password_hash::AdminPasswordHash {
    #[must_use]
    pub(crate) fn expose(&self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'_> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(self.0.as_ref())
    }

    #[must_use]
    pub fn new(
        value: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextSecret,
    ) -> Self {
        Self::from(value)
    }
}
impl crate::runtime_admin_jwt_secret::RuntimeAdminJwtSecret {
    #[must_use]
    pub fn new(value: server_admin_core::secrecy_admin_string::SecrecyAdminString) -> Self {
        Self::from(value)
    }
}
impl crate::admin_opaque_token::AdminOpaqueToken {
    #[must_use]
    pub fn new(value: server_admin_core::secrecy_admin_string::SecrecyAdminString) -> Self {
        Self::from(value)
    }
    #[must_use]
    pub(crate) fn expose(&self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'_> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            secrecy::ExposeSecret::expose_secret(self.0.as_ref()).as_str(),
        )
    }
    pub(crate) fn clone_secret(
        &self,
    ) -> server_admin_core::secrecy_admin_string::SecrecyAdminString {
        server_admin_core::secrecy_admin_string::SecrecyAdminString::from(secrecy::SecretBox::new(
            Box::new(secrecy::ExposeSecret::expose_secret(&self.0).clone()),
        ))
    }
}
impl crate::admin_refresh_token::AdminRefreshToken {
    #[must_use]
    pub fn new(value: crate::admin_opaque_token::AdminOpaqueToken) -> Self {
        Self::from(value)
    }
    #[must_use]
    pub fn expose(&self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'_> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            secrecy::ExposeSecret::expose_secret(self.0.0.as_ref()).as_str(),
        )
    }
}
impl crate::admin_token_hash::AdminTokenHash {
    #[must_use]
    pub(crate) fn new(value: server_admin_core::secrecy_admin_string::SecrecyAdminString) -> Self {
        Self::from(value)
    }
    #[must_use]
    pub fn expose(&self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'_> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            secrecy::ExposeSecret::expose_secret(self.0.as_ref()).as_str(),
        )
    }
}
impl crate::admin_generated_token::AdminGeneratedToken {
    pub fn generate() -> Result<Self, crate::admin_secret_text_error::AdminSecretTextError> {
        let token = server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(format!(
            "{}.{}",
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4()
        ))
        .map(crate::admin_opaque_token::AdminOpaqueToken::new)?;
        crate::hash_opaque_token::hash_opaque_token(&token).map(|hash| Self { hash, token })
    }
    #[must_use]
    pub const fn hash(&self) -> &crate::admin_token_hash::AdminTokenHash {
        &self.hash
    }
    #[must_use]
    pub const fn token(&self) -> &crate::admin_opaque_token::AdminOpaqueToken {
        &self.token
    }
}
impl<'headers_lt> crate::http_admin_header_map_ref::HttpAdminHeaderMapRef<'headers_lt> {
    pub(crate) const fn get(self) -> &'headers_lt http::HeaderMap {
        self.0
    }
}
impl crate::admin_cookie_kind::AdminCookieKind {
    pub(crate) fn name(self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'static> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(match self {
            Self::Access => constants_str::SERVER_ADMIN_ACCESS_COOKIE_NAME,
            Self::Csrf => constants_str::ADMIN_CSRF_TOKEN,
            Self::Refresh => constants_str::ADMIN_REFRESH_TOKEN,
        })
    }
}
impl crate::runtime_admin_password_hash_concurrency::RuntimeAdminPasswordHashConcurrency {
    pub(crate) const fn get(self) -> std::num::NonZeroUsize {
        self.0
    }
}
impl crate::admin_unix_token_stream::AdminUnixTokenStream {
    pub(crate) const fn get(self) -> u64 {
        self.0
    }
}
impl crate::admin_session_id::AdminSessionId {
    pub(crate) const fn get(self) -> server_admin_core::uuid_admin_value::UuidAdminValue {
        self.0
    }
}
impl crate::admin_access_claims::AdminAccessClaims {
    #[must_use]
    pub const fn new(
        user_id: server_admin_core::admin_user_record_id::AdminUserRecordId,
        session_id: crate::admin_session_id::AdminSessionId,
        issued_at: crate::admin_unix_token_stream::AdminUnixTokenStream,
        expires_at: crate::admin_unix_token_stream::AdminUnixTokenStream,
        issuer: config_lib::admin_token_issuer::AdminTokenIssuer,
        audience: config_lib::admin_token_audience::AdminTokenAudience,
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
    pub const fn user_id(&self) -> server_admin_core::admin_user_record_id::AdminUserRecordId {
        self.sub
    }
    #[must_use]
    pub const fn session_id(&self) -> crate::admin_session_id::AdminSessionId {
        self.jti
    }
}
#[allow(
    clippy::multiple_inherent_impl,
    reason = "semaphore acquisition stays with the security-owned wrapper while hashing behavior stays in the password module"
)]
impl crate::admin_password_hasher::AdminPasswordHasher {
    pub(crate) async fn acquire(
        &self,
    ) -> Result<
        crate::tokio_admin_owned_semaphore_permit::TokioAdminOwnedSemaphorePermit,
        crate::admin_password_hash_error::AdminPasswordHashError,
    > {
        std::sync::Arc::<tokio::sync::Semaphore>::clone(&self.semaphore.0)
            .acquire_owned()
            .await
            .map(crate::tokio_admin_owned_semaphore_permit::TokioAdminOwnedSemaphorePermit::from)
            .map_err(|error| {
                crate::admin_password_hash_error::AdminPasswordHashError::SemaphoreClosed(
                    crate::tokio_admin_acquire_error::TokioAdminAcquireError::from(error),
                )
            })
    }
}
impl From<crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError>
    for crate::admin_secret_text_error::AdminSecretTextError
{
    fn from(value: crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError) -> Self {
        match value {
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::ContainsNul => Self::ContainsNul,
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::InvalidBounds { .. } => Self::InvalidBounds,
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::InvalidValue => Self::InvalidValue,
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::TooLong { .. } => Self::TooLong,
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::TooShort { .. } => Self::TooShort,
        }
    }
}
impl std::fmt::Debug for crate::std_admin_access_token::StdAdminAccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::REDACTED_ALT_3)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn opaque_token_debug_is_redacted() {
        let token = crate::admin_opaque_token::AdminOpaqueToken::new(
            server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(
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
mod admin_auth_collection_max_len {}
mod admin_password_change_required {}
mod admin_secret_text_error {}
mod admin_auth_permissions {}
mod admin_role_names {}
mod admin_auth_collection_error {}
mod admin_shared_semaphore_arc {}
mod tokio_admin_join_error {}
mod tokio_admin_acquire_error {}
mod tokio_admin_owned_semaphore_permit {}
mod argon2_admin_password_hash_error {}
mod sqlx_admin_error {}
mod admin_password {}
mod admin_password_try_from_string_error {}
mod admin_password_hash {}
mod admin_jwt_secret {}
mod admin_opaque_token {}
mod admin_refresh_token {}
mod admin_token_hash {}
mod admin_generated_token {}
mod token {}
mod admin_cookie_secure {}
mod admin_cookie_max_age_seconds {}
mod std_admin_cookie {}
mod http_admin_header_map_ref {}
mod admin_cookie_kind {}
mod build_admin_cookie {}
mod clear_admin_cookie {}
mod find_admin_cookie {}
mod admin_password_hash_concurrency {}
mod admin_unix_token_stream {}
mod admin_session_id {}
mod admin_access_claims {}
mod admin_password_hash_error {}
mod admin_password_hasher {}
mod jsonwebtoken_admin_error {}
mod admin_access_token_error {}
mod std_admin_access_token {}
mod encode_access_token {}
mod decode_access_token {}
