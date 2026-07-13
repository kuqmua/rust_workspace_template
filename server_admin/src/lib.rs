#![allow(clippy::arbitrary_source_item_ordering)] // domain declarations are grouped by authentication and authorization responsibility
#![cfg_attr(test, allow(unused_crate_dependencies))] // tower is used by the separate admin_api integration test target
pub mod auth;
pub mod generated_tables;
#[derive(newtype::Newtype)]
#[newtype(as_ref_owned, from_inner)]
pub struct SecrecyAdminString(secrecy::SecretBox<String>);
impl std::fmt::Debug for SecrecyAdminString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED]")
    }
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::BoundedString,
    newtype::Newtype,
)]
#[bounded_string(max = 8192, description = "administrator internal text")]
#[newtype(as_ref_owned, into_inner)]
pub struct StdAdminString(String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, std::hash::Hash, newtype::Newtype)]
#[newtype(as_ref_inner, from_inner)]
pub struct StdAdminStrRef<'value_lt>(&'value_lt str);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Newtype,
)]
#[newtype(from_inner)]
pub struct StdAdminBool(bool);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(deref_inner, from_inner)]
pub struct StdAdminNonZeroUsize(std::num::NonZeroUsize);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, newtype::Newtype,
)]
#[newtype(from_inner)]
pub struct UuidAdminValue(uuid::Uuid);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(as_ref_owned, from_inner)]
pub struct StdAdminSocketAddr(std::net::SocketAddr);
#[derive(Clone, Debug)]
pub struct StdAdminSharedSemaphore(std::sync::Arc<tokio::sync::Semaphore>);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent, from_inner)]
pub struct TokioAdminJoinEr(tokio::task::JoinError);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent, from_inner)]
pub struct TokioAdminAcquireEr(tokio::sync::AcquireError);
#[derive(Clone, Copy, newtype::Newtype)]
#[newtype(debug_transparent, from_inner)]
pub struct Argon2AdminPasswordHashEr(argon2::password_hash::Error);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent, from_inner)]
pub struct SqlxAdminEr(sqlx::Error);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Newtype,
)]
#[newtype(from_inner)]
pub struct AdminUserId(i64);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    std::hash::Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Newtype,
)]
#[newtype(from_inner)]
pub struct AdminRoleId(i64);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    std::hash::Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Newtype,
)]
#[newtype(from_inner)]
pub struct AdminPermissionId(i64);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, utoipa::ToSchema, newtype::Newtype,
)]
#[newtype(from_inner)]
pub struct AdminAuditLogId(i64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(from_inner)]
pub struct AdminPermissionName(AdminPermission);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::BoundedString,
    newtype::Newtype,
)]
#[serde(try_from = "String")]
#[bounded_string(max = 128, chars, description = "administrator login", utoipa)]
#[newtype(as_ref_owned)]
pub struct AdminLogin(String);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::BoundedString,
    newtype::Newtype,
)]
#[serde(try_from = "String")]
#[bounded_string(max = 256, chars, description = "administrator display name", utoipa)]
#[newtype(as_ref_owned)]
pub struct AdminDisplayName(String);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::BoundedString,
    newtype::Newtype,
)]
#[serde(try_from = "String")]
#[bounded_string(max = 128, chars, description = "administrator role name", utoipa)]
#[newtype(as_ref_owned)]
pub struct AdminRoleName(String);
pub struct AdminPassword(SecrecyAdminString);
impl<'schema_lt> utoipa::ToSchema<'schema_lt> for AdminPassword {
    fn schema() -> (
        &'schema_lt str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        (
            "AdminPassword",
            utoipa::openapi::ObjectBuilder::new()
                .schema_type(utoipa::openapi::schema::SchemaType::String)
                .min_length(Some(1usize))
                .max_length(Some(1024usize))
                .write_only(Some(true))
                .build()
                .into(),
        )
    }
}
impl<'de> serde::Deserialize<'de> for AdminPassword {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        if value.is_empty() || value.len() > 1024usize {
            return Err(serde::de::Error::custom(
                "administrator password length is invalid",
            ));
        }
        Ok(Self(SecrecyAdminString::from(secrecy::SecretBox::new(
            Box::new(value),
        ))))
    }
}
impl AdminPassword {
    #[must_use]
    pub const fn new(value: SecrecyAdminString) -> Self {
        Self(value)
    }
    fn into_inner(self) -> SecrecyAdminString {
        self.0
    }
}
impl std::fmt::Debug for AdminPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AdminPassword").field(&"[REDACTED]").finish()
    }
}
pub struct AdminPasswordHash(pg_types_text_misc::StringAsNnTextSecret);
impl AdminPasswordHash {
    #[must_use]
    pub const fn new(value: pg_types_text_misc::StringAsNnTextSecret) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for AdminPasswordHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AdminPasswordHash")
            .field(&"[REDACTED]")
            .finish()
    }
}
pub struct AdminJwtSecret(SecrecyAdminString);
impl AdminJwtSecret {
    #[must_use]
    pub const fn new(value: SecrecyAdminString) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for AdminJwtSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AdminJwtSecret")
            .field(&"[REDACTED]")
            .finish()
    }
}
pub struct AdminOpaqueToken(SecrecyAdminString);
impl AdminOpaqueToken {
    #[must_use]
    pub const fn new(value: SecrecyAdminString) -> Self {
        Self(value)
    }
}
impl std::fmt::Debug for AdminOpaqueToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AdminOpaqueToken")
            .field(&"[REDACTED]")
            .finish()
    }
}
pub struct AdminRefreshToken(AdminOpaqueToken);
impl AdminRefreshToken {
    #[must_use]
    pub const fn new(value: AdminOpaqueToken) -> Self {
        Self(value)
    }
    #[must_use]
    pub fn expose(&self) -> StdAdminStrRef<'_> {
        StdAdminStrRef::from(secrecy::ExposeSecret::expose_secret(self.0.0.as_ref()).as_str())
    }
}
impl std::fmt::Debug for AdminRefreshToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AdminRefreshToken")
            .field(&"[REDACTED]")
            .finish()
    }
}
pub struct AdminTokenHash(SecrecyAdminString);
impl AdminTokenHash {
    #[must_use]
    pub const fn new(value: SecrecyAdminString) -> Self {
        Self(value)
    }
    #[must_use]
    pub fn expose(&self) -> StdAdminStrRef<'_> {
        StdAdminStrRef::from(secrecy::ExposeSecret::expose_secret(self.0.as_ref()).as_str())
    }
}
impl std::fmt::Debug for AdminTokenHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AdminTokenHash")
            .field(&"[REDACTED]")
            .finish()
    }
}
#[derive(Debug)]
pub struct AdminGeneratedToken {
    hash: AdminTokenHash,
    token: AdminOpaqueToken,
}
impl AdminGeneratedToken {
    #[must_use]
    pub fn generate() -> Self {
        let token = AdminOpaqueToken::new(SecrecyAdminString::from(secrecy::SecretBox::new(
            Box::new(format!("{}.{}", uuid::Uuid::new_v4(), uuid::Uuid::new_v4())),
        )));
        let hash = hash_opaque_token(&token);
        Self { hash, token }
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
#[must_use]
pub fn hash_opaque_token(token: &AdminOpaqueToken) -> AdminTokenHash {
    let digest = <sha2::Sha256 as sha2::Digest>::digest(
        secrecy::ExposeSecret::expose_secret(token.0.as_ref()).as_bytes(),
    );
    AdminTokenHash::new(SecrecyAdminString::from(secrecy::SecretBox::new(Box::new(
        format!("{digest:x}"),
    ))))
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(from_inner)]
pub struct AdminCookieSecure(bool);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(from_inner)]
pub struct AdminCookieMaxAgeSeconds(u64);
#[derive(Debug, Clone, PartialEq, Eq, newtype::BoundedString, newtype::Newtype)]
#[bounded_string(max = 8192, description = "administrator cookie")]
#[newtype(as_ref_owned, into_inner)]
pub struct StdAdminCookie(String);
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(as_ref_inner, from_inner)]
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
            Self::Access => "admin_access_token",
            Self::Csrf => "admin_csrf_token",
            Self::Refresh => "admin_refresh_token",
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
    let http_only = if kind.is_http_only().0 {
        "; HttpOnly"
    } else {
        ""
    };
    let secure_attr = if secure.0 { "; Secure" } else { "" };
    StdAdminCookie(format!(
        "{}={}; Path=/; Max-Age={}; SameSite=Strict{http_only}{secure_attr}",
        kind.name().as_ref(),
        value.as_ref(),
        max_age.0
    ))
}
#[must_use]
pub fn clear_admin_cookie(kind: AdminCookieKind, secure: AdminCookieSecure) -> StdAdminCookie {
    build_admin_cookie(
        kind,
        StdAdminStrRef::from(""),
        AdminCookieMaxAgeSeconds::from(0),
        secure,
    )
}
#[must_use]
pub fn find_admin_cookie(
    headers: HttpAdminHeaderMapRef<'_>,
    kind: AdminCookieKind,
) -> Option<StdAdminStrRef<'_>> {
    let cookies = headers
        .0
        .get(http::header::COOKIE)
        .and_then(|header| header.to_str().ok())?;
    cookies.split(';').find_map(|cookie| {
        let (name, value) = cookie.trim().split_once('=')?;
        (name == kind.name().as_ref()).then_some(StdAdminStrRef::from(value))
    })
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(from_inner)]
pub struct AdminPasswordHashConcurrency(StdAdminNonZeroUsize);
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, newtype::Newtype,
)]
#[newtype(from_inner)]
pub struct AdminUnixTs(u64);
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    newtype::Newtype,
)]
#[newtype(from_inner)]
pub struct AdminSessionId(UuidAdminValue);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::BoundedString,
    newtype::Newtype,
)]
#[bounded_string(max = 256, description = "administrator access token issuer")]
#[newtype(as_ref_owned)]
pub struct AdminTokenIssuer(String);
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    newtype::BoundedString,
    newtype::Newtype,
)]
#[bounded_string(max = 256, description = "administrator access token audience")]
#[newtype(as_ref_owned)]
pub struct AdminTokenAudience(String);
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AdminAccessClaims {
    aud: AdminTokenAudience,
    exp: AdminUnixTs,
    iat: AdminUnixTs,
    iss: AdminTokenIssuer,
    jti: AdminSessionId,
    sub: AdminUserId,
}
impl AdminAccessClaims {
    #[must_use]
    pub const fn new(
        user_id: AdminUserId,
        session_id: AdminSessionId,
        issued_at: AdminUnixTs,
        expires_at: AdminUnixTs,
        issuer: AdminTokenIssuer,
        audience: AdminTokenAudience,
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
pub enum AdminPasswordHashEr {
    #[error("administrator password hashing task failed: {0:?}")]
    Join(TokioAdminJoinEr),
    #[error("administrator password hashing failed: {0:?}")]
    PasswordHash(Argon2AdminPasswordHashEr),
    #[error("administrator password hashing concurrency limiter was closed: {0:?}")]
    SemaphoreClosed(TokioAdminAcquireEr),
}
#[derive(Clone, Debug)]
pub struct AdminPasswordHasher {
    semaphore: StdAdminSharedSemaphore,
}
impl AdminPasswordHasher {
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Tokio semaphore and Arc constructors are not const
    pub fn new(max_concurrent_hashes: AdminPasswordHashConcurrency) -> Self {
        Self {
            semaphore: StdAdminSharedSemaphore(std::sync::Arc::new(tokio::sync::Semaphore::new(
                max_concurrent_hashes.0.0.get(),
            ))),
        }
    }
    pub async fn hash(
        &self,
        password: AdminPassword,
    ) -> Result<AdminPasswordHash, AdminPasswordHashEr> {
        let permit = std::sync::Arc::<tokio::sync::Semaphore>::clone(&self.semaphore.0)
            .acquire_owned()
            .await
            .map_err(|er| AdminPasswordHashEr::SemaphoreClosed(TokioAdminAcquireEr::from(er)))?;
        tokio::task::spawn_blocking(move || {
            let password_secret = password.into_inner();
            let salt = argon2::password_hash::SaltString::generate(
                &mut argon2::password_hash::rand_core::OsRng,
            );
            let result = argon2::PasswordHasher::hash_password(
                &argon2::Argon2::default(),
                secrecy::ExposeSecret::expose_secret(password_secret.as_ref()).as_bytes(),
                &salt,
            )
            .map(|hash| {
                AdminPasswordHash::new(pg_types_text_misc::StringAsNnTextSecret::from(
                    hash.to_string(),
                ))
            })
            .map_err(|er| AdminPasswordHashEr::PasswordHash(Argon2AdminPasswordHashEr::from(er)));
            drop(permit);
            result
        })
        .await
        .map_err(|er| AdminPasswordHashEr::Join(TokioAdminJoinEr::from(er)))?
    }
    pub async fn verify(
        &self,
        password: AdminPassword,
        expected_hash: AdminPasswordHash,
    ) -> Result<StdAdminBool, AdminPasswordHashEr> {
        let permit = std::sync::Arc::<tokio::sync::Semaphore>::clone(&self.semaphore.0)
            .acquire_owned()
            .await
            .map_err(|er| AdminPasswordHashEr::SemaphoreClosed(TokioAdminAcquireEr::from(er)))?;
        tokio::task::spawn_blocking(move || {
            let password_secret = password.into_inner();
            let parsed_hash =
                argon2::PasswordHash::new(expected_hash.0.as_ref()).map_err(|er| {
                    AdminPasswordHashEr::PasswordHash(Argon2AdminPasswordHashEr::from(er))
                })?;
            let result = argon2::PasswordVerifier::verify_password(
                &argon2::Argon2::default(),
                secrecy::ExposeSecret::expose_secret(password_secret.as_ref()).as_bytes(),
                &parsed_hash,
            );
            drop(permit);
            match result {
                Ok(()) => Ok(StdAdminBool::from(true)),
                Err(argon2::password_hash::Error::Password) => Ok(StdAdminBool::from(false)),
                Err(er) => Err(AdminPasswordHashEr::PasswordHash(
                    Argon2AdminPasswordHashEr::from(er),
                )),
            }
        })
        .await
        .map_err(|er| AdminPasswordHashEr::Join(TokioAdminJoinEr::from(er)))?
    }
}
#[derive(newtype::Newtype)]
#[newtype(debug_transparent, from_inner)]
pub struct JsonwebtokenAdminEr(jsonwebtoken::errors::Error);
#[derive(Debug, thiserror::Error)]
#[error("administrator access token operation failed: {0:?}")]
pub struct AdminAccessTokenEr(JsonwebtokenAdminEr);
#[derive(Debug, Clone, PartialEq, Eq, newtype::BoundedString, newtype::Newtype)]
#[bounded_string(max = 8192, description = "administrator access token")]
#[newtype(as_ref_owned, into_inner)]
pub struct StdAdminAccessToken(String);
pub fn encode_access_token(
    claims: &AdminAccessClaims,
    secret: &AdminJwtSecret,
) -> Result<StdAdminAccessToken, AdminAccessTokenEr> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        claims,
        &jsonwebtoken::EncodingKey::from_secret(
            secrecy::ExposeSecret::expose_secret(secret.0.as_ref()).as_bytes(),
        ),
    )
    .map(StdAdminAccessToken)
    .map_err(|er| AdminAccessTokenEr(JsonwebtokenAdminEr::from(er)))
}
pub fn decode_access_token(
    token: &StdAdminAccessToken,
    secret: &AdminJwtSecret,
    issuer: &AdminTokenIssuer,
    audience: &AdminTokenAudience,
) -> Result<AdminAccessClaims, AdminAccessTokenEr> {
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
    .map_err(|er| AdminAccessTokenEr(JsonwebtokenAdminEr::from(er)))
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, utoipa::ToSchema, optml::Optml)]
#[serde(rename_all = "snake_case")]
pub enum AdminPermission {
    AuditLogRead,
    MetricsRead,
    OpenApiRead,
    PermissionsRead,
    RolePermissionsCreate,
    RolePermissionsDelete,
    RolePermissionsRead,
    RolePermissionsUpdate,
    RolesCreate,
    RolesDelete,
    RolesRead,
    RolesUpdate,
    SystemSettingsRead,
    SystemSettingsUpdate,
    UserRolesCreate,
    UserRolesDelete,
    UserRolesRead,
    UserRolesUpdate,
    UsersCreate,
    UsersDelete,
    UsersRead,
    UsersUpdate,
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
impl AdminAuditAction {
    #[must_use]
    pub fn as_str(self) -> StdAdminStrRef<'static> {
        StdAdminStrRef::from(match self {
            Self::Create => "create",
            Self::Delete => "delete",
            Self::Refresh => "refresh",
            Self::SignIn => "sign_in",
            Self::SignOut => "sign_out",
            Self::Update => "update",
        })
    }
}
impl AdminAuditResource {
    #[must_use]
    pub fn as_str(self) -> StdAdminStrRef<'static> {
        StdAdminStrRef::from(match self {
            Self::AuditLog => "audit_log",
            Self::Permission => "permission",
            Self::Role => "role",
            Self::Session => "session",
            Self::SystemSettings => "system_settings",
            Self::User => "user",
        })
    }
}
impl AdminPermission {
    #[must_use]
    pub fn as_str(self) -> StdAdminStrRef<'static> {
        StdAdminStrRef::from(match self {
            Self::AuditLogRead => "audit_log:read",
            Self::MetricsRead => "metrics:read",
            Self::OpenApiRead => "openapi:read",
            Self::PermissionsRead => "permissions:read",
            Self::RolePermissionsCreate => "role_permissions:create",
            Self::RolePermissionsDelete => "role_permissions:delete",
            Self::RolePermissionsRead => "role_permissions:read",
            Self::RolePermissionsUpdate => "role_permissions:update",
            Self::RolesCreate => "roles:create",
            Self::RolesDelete => "roles:delete",
            Self::RolesRead => "roles:read",
            Self::RolesUpdate => "roles:update",
            Self::SystemSettingsRead => "system_settings:read",
            Self::SystemSettingsUpdate => "system_settings:update",
            Self::UserRolesCreate => "user_roles:create",
            Self::UserRolesDelete => "user_roles:delete",
            Self::UserRolesRead => "user_roles:read",
            Self::UserRolesUpdate => "user_roles:update",
            Self::UsersCreate => "users:create",
            Self::UsersDelete => "users:delete",
            Self::UsersRead => "users:read",
            Self::UsersUpdate => "users:update",
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("unknown administrator permission: {value:?}")]
pub struct AdminPermissionTryFromStrEr {
    value: StdAdminString,
}
impl TryFrom<&str> for AdminPermission {
    type Error = AdminPermissionTryFromStrEr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "audit_log:read" => Ok(Self::AuditLogRead),
            "metrics:read" => Ok(Self::MetricsRead),
            "openapi:read" => Ok(Self::OpenApiRead),
            "permissions:read" => Ok(Self::PermissionsRead),
            "role_permissions:create" => Ok(Self::RolePermissionsCreate),
            "role_permissions:delete" => Ok(Self::RolePermissionsDelete),
            "role_permissions:read" => Ok(Self::RolePermissionsRead),
            "role_permissions:update" => Ok(Self::RolePermissionsUpdate),
            "roles:create" => Ok(Self::RolesCreate),
            "roles:delete" => Ok(Self::RolesDelete),
            "roles:read" => Ok(Self::RolesRead),
            "roles:update" => Ok(Self::RolesUpdate),
            "system_settings:read" => Ok(Self::SystemSettingsRead),
            "system_settings:update" => Ok(Self::SystemSettingsUpdate),
            "user_roles:create" => Ok(Self::UserRolesCreate),
            "user_roles:delete" => Ok(Self::UserRolesDelete),
            "user_roles:read" => Ok(Self::UserRolesRead),
            "user_roles:update" => Ok(Self::UserRolesUpdate),
            "users:create" => Ok(Self::UsersCreate),
            "users:delete" => Ok(Self::UsersDelete),
            "users:read" => Ok(Self::UsersRead),
            "users:update" => Ok(Self::UsersUpdate),
            _ => Err(AdminPermissionTryFromStrEr {
                value: StdAdminString(value.to_owned()),
            }),
        }
    }
}
static ADMIN_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
#[derive(newtype::Newtype)]
#[newtype(debug_transparent, from_inner)]
pub struct SqlxAdminMigrateEr(sqlx::migrate::MigrateError);
#[derive(Debug, thiserror::Error)]
#[error("failed to migrate administrator schema: {0:?}")]
pub struct AdminMigrateEr(SqlxAdminMigrateEr);
pub async fn prep_pg(pool: app_state::SqlxPgPoolRef<'_>) -> Result<(), AdminMigrateEr> {
    ADMIN_MIGRATOR
        .run(pool.as_ref())
        .await
        .map_err(|er| AdminMigrateEr(SqlxAdminMigrateEr::from(er)))
}
#[derive(Debug, thiserror::Error)]
pub enum AdminBootstrapEr {
    #[error("administrator bootstrap display name is empty")]
    EmptyDisplayName,
    #[error("administrator bootstrap login has an invalid format")]
    InvalidLogin,
    #[error("administrator bootstrap has already been completed")]
    AlreadyInitialized,
    #[error("administrator bootstrap password hashing failed: {0}")]
    PasswordHash(AdminPasswordHashEr),
    #[error("administrator bootstrap database operation failed: {0:?}")]
    Pg(SqlxAdminEr),
}
#[allow(clippy::single_call_fn)] // shared validator keeps bootstrap behavior directly unit-testable and aligned with the database constraint
fn admin_login_has_valid_format(login: &AdminLogin) -> StdAdminBool {
    let value: &String = login.as_ref();
    StdAdminBool::from(
        value.len() >= 3
            && value.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'_' | b'.' | b'-')
            }),
    )
}
pub async fn bootstrap_admin(
    pool: app_state::SqlxPgPoolRef<'_>,
    login: AdminLogin,
    display_name: AdminDisplayName,
    password: AdminPassword,
    password_hasher: &AdminPasswordHasher,
) -> Result<AdminUserId, AdminBootstrapEr> {
    if !admin_login_has_valid_format(&login).0 {
        return Err(AdminBootstrapEr::InvalidLogin);
    }
    if display_name.as_ref().trim().is_empty() {
        return Err(AdminBootstrapEr::EmptyDisplayName);
    }
    let password_hash = password_hasher
        .hash(password)
        .await
        .map_err(AdminBootstrapEr::PasswordHash)?;
    let mut tx = pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminBootstrapEr::Pg(SqlxAdminEr::from(er)))?;
    let _lock_result = sqlx::query("LOCK TABLE admin_users IN EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(|er| AdminBootstrapEr::Pg(SqlxAdminEr::from(er)))?;
    let user_exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS (SELECT 1 FROM admin_users)")
        .fetch_one(&mut *tx)
        .await
        .map_err(|er| AdminBootstrapEr::Pg(SqlxAdminEr::from(er)))?;
    if user_exists {
        return Err(AdminBootstrapEr::AlreadyInitialized);
    }
    let user_id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO admin_users (login, display_name, password_hash) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(login.as_ref())
    .bind(display_name.as_ref())
    .bind(password_hash.0.as_ref())
    .fetch_one(&mut *tx)
    .await
    .map_err(|er| AdminBootstrapEr::Pg(SqlxAdminEr::from(er)))?;
    let _role_link_result = sqlx::query(
        "INSERT INTO admin_user_roles (user_id, role_id) SELECT $1, id FROM admin_roles WHERE name = 'admin'",
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await
    .map_err(|er| AdminBootstrapEr::Pg(SqlxAdminEr::from(er)))?;
    tx.commit()
        .await
        .map_err(|er| AdminBootstrapEr::Pg(SqlxAdminEr::from(er)))?;
    Ok(AdminUserId::from(user_id))
}
#[cfg(test)]
#[allow(clippy::needless_for_each, clippy::single_call_fn)] // repository policy forbids for loops and compact fixtures keep secret setup deterministic
mod tests {
    fn secret(value: &str) -> super::SecrecyAdminString {
        super::SecrecyAdminString::from(secrecy::SecretBox::new(Box::new(value.to_owned())))
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
        super::AdminJwtSecret::new(secret("test-only-secret-with-sufficient-entropy"))
    }
    #[test]
    fn permission_round_trip_is_exhaustive() {
        [
            super::AdminPermission::AuditLogRead,
            super::AdminPermission::MetricsRead,
            super::AdminPermission::OpenApiRead,
            super::AdminPermission::PermissionsRead,
            super::AdminPermission::RolePermissionsCreate,
            super::AdminPermission::RolePermissionsDelete,
            super::AdminPermission::RolePermissionsRead,
            super::AdminPermission::RolePermissionsUpdate,
            super::AdminPermission::RolesCreate,
            super::AdminPermission::RolesDelete,
            super::AdminPermission::RolesRead,
            super::AdminPermission::RolesUpdate,
            super::AdminPermission::SystemSettingsRead,
            super::AdminPermission::SystemSettingsUpdate,
            super::AdminPermission::UserRolesCreate,
            super::AdminPermission::UserRolesDelete,
            super::AdminPermission::UserRolesRead,
            super::AdminPermission::UserRolesUpdate,
            super::AdminPermission::UsersCreate,
            super::AdminPermission::UsersDelete,
            super::AdminPermission::UsersRead,
            super::AdminPermission::UsersUpdate,
        ]
        .into_iter()
        .for_each(|permission| {
            assert_eq!(
                super::AdminPermission::try_from(permission.as_str().as_ref()).expect("0f53b75c"),
                permission
            );
        });
    }
    #[test]
    fn unknown_permission_is_rejected() {
        drop(super::AdminPermission::try_from("unknown:read").expect_err("b482b167"));
    }
    #[test]
    fn migration_inventory_is_not_empty() {
        let migrations = super::ADMIN_MIGRATOR.iter().collect::<Vec<_>>();
        assert_eq!(migrations.len(), 3usize);
        assert!(
            migrations
                .iter()
                .any(|migration| migration.description == "admin rate limits")
        );
    }
    #[tokio::test]
    async fn password_hash_verifies_only_matching_password() {
        let hasher = password_hasher();
        let hash = hasher
            .hash(password("correct password"))
            .await
            .expect("174a5d2f");
        assert!(
            hasher
                .verify(password("correct password"), hash)
                .await
                .expect("604f40be")
                .0
        );
        let other_hash = hasher
            .hash(password("correct password"))
            .await
            .expect("38819b94");
        assert!(
            !hasher
                .verify(password("wrong password"), other_hash)
                .await
                .expect("ed6b499a")
                .0
        );
    }
    #[test]
    fn secrets_are_redacted_in_debug_output() {
        let raw_secret = "never-print-this-value";
        let password = password(raw_secret);
        let jwt_secret = super::AdminJwtSecret::new(secret(raw_secret));
        assert!(!format!("{password:?}").contains(raw_secret));
        assert!(!format!("{jwt_secret:?}").contains(raw_secret));
    }
    #[test]
    fn generated_token_hash_is_stable_and_does_not_expose_token() {
        let token = super::AdminOpaqueToken::new(secret("fixed-test-token"));
        let hash = super::hash_opaque_token(&token);
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
            super::StdAdminStrRef::from("access"),
            super::AdminCookieMaxAgeSeconds::from(60),
            super::AdminCookieSecure::from(true),
        );
        let csrf = super::build_admin_cookie(
            super::AdminCookieKind::Csrf,
            super::StdAdminStrRef::from("csrf"),
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
                "other=1; admin_access_token=expected; admin_access_token_suffix=wrong",
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
        let valid = super::AdminLogin::try_from("admin.user-1".to_owned()).expect("078c759d");
        let uppercase = super::AdminLogin::try_from("Admin".to_owned()).expect("a164aedd");
        let too_short = super::AdminLogin::try_from("ab".to_owned()).expect("735a2858");
        assert!(super::admin_login_has_valid_format(&valid).0);
        assert!(!super::admin_login_has_valid_format(&uppercase).0);
        assert!(!super::admin_login_has_valid_format(&too_short).0);
    }
    #[test]
    fn access_token_round_trip_checks_issuer_and_audience() {
        let claims = super::AdminAccessClaims::new(
            super::AdminUserId::from(7),
            super::AdminSessionId::from(super::UuidAdminValue::from(
                uuid::Uuid::parse_str("b871bd8f-7810-4d4b-94a1-5458d3016907").expect("05562da0"),
            )),
            super::AdminUnixTs::from(1),
            super::AdminUnixTs::from(4_102_444_800),
            super::AdminTokenIssuer::try_from("test-issuer".to_owned()).expect("fd6a65b0"),
            super::AdminTokenAudience::try_from("test-audience".to_owned()).expect("6e423e16"),
        );
        let secret = jwt_secret();
        let token = super::encode_access_token(&claims, &secret).expect("b41052bc");
        let issuer = super::AdminTokenIssuer::try_from("test-issuer".to_owned()).expect("5edc807f");
        let audience =
            super::AdminTokenAudience::try_from("test-audience".to_owned()).expect("0c3975a1");
        let decoded =
            super::decode_access_token(&token, &secret, &issuer, &audience).expect("0ed905ff");
        assert_eq!(decoded.user_id(), super::AdminUserId::from(7));
        assert_eq!(decoded.session_id(), claims.session_id());
        drop(
            super::decode_access_token(
                &token,
                &secret,
                &issuer,
                &super::AdminTokenAudience::try_from("wrong-audience".to_owned())
                    .expect("92f9c5ec"),
            )
            .expect_err("a82438cc"),
        );
    }
}
