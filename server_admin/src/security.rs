impl From<server_admin_core::std_admin_string::StdAdminStringTryFromStringError>
    for crate::admin_secret_text_error::AdminSecretTextError
{
    fn from(
        std_admin_string_try_from_string_error: server_admin_core::std_admin_string::StdAdminStringTryFromStringError,
    ) -> Self {
        match std_admin_string_try_from_string_error {
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
impl TryFrom<Vec<server_admin_contract::admin_permission::AdminPermission>>
    for crate::admin_auth_permissions::AdminAuthPermissions
{
    type Error = crate::admin_auth_collection_error::AdminAuthCollectionError;
    fn try_from(
        vec: Vec<server_admin_contract::admin_permission::AdminPermission>,
    ) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(vec)
            .map(Self::from)
            .map_err(crate::admin_auth_collection_error::AdminAuthCollectionError::from)
    }
}
impl TryFrom<Vec<server_admin_contract::admin_role_name::AdminRoleName>>
    for crate::runtime_admin_role_names::RuntimeAdminRoleNames
{
    type Error = crate::admin_auth_collection_error::AdminAuthCollectionError;
    fn try_from(
        vec: Vec<server_admin_contract::admin_role_name::AdminRoleName>,
    ) -> Result<Self, Self::Error> {
        bounded_types::bounded_vec::BoundedVec::try_from(vec)
            .map(Self::from)
            .map_err(crate::admin_auth_collection_error::AdminAuthCollectionError::from)
    }
}
impl From<bounded_types::bounded_value_error::BoundedValueError>
    for crate::admin_auth_collection_error::AdminAuthCollectionError
{
    fn from(bounded_value_error: bounded_types::bounded_value_error::BoundedValueError) -> Self {
        let _: bounded_types::bounded_value_error::BoundedValueError = bounded_value_error;
        Self::TooLarge
    }
}
impl From<server_admin_core::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error>
    for crate::sqlx_admin_error::SqlxAdminError
{
    fn from(
        admin_entity_id_try_from_i64_error: server_admin_core::admin_entity_id_try_from_i64_error::AdminEntityIdTryFromI64Error,
    ) -> Self {
        Self::from(sqlx::Error::Decode(Box::new(
            admin_entity_id_try_from_i64_error,
        )))
    }
}
impl From<server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error>
    for crate::sqlx_admin_error::SqlxAdminError
{
    fn from(
        admin_id_try_from_i64_error: server_admin_contract::admin_id_try_from_i64_error::AdminIdTryFromI64Error,
    ) -> Self {
        Self::from(sqlx::Error::Decode(Box::new(admin_id_try_from_i64_error)))
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
    fn try_from(string: String) -> Result<Self, Self::Error> {
        let len = string.chars().count();
        if !(server_admin_contract::identity::ADMIN_PASSWORD_MIN_CHARS
            ..=server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS)
            .contains(&len)
        {
            return Err(crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError::InvalidLength);
        }
        server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(string)
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
    pub fn new(
        secrecy_admin_string: server_admin_core::secrecy_admin_string::SecrecyAdminString,
    ) -> Self {
        Self::from(secrecy_admin_string)
    }
}
impl crate::admin_password_hash::AdminPasswordHash {
    #[must_use]
    pub(crate) fn expose(&self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'_> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(self.get_inner().as_ref())
    }

    #[must_use]
    pub fn new(
        string_as_non_null_text_secret: pg_types_text_misc::generate_pg_types_mod::StringAsNonNullTextSecret,
    ) -> Self {
        Self::from(string_as_non_null_text_secret)
    }
}
impl crate::runtime_admin_jwt_secret::RuntimeAdminJwtSecret {
    #[must_use]
    pub fn new(
        secrecy_admin_string: server_admin_core::secrecy_admin_string::SecrecyAdminString,
    ) -> Self {
        Self::from(secrecy_admin_string)
    }
}
impl crate::admin_opaque_token::AdminOpaqueToken {
    #[must_use]
    pub fn new(
        secrecy_admin_string: server_admin_core::secrecy_admin_string::SecrecyAdminString,
    ) -> Self {
        Self::from(secrecy_admin_string)
    }
    #[must_use]
    pub(crate) fn expose(&self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'_> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            secrecy::ExposeSecret::expose_secret(self.get_inner().as_ref()).as_str(),
        )
    }
    pub(crate) fn clone_secret(
        &self,
    ) -> server_admin_core::secrecy_admin_string::SecrecyAdminString {
        server_admin_core::secrecy_admin_string::SecrecyAdminString::from(secrecy::SecretBox::new(
            Box::new(secrecy::ExposeSecret::expose_secret(self.get_inner()).clone()),
        ))
    }
}
impl crate::admin_refresh_token::AdminRefreshToken {
    #[must_use]
    pub fn new(admin_opaque_token: crate::admin_opaque_token::AdminOpaqueToken) -> Self {
        Self::from(admin_opaque_token)
    }
    #[must_use]
    pub fn expose(&self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'_> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            secrecy::ExposeSecret::expose_secret(self.get_inner().get_inner().as_ref()).as_str(),
        )
    }
}
impl crate::admin_token_hash::AdminTokenHash {
    #[must_use]
    pub(crate) fn new(
        secrecy_admin_string: server_admin_core::secrecy_admin_string::SecrecyAdminString,
    ) -> Self {
        Self::from(secrecy_admin_string)
    }
    #[must_use]
    pub fn expose(&self) -> server_admin_core::std_admin_str_ref::StdAdminStrRef<'_> {
        server_admin_core::std_admin_str_ref::StdAdminStrRef::from(
            secrecy::ExposeSecret::expose_secret(self.get_inner().as_ref()).as_str(),
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
        crate::hash_opaque_token::hash_opaque_token(&token).map(|hash| Self::new(hash, token))
    }
    #[must_use]
    pub const fn hash(&self) -> &crate::admin_token_hash::AdminTokenHash {
        self.get_hash()
    }
    #[must_use]
    pub const fn token(&self) -> &crate::admin_opaque_token::AdminOpaqueToken {
        self.get_token()
    }
}
impl<'headers_lt> crate::http_admin_header_map_ref::HttpAdminHeaderMapRef<'headers_lt> {
    pub(crate) const fn get(self) -> &'headers_lt http::HeaderMap {
        self.get_inner()
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
        *self.get_inner()
    }
}
impl crate::admin_unix_token_stream::AdminUnixTokenStream {
    pub(crate) const fn get(self) -> u64 {
        *self.get_inner()
    }
}
impl crate::admin_session_id::AdminSessionId {
    pub(crate) const fn get(self) -> server_admin_core::uuid_admin_value::UuidAdminValue {
        *self.get_inner()
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
        std::sync::Arc::<tokio::sync::Semaphore>::clone(self.get_semaphore().get_inner())
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
    fn from(
        std_admin_access_token_try_from_string_error: crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError,
    ) -> Self {
        match std_admin_access_token_try_from_string_error {
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::ContainsNul => Self::ContainsNul,
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::InvalidBounds { .. } => Self::InvalidBounds,
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::InvalidValue => Self::InvalidValue,
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::TooLong { .. } => Self::TooLong,
            crate::std_admin_access_token::StdAdminAccessTokenTryFromStringError::TooShort { .. } => Self::TooShort,
        }
    }
}
impl std::fmt::Debug for crate::std_admin_access_token::StdAdminAccessToken {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(constants_str::REDACTED_ALT_3)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_opaque_token_debug_is_redacted() {
        let token = crate::admin_opaque_token::AdminOpaqueToken::new(
            server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(
                constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES.to_owned(),
            )
            .expect(constants_str::DIAGNOSTIC_4F0DB163),
        );
        let debug = format!("{token:?}");
        assert!(debug.contains(constants_str::REDACTED_ALT_3));
        assert!(!debug.contains(constants_str::TEST_ONLY_ADMIN_JWT_SECRET_WITH_32_BYTES));
    }
}
