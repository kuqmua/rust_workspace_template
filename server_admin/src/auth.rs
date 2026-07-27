#![allow(clippy::needless_for_each)] // utoipa 4 generated OpenAPI registration uses iterator callbacks
mod html;
#[derive(newtype::DebugTransparent, newtype::FromInner)]
pub struct JsonwebtokenAdminEncodingKey(jsonwebtoken::EncodingKey);
#[derive(Debug, newtype::AsRefTarget, newtype::FromInner)]
struct JsonwebtokenAdminDecodingKeys(Vec<jsonwebtoken::DecodingKey>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::IntoInnerFrom, newtype::TryFrom)]
#[try_from(
    error = AdminAuthPositiveValueError,
    validator = StdAdminAccessTtlSeconds::validate
)]
pub struct StdAdminAccessTtlSeconds(u64);
impl StdAdminAccessTtlSeconds {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &u64) -> Result<(), AdminAuthPositiveValueError> {
        if *value == 0u64 {
            Err(AdminAuthPositiveValueError)
        } else {
            Ok(())
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::IntoInnerFrom, newtype::TryFrom)]
#[try_from(
    error = AdminAuthPositiveValueError,
    validator = StdAdminRefreshTtlSeconds::validate
)]
pub struct StdAdminRefreshTtlSeconds(u64);
impl StdAdminRefreshTtlSeconds {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &u64) -> Result<(), AdminAuthPositiveValueError> {
        if *value == 0u64 {
            Err(AdminAuthPositiveValueError)
        } else {
            Ok(())
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::IntoInnerFrom, newtype::TryFrom)]
#[try_from(
    error = AdminAuthPositiveValueError,
    validator = StdAdminSessionLimit::validate
)]
pub struct StdAdminSessionLimit(usize);
impl StdAdminSessionLimit {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &usize) -> Result<(), AdminAuthPositiveValueError> {
        if *value == 0usize {
            Err(AdminAuthPositiveValueError)
        } else {
            Ok(())
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::IntoInnerFrom, newtype::TryFrom)]
#[try_from(
    error = AdminAuthPositiveValueError,
    validator = StdAdminFailureThreshold::validate
)]
pub struct StdAdminFailureThreshold(i64);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AdminKnownFailureThreshold {
    Default,
}
impl From<AdminKnownFailureThreshold> for StdAdminFailureThreshold {
    fn from(value: AdminKnownFailureThreshold) -> Self {
        match value {
            AdminKnownFailureThreshold::Default => Self(10i64),
        }
    }
}
impl StdAdminFailureThreshold {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &i64) -> Result<(), AdminAuthPositiveValueError> {
        if *value <= 0i64 {
            Err(AdminAuthPositiveValueError)
        } else {
            Ok(())
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{self:?}")]
pub struct AdminAuthPositiveValueError;
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
pub struct StdAdminFailureDelayMillis(u64);
#[derive(Debug, Clone, Copy, newtype::FromInner, newtype::IntoInnerFrom)]
pub(crate) struct StdAdminRateLimitCount(i64);
#[derive(Debug, Clone, Copy, newtype::FromInner, newtype::IntoInnerFrom)]
pub(crate) struct StdAdminRateLimitWindowSeconds(i32);
#[derive(Debug, Clone, Copy)]
pub struct AdminAuthPolicy {
    audit_export_limit: StdAdminRateLimitCount,
    audit_export_window: StdAdminRateLimitWindowSeconds,
    failure_delay: StdAdminFailureDelayMillis,
    failure_threshold: StdAdminFailureThreshold,
    mutation_limit: StdAdminRateLimitCount,
    mutation_window: StdAdminRateLimitWindowSeconds,
    refresh_limit: StdAdminRateLimitCount,
    refresh_window: StdAdminRateLimitWindowSeconds,
    sign_in_ip_limit: StdAdminRateLimitCount,
    sign_in_limit: StdAdminRateLimitCount,
    sign_in_window: StdAdminRateLimitWindowSeconds,
}
impl AdminAuthPolicy {
    #[allow(
        clippy::single_call_fn,
        reason = "keeps every administrator authentication threshold in one immutable policy constructor"
    )]
    fn from_sign_in_limit(sign_in_limit: StdAdminRateLimitCount) -> Self {
        Self {
            audit_export_limit: StdAdminRateLimitCount::from(60i64),
            audit_export_window: StdAdminRateLimitWindowSeconds::from(60i32),
            failure_delay: StdAdminFailureDelayMillis::from(200u64),
            failure_threshold: StdAdminFailureThreshold::from(AdminKnownFailureThreshold::Default),
            mutation_limit: StdAdminRateLimitCount::from(300i64),
            mutation_window: StdAdminRateLimitWindowSeconds::from(60i32),
            refresh_limit: StdAdminRateLimitCount::from(60i64),
            refresh_window: StdAdminRateLimitWindowSeconds::from(900i32),
            sign_in_ip_limit: StdAdminRateLimitCount::from(sign_in_limit.0.saturating_mul(5i64)),
            sign_in_limit,
            sign_in_window: StdAdminRateLimitWindowSeconds::from(900i32),
        }
    }
}
#[derive(Debug)]
pub struct AdminAuthSvcState {
    access_ttl: StdAdminAccessTtlSeconds,
    allowed_origins: server_runtime::AllowedOrigins,
    audience: config_lib::AdminTokenAudience,
    cookie_secure: super::AdminCookieSecure,
    decoding_keys: JsonwebtokenAdminDecodingKeys,
    encoding_key: JsonwebtokenAdminEncodingKey,
    issuer: config_lib::AdminTokenIssuer,
    password_hasher: super::AdminPasswordHasher,
    policy: AdminAuthPolicy,
    pool: app_state::SqlxPgPool,
    refresh_ttl: StdAdminRefreshTtlSeconds,
    session_limit: StdAdminSessionLimit,
}
#[derive(Clone, Debug, newtype::AsRefOwned, newtype::FromInner)]
pub struct StdSharedAdminAuthSvcState(std::sync::Arc<AdminAuthSvcState>);
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum AdminAuthSvcStateBuildError {
    #[error("administrator allowed origin is invalid")]
    AllowedOrigin,
    #[error("administrator JWT secret list is empty")]
    JwtSecret,
    #[error("administrator password hash concurrency is zero")]
    PasswordHashConcurrency,
    #[error("administrator authentication numeric value is not positive")]
    PositiveValue(#[source] AdminAuthPositiveValueError),
}
#[allow(clippy::single_call_fn)] // sign-in accepts existing credentials without applying the policy for newly assigned passwords
fn admin_password_from_contract(
    value: server_admin_contract::AdminPassword,
) -> super::AdminPassword {
    super::AdminPassword::new(super::SecrecyAdminString::from(secrecy::SecretBox::new(
        Box::new(value.into_inner()),
    )))
}
fn admin_new_password_from_contract(
    value: server_admin_contract::AdminNewPassword,
) -> super::AdminPassword {
    super::AdminPassword::new(super::SecrecyAdminString::from(secrecy::SecretBox::new(
        Box::new(value.into_inner()),
    )))
}
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct AuthenticatedAdmin {
    display_name: super::AdminDisplayName,
    id: super::AdminUserId,
    login: super::AdminLogin,
    permissions: super::AdminPermissions,
    roles: super::AdminRoleNames,
    session_id: super::AdminSessionId,
}
impl AuthenticatedAdmin {
    #[must_use]
    pub const fn id(&self) -> super::AdminUserId {
        self.id
    }
}
fn authenticated_admin_contract(
    value: &AuthenticatedAdmin,
) -> Result<server_admin_contract::AuthenticatedAdmin, AdminError> {
    let permissions = value
        .permissions
        .as_ref()
        .iter()
        .map(|permission| {
            server_admin_contract::AdminPermissionValue::try_from(
                permission.as_str().as_ref().to_owned(),
            )
            .map_err(|_error| AdminError::Validation)
        })
        .collect::<Result<Vec<_>, AdminError>>()?;
    let roles = value
        .roles
        .as_ref()
        .iter()
        .map(|role| {
            server_admin_contract::AdminRoleName::try_from(role.as_ref().to_owned())
                .map_err(|_error| AdminError::Validation)
        })
        .collect::<Result<Vec<_>, AdminError>>()?;
    Ok(server_admin_contract::AuthenticatedAdmin::new(
        server_admin_contract::AdminDisplayName::try_from(value.display_name.as_ref().to_owned())
            .map_err(|_error| AdminError::Validation)?,
        server_admin_contract::AdminUserId::from(value.id.value()),
        server_admin_contract::AdminLogin::try_from(value.login.as_ref().to_owned())
            .map_err(|_error| AdminError::Validation)?,
        server_admin_contract::AdminPermissionValues::try_from(permissions)
            .map_err(|_error| AdminError::Validation)?,
        server_admin_contract::AdminRoleNames::try_from(roles)
            .map_err(|_error| AdminError::Validation)?,
    ))
}
#[derive(Clone, Debug, serde::Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct AdminAuditQuery {
    #[param(inline)]
    action: Option<super::AdminAuditAction>,
    created_after: Option<server_admin_contract::AdminAuditTimestamp>,
    created_before: Option<server_admin_contract::AdminAuditTimestamp>,
    cursor_created_at: Option<server_admin_contract::AdminAuditTimestamp>,
    cursor_id: Option<server_admin_contract::AdminAuditLogId>,
    #[serde(default)]
    #[param(value_type = u16, minimum = 1, maximum = 100)]
    limit: server_admin_contract::AdminPageLimit,
    #[serde(default)]
    #[param(value_type = u32)]
    offset: server_admin_contract::AdminPageOffset,
    #[param(inline)]
    resource: Option<super::AdminAuditResource>,
    resource_id: Option<server_admin_contract::AdminText>,
    succeeded: Option<server_admin_contract::AdminBool>,
    #[param(inline)]
    user_id: Option<super::AdminUserId>,
    user_login: Option<server_admin_contract::AdminLogin>,
}
#[allow(clippy::field_scoped_visibility_modifiers)] // repository query binding consumes this internal cross-module DTO field-by-field
pub(crate) struct AdminAuditQueryParts {
    pub(crate) action: Option<super::AdminAuditAction>,
    pub(crate) created_after: Option<server_admin_contract::AdminAuditTimestamp>,
    pub(crate) created_before: Option<server_admin_contract::AdminAuditTimestamp>,
    pub(crate) cursor_created_at: Option<server_admin_contract::AdminAuditTimestamp>,
    pub(crate) cursor_id: Option<server_admin_contract::AdminAuditLogId>,
    pub(crate) limit: server_admin_contract::AdminPageLimit,
    pub(crate) offset: server_admin_contract::AdminPageOffset,
    pub(crate) resource: Option<super::AdminAuditResource>,
    pub(crate) resource_id: Option<server_admin_contract::AdminText>,
    pub(crate) succeeded: Option<server_admin_contract::AdminBool>,
    pub(crate) user_id: Option<super::AdminUserId>,
    pub(crate) user_login: Option<server_admin_contract::AdminLogin>,
}
impl AdminAuditQuery {
    pub(crate) fn cursor_is_complete(&self) -> super::StdAdminBool {
        super::StdAdminBool::from(self.cursor_created_at.is_some() == self.cursor_id.is_some())
    }
    pub(crate) fn into_parts(self) -> AdminAuditQueryParts {
        AdminAuditQueryParts {
            action: self.action,
            created_after: self.created_after,
            created_before: self.created_before,
            cursor_created_at: self.cursor_created_at,
            cursor_id: self.cursor_id,
            limit: self.limit,
            offset: self.offset,
            resource: self.resource,
            resource_id: self.resource_id,
            succeeded: self.succeeded,
            user_id: self.user_id,
            user_login: self.user_login,
        }
    }
}
#[derive(Clone, Debug, newtype::AsRefOwned, newtype::FromInner)]
pub struct HttpAdminHeaderMap(http::HeaderMap);
#[derive(Debug, Clone)]
pub(crate) struct AdminAuthReq {
    headers: HttpAdminHeaderMap,
    peer: AdminPeerAddr,
    state: StdSharedAdminAuthSvcState,
}
#[derive(Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct AdminPeerAddr(super::StdAdminSocketAddr);
impl AdminPeerAddr {
    pub(crate) const fn socket_addr(self) -> super::StdAdminSocketAddr {
        self.0
    }
}
impl<State> axum::extract::FromRequestParts<State> for AdminPeerAddr
where
    State: Send + Sync,
{
    type Rejection = AdminError;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &State,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(
            parts
                .extensions
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|value| Self::from(super::StdAdminSocketAddr::from(value.0)))
                .ok_or(AdminError::Authentication),
        )
    }
}
#[derive(Debug, newtype::FromInner)]
pub(crate) struct AdminSignInJson(server_admin_contract::AdminSignInReq);
#[derive(Debug, newtype::FromInner)]
pub(crate) struct AxumAdminJson<Value>(Value);
#[derive(Debug, newtype::FromInner)]
pub(crate) struct AxumAdminForm<Value>(Value);
#[derive(Debug, newtype::FromInner)]
pub(crate) struct AxumAdminPath<Value>(Value);
#[derive(Debug, newtype::FromInner)]
pub(crate) struct AxumAdminQuery<Value>(Value);
#[derive(Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct AdminSessionPath(super::AdminSessionId);
impl<S> axum::extract::FromRequestParts<S> for HttpAdminHeaderMap
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self::from(parts.headers.clone())))
    }
}
impl axum::extract::FromRequestParts<StdSharedAdminAuthSvcState> for AdminAuthReq {
    type Rejection = AdminError;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &StdSharedAdminAuthSvcState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(
            parts
                .extensions
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|peer| Self {
                    headers: HttpAdminHeaderMap::from(parts.headers.clone()),
                    peer: AdminPeerAddr::from(super::StdAdminSocketAddr::from(peer.0)),
                    state: state.clone(),
                })
                .ok_or(AdminError::Authentication),
        )
    }
}
impl<S> axum::extract::FromRequest<S> for AdminSignInJson
where
    S: Send + Sync,
{
    type Rejection = AdminError;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        axum::Json::<server_admin_contract::AdminSignInReq>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
            .map_err(|error| {
                if error.status() == http::StatusCode::PAYLOAD_TOO_LARGE {
                    AdminError::PayloadTooLarge
                } else {
                    AdminError::Validation
                }
            })
    }
}
impl<S, Value> axum::extract::FromRequest<S> for AxumAdminJson<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = AdminError;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        axum::Json::<Value>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
            .map_err(|error| {
                if error.status() == http::StatusCode::PAYLOAD_TOO_LARGE {
                    AdminError::PayloadTooLarge
                } else {
                    AdminError::Validation
                }
            })
    }
}
impl<S, Value> axum::extract::FromRequest<S> for AxumAdminForm<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = AdminError;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        axum::Form::<Value>::from_request(req, state)
            .await
            .map(|axum::Form(value)| Self::from(value))
            .map_err(|error| {
                if error.status() == http::StatusCode::PAYLOAD_TOO_LARGE {
                    AdminError::PayloadTooLarge
                } else {
                    AdminError::Validation
                }
            })
    }
}
impl<S, Value> axum::extract::FromRequestParts<S> for AxumAdminPath<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = AdminError;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Path::<Value>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Path(value)| Self::from(value))
            .map_err(|_error| AdminError::Validation)
    }
}
impl<S, Value> axum::extract::FromRequestParts<S> for AxumAdminQuery<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = AdminError;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Query::<Value>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Query(value)| Self::from(value))
            .map_err(|_error| AdminError::Validation)
    }
}
impl axum::extract::FromRequestParts<StdSharedAdminAuthSvcState> for AdminSessionPath {
    type Rejection = AdminError;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &StdSharedAdminAuthSvcState,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Path::<uuid::Uuid>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Path(value)| {
                Self::from(super::AdminSessionId::from(super::UuidAdminValue::from(
                    value,
                )))
            })
            .map_err(|_error| AdminError::Validation)
    }
}
fn session_context_hash(
    headers: super::HttpAdminHeaderMapRef<'_>,
    peer: AdminPeerAddr,
) -> super::AdminTokenHash {
    let mut context = String::with_capacity(352usize);
    context.push_str(str_constants::CLIENT_ADDRESS);
    let client_address = peer.0.as_ref().ip().to_string();
    context.extend(client_address.chars().take(256usize));
    context.push_str(str_constants::USER_AGENT);
    let user_agent = headers
        .0
        .get(http::header::USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|candidate| !candidate.is_empty() && candidate.len() <= 8_192usize);
    match user_agent {
        Some(normalized_user_agent) => {
            context.extend(normalized_user_agent.chars().take(256usize));
        }
        None => context.push_str(str_constants::UNKNOWN_USER_AGENT),
    }
    super::hash_opaque_token(&super::AdminOpaqueToken::new(
        super::SecrecyAdminString::from(secrecy::SecretBox::new(Box::new(context))),
    ))
}
fn hash_refresh_token_with_context(
    token: &super::AdminOpaqueToken,
    context_hash: &super::AdminTokenHash,
) -> super::AdminTokenHash {
    let token_text = secrecy::ExposeSecret::expose_secret(token.0.as_ref());
    let context_hash_text = secrecy::ExposeSecret::expose_secret(context_hash.0.as_ref());
    let mut token_with_context =
        String::with_capacity(token_text.len().saturating_add(context_hash_text.len()));
    token_with_context.push_str(token_text);
    token_with_context.push_str(context_hash_text);
    super::hash_opaque_token(&super::AdminOpaqueToken::new(
        super::SecrecyAdminString::from(secrecy::SecretBox::new(Box::new(token_with_context))),
    ))
}
#[allow(clippy::single_call_fn)] // CSRF origin validation stays isolated from token validation
fn origin_is_present_and_allowed(
    state: &AdminAuthSvcState,
    headers: super::HttpAdminHeaderMapRef<'_>,
) -> super::StdAdminBool {
    super::StdAdminBool::from(bool::from(server_runtime::request_origin_allowed(
        server_runtime::HttpOriginHeadersRef::from(headers.0),
        &state.allowed_origins,
    )))
}
async fn authenticate(
    state: &AdminAuthSvcState,
    headers: super::HttpAdminHeaderMapRef<'_>,
    peer: AdminPeerAddr,
) -> Result<AuthenticatedAdmin, AdminError> {
    let token = super::find_admin_cookie(headers, super::AdminCookieKind::Access)
        .ok_or(AdminError::Authentication)?;
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[state.issuer.as_ref()]);
    validation.set_audience(&[state.audience.as_ref()]);
    let claims = state
        .decoding_keys
        .as_ref()
        .iter()
        .find_map(|decoding_key| {
            jsonwebtoken::decode::<super::AdminAccessClaims>(
                token.as_ref(),
                decoding_key,
                &validation,
            )
            .ok()
            .map(|data| data.claims)
        })
        .ok_or(AdminError::Authentication)?;
    let context_hash = session_context_hash(headers, peer);
    let active = super::repository::sessions::access_session_is_active(
        super::repository::SqlxAdminRepositoryPoolRef::from(state.pool.as_ref()),
        claims.session_id(),
        claims.user_id(),
        &context_hash,
    )
    .await
    .map_err(AdminError::pg)?;
    if !active.get() {
        return Err(AdminError::Authentication);
    }
    load_authenticated_admin(state, claims.user_id(), claims.session_id()).await
}
async fn validate_csrf(
    state: &AdminAuthSvcState,
    headers: super::HttpAdminHeaderMapRef<'_>,
    authenticated: &AuthenticatedAdmin,
) -> Result<(), AdminError> {
    if !origin_is_present_and_allowed(state, headers).get() {
        return Err(AdminError::Csrf);
    }
    let provided = headers
        .0
        .get(http::HeaderName::from_static(
            str_constants::X_CSRF_TOKEN_ALT,
        ))
        .and_then(|value| value.to_str().ok())
        .ok_or(AdminError::Csrf)?;
    let provided_token = super::AdminOpaqueToken::new(super::SecrecyAdminString::from(
        secrecy::SecretBox::new(Box::new(provided.to_owned())),
    ));
    let provided_hash = super::hash_opaque_token(&provided_token);
    let expected = super::repository::sessions::read_csrf_hash(
        super::repository::SqlxAdminRepositoryPoolRef::from(state.pool.as_ref()),
        authenticated.session_id,
        authenticated.id,
    )
    .await
    .map_err(AdminError::pg)?
    .ok_or(AdminError::Csrf)?;
    if provided_hash.expose().as_ref() != expected.expose().as_ref() {
        return Err(AdminError::Csrf);
    }
    Ok(())
}
pub(crate) async fn authorize_generated_request(
    state: &AdminAuthSvcState,
    headers: super::HttpAdminHeaderMapRef<'_>,
    peer: AdminPeerAddr,
    permission: server_admin_contract::AdminPermissionStrRef<'_>,
    mutates: super::StdAdminBool,
) -> Result<AuthenticatedAdmin, AdminError> {
    let authenticated = authenticate(state, headers, peer).await?;
    let required_permission = super::AdminPermission::try_from(permission.as_ref())
        .map_err(|_error| AdminError::Authorization)?;
    if !authenticated
        .permissions
        .as_ref()
        .contains(&required_permission)
    {
        return Err(AdminError::Authorization);
    }
    if mutates.get() {
        let subject = super::StdAdminString::try_from(authenticated.id.get().to_string())
            .map_err(|_error| AdminError::Validation)?;
        rate_limit::enforce_rate_limit(
            state,
            rate_limit::AdminRateLimitScope::Mutation,
            &subject,
            state.policy.mutation_limit,
            state.policy.mutation_window,
        )
        .await?;
        validate_csrf(state, headers, &authenticated).await?;
    }
    Ok(authenticated)
}
#[derive(newtype::DebugTransparent, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
pub struct HttpAdminHeaderValueError(http::header::InvalidHeaderValue);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AdminObservedErrorCode {
    Database,
    Header,
    PasswordHash,
    Session,
}
impl AdminObservedErrorCode {
    const fn get(self) -> &'static str {
        match self {
            Self::Database => str_constants::ADMIN_OBSERVED_ERROR_DATABASE,
            Self::Header => str_constants::ADMIN_OBSERVED_ERROR_RESPONSE_HEADER,
            Self::PasswordHash => str_constants::ADMIN_OBSERVED_ERROR_PASSWORD_HASH,
            Self::Session => str_constants::ADMIN_OBSERVED_ERROR_SESSION,
        }
    }
}
#[derive(Debug, thiserror::Error)]
pub(crate) enum AdminError {
    #[error("administrator authentication failed")]
    Authentication,
    #[error("administrator authorization failed")]
    Authorization,
    #[error("administrator operation conflicts with current state")]
    Conflict,
    #[error("administrator request failed CSRF validation")]
    Csrf,
    #[error("administrator authentication is temporarily rate limited")]
    RateLimited,
    #[error("administrator request validation failed")]
    Validation,
    #[error("administrator API database operation failed: {0:?}")]
    Pg(#[source] server_runtime::ObservedError<super::SqlxAdminError>),
    #[error("administrator password hashing failed: {0}")]
    PasswordHash(#[source] server_runtime::ObservedError<super::AdminPasswordHashError>),
    #[error("administrator request body is too large")]
    PayloadTooLarge,
    #[error("administrator route does not support this HTTP method")]
    MethodNotAllowed,
    #[error("administrator session operation failed: {0}")]
    Session(#[source] server_runtime::ObservedError<AdminSessionError>),
    #[error("administrator response header is invalid: {0:?}")]
    Header(#[source] server_runtime::ObservedError<HttpAdminHeaderValueError>),
}
impl AdminError {
    const fn route_error_status(&self) -> frontend_contract::RouteErrorStatus {
        match self {
            Self::Authentication => frontend_contract::RouteErrorStatus::Authentication,
            Self::Authorization | Self::Csrf => frontend_contract::RouteErrorStatus::Authorization,
            Self::Conflict => frontend_contract::RouteErrorStatus::Conflict,
            Self::MethodNotAllowed => frontend_contract::RouteErrorStatus::MethodNotAllowed,
            Self::PayloadTooLarge => frontend_contract::RouteErrorStatus::PayloadTooLarge,
            Self::RateLimited => frontend_contract::RouteErrorStatus::RateLimited,
            Self::Validation => frontend_contract::RouteErrorStatus::Validation,
            Self::Pg(_) | Self::PasswordHash(_) | Self::Session(_) | Self::Header(_) => {
                frontend_contract::RouteErrorStatus::Internal
            }
        }
    }

    #[track_caller]
    fn header(source: HttpAdminHeaderValueError) -> Self {
        Self::Header(server_runtime::ObservedError::capture(
            source,
            server_runtime::ObservedErrorCode::from(AdminObservedErrorCode::Header.get()),
        ))
    }

    #[track_caller]
    fn password_hash(source: super::AdminPasswordHashError) -> Self {
        Self::PasswordHash(server_runtime::ObservedError::capture(
            source,
            server_runtime::ObservedErrorCode::from(AdminObservedErrorCode::PasswordHash.get()),
        ))
    }

    #[track_caller]
    fn pg(source: super::SqlxAdminError) -> Self {
        Self::Pg(server_runtime::ObservedError::capture(
            source,
            server_runtime::ObservedErrorCode::from(AdminObservedErrorCode::Database.get()),
        ))
    }

    #[track_caller]
    fn session(source: AdminSessionError) -> Self {
        Self::Session(server_runtime::ObservedError::capture(
            source,
            server_runtime::ObservedErrorCode::from(AdminObservedErrorCode::Session.get()),
        ))
    }
}
impl From<sqlx::Error> for AdminError {
    fn from(value: sqlx::Error) -> Self {
        Self::pg(super::SqlxAdminError::from(value))
    }
}
impl From<super::SqlxAdminError> for AdminError {
    fn from(value: super::SqlxAdminError) -> Self {
        Self::pg(value)
    }
}
#[derive(Debug, newtype::IntoInnerFrom, newtype::FromInner)]
pub struct AxumAdminResponse(axum::response::Response);
impl axum::response::IntoResponse for AdminError {
    fn into_response(self) -> axum::response::Response {
        admin_error_response(&self)
    }
}
fn admin_error_response(error: &AdminError) -> axum::response::Response {
    let route_error_status = error.route_error_status();
    let error_type = server_runtime::HttpErrorType::from(str_constants::ADMIN_API_ERROR_TYPE);
    let optional_diagnostic = match &error {
        AdminError::Pg(source) => Some(server_runtime::HttpErrorDiagnostic::from_observed(
            error_type, source,
        )),
        AdminError::PasswordHash(source) => Some(
            server_runtime::HttpErrorDiagnostic::from_observed(error_type, source),
        ),
        AdminError::Session(source) => Some(server_runtime::HttpErrorDiagnostic::from_observed(
            error_type, source,
        )),
        AdminError::Header(source) => Some(server_runtime::HttpErrorDiagnostic::from_observed(
            error_type, source,
        )),
        AdminError::Authentication
        | AdminError::Authorization
        | AdminError::Conflict
        | AdminError::Csrf
        | AdminError::MethodNotAllowed
        | AdminError::PayloadTooLarge
        | AdminError::RateLimited
        | AdminError::Validation => None,
    };
    let problem_status = frontend_contract::ApiProblemStatus::try_from(u16::from(
        route_error_status.transport_status(),
    ))
    .unwrap_or_else(|_error| {
        frontend_contract::ApiProblemStatus::from(
            frontend_contract::KnownHttpStatus::InternalServerError,
        )
    });
    let mut response = axum::response::IntoResponse::into_response(
        frontend_contract::ApiProblemError::from_status(problem_status),
    );
    if let Some(diagnostic) = optional_diagnostic {
        let _previous_diagnostic = response.extensions_mut().insert(diagnostic);
    }
    response
}
frontend_contract::api_operation_error!(AdminAuditLogError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminAuditExportError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminBrandingError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(
    AdminChangeOwnPasswordError,
    AdminError,
    admin_error_response
);
frontend_contract::api_operation_error!(AdminCreateRoleError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminCreateUserError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminDataTableError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminDataTablesError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminDeleteRoleError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminDeleteUserError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(
    AdminListPermissionsError,
    AdminError,
    admin_error_response
);
frontend_contract::api_operation_error!(AdminListRolesError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminListUsersError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminMeError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminRefreshError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(
    AdminRevokeAllSessionsError,
    AdminError,
    admin_error_response
);
frontend_contract::api_operation_error!(AdminRevokeSessionError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminSessionsError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(
    AdminSetRolePermissionsError,
    AdminError,
    admin_error_response
);
frontend_contract::api_operation_error!(AdminSetUserBanError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(
    AdminSetUserPasswordError,
    AdminError,
    admin_error_response
);
frontend_contract::api_operation_error!(AdminSetUserRolesError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminSettingsError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminSignInError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminSignOutError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminUpdateRoleError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminUpdateSettingsError, AdminError, admin_error_response);
frontend_contract::api_operation_error!(AdminUpdateUserError, AdminError, admin_error_response);
impl axum::response::IntoResponse for AxumAdminResponse {
    fn into_response(self) -> axum::response::Response {
        self.0
    }
}
async fn record_login_attempt(
    state: &AdminAuthSvcState,
    login: &super::AdminLogin,
    peer: AdminPeerAddr,
    succeeded: super::StdAdminBool,
) -> Result<(), AdminError> {
    super::repository::audit::record_login_attempt(
        super::repository::SqlxAdminRepositoryPoolRef::from(state.pool.as_ref()),
        login,
        peer,
        succeeded,
        super::UuidAdminValue::from(uuid::Uuid::new_v4()),
    )
    .await
    .map_err(AdminError::pg)
}
#[derive(Debug, Clone, Copy)]
struct AdminAuditSuccessRef<'value_lt> {
    action: super::AdminAuditAction,
    login: &'value_lt super::AdminLogin,
    resource: super::AdminAuditResource,
    resource_id: AdminAuditResourceId,
    user_id: super::AdminUserId,
}
#[derive(Debug, Clone, Copy)]
enum AdminAuditResourceId {
    Role(super::AdminRoleId),
    Session(super::AdminSessionId),
    SystemSettings,
    User(super::AdminUserId),
}
impl AdminAuditResourceId {
    fn value(self) -> super::StdAdminString {
        super::StdAdminString::from(match self {
            Self::User(value) => super::domain::AdminAuditResourceValue::User(value),
            Self::Role(value) => super::domain::AdminAuditResourceValue::Role(value),
            Self::Session(value) => super::domain::AdminAuditResourceValue::Session(value),
            Self::SystemSettings => super::domain::AdminAuditResourceValue::SystemSettings,
        })
    }
}
async fn record_audit_success_in_connection(
    connection: SqlxAdminPgConnectionRef<'_>,
    event: AdminAuditSuccessRef<'_>,
) -> Result<(), AdminError> {
    audit::record_success_in_connection(connection, event).await
}
#[derive(newtype::AsMut, newtype::FromInner)]
struct SqlxAdminPgConnectionRef<'connection_lt>(&'connection_lt mut sqlx::PgConnection);

async fn load_authenticated_admin(
    state: &AdminAuthSvcState,
    user_id: super::AdminUserId,
    session_id: super::AdminSessionId,
) -> Result<AuthenticatedAdmin, AdminError> {
    let mut db = super::repository::AdminRepositoryDbRef::Pool(
        super::repository::SqlxAdminRepositoryPoolRef::from(state.pool.as_ref()),
    );
    load_authenticated_admin_from_db(&mut db, user_id, session_id).await
}
async fn load_authenticated_admin_from_db(
    db: &mut super::repository::AdminRepositoryDbRef<'_, '_>,
    user_id: super::AdminUserId,
    session_id: super::AdminSessionId,
) -> Result<AuthenticatedAdmin, AdminError> {
    let record = super::repository::users::read_authenticated_record(db, user_id)
        .await
        .map_err(|repository_error| match repository_error {
            super::repository::AdminRepositoryError::InvalidStoredValue => {
                AdminError::Authentication
            }
            super::repository::AdminRepositoryError::Sqlx(sqlx_error) => AdminError::pg(sqlx_error),
        })?
        .ok_or(AdminError::Authentication)?;
    let (display_name, login, permissions, roles) = record.into_parts();
    Ok(AuthenticatedAdmin {
        display_name,
        id: user_id,
        login,
        permissions,
        roles,
        session_id,
    })
}
#[allow(clippy::single_call_fn)] // sign-in alone creates the long-lived refresh cookie
fn append_session_cookies(
    response: &mut AxumAdminResponse,
    state: &AdminAuthSvcState,
    session: &AdminSessionBundle,
) -> Result<(), AdminError> {
    append_access_session_cookies(response, state, session)?;
    let refresh = super::build_admin_cookie(
        super::AdminCookieKind::Refresh,
        session.refresh_token.expose(),
        super::AdminCookieMaxAgeSeconds::from(state.refresh_ttl.0),
        state.cookie_secure,
    );
    http::HeaderValue::from_str(refresh.as_ref())
        .map(|header| {
            response
                .0
                .headers_mut()
                .append(http::header::SET_COOKIE, header)
        })
        .map(drop)
        .map_err(|error| AdminError::header(HttpAdminHeaderValueError::from(error)))
}
fn append_access_session_cookies(
    response: &mut AxumAdminResponse,
    state: &AdminAuthSvcState,
    session: &AdminSessionBundle,
) -> Result<(), AdminError> {
    let access = super::build_admin_cookie(
        super::AdminCookieKind::Access,
        super::StdAdminStrRef::from(session.access_token.as_ref().as_str()),
        super::AdminCookieMaxAgeSeconds::from(state.access_ttl.0),
        state.cookie_secure,
    );
    let csrf = super::build_admin_cookie(
        super::AdminCookieKind::Csrf,
        super::StdAdminStrRef::from(
            secrecy::ExposeSecret::expose_secret(session.csrf_token.0.as_ref()).as_str(),
        ),
        super::AdminCookieMaxAgeSeconds::from(state.access_ttl.0),
        state.cookie_secure,
    );
    [access, csrf].into_iter().try_for_each(|cookie| {
        http::HeaderValue::from_str(cookie.as_ref())
            .map(|header| {
                response
                    .0
                    .headers_mut()
                    .append(http::header::SET_COOKIE, header)
            })
            .map(drop)
            .map_err(|error| AdminError::header(HttpAdminHeaderValueError::from(error)))
    })
}
fn append_cleared_session_cookies(
    response: &mut AxumAdminResponse,
    state: &AdminAuthSvcState,
) -> Result<(), AdminError> {
    [
        super::AdminCookieKind::Access,
        super::AdminCookieKind::Refresh,
        super::AdminCookieKind::Csrf,
    ]
    .into_iter()
    .try_for_each(|kind| {
        let cookie = super::clear_admin_cookie(kind, state.cookie_secure);
        http::HeaderValue::from_str(cookie.as_ref())
            .map(|header| {
                response
                    .0
                    .headers_mut()
                    .append(http::header::SET_COOKIE, header)
            })
            .map(drop)
            .map_err(|error| AdminError::header(HttpAdminHeaderValueError::from(error)))
    })
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_auth")]
async fn sign_in(
    auth: AdminAuthReq,
    peer: AdminPeerAddr,
    request_json: AdminSignInJson,
) -> Result<AxumAdminResponse, AdminSignInError> {
    handlers::sign_in(auth, peer, request_json)
        .await
        .map_err(AdminSignInError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_auth")]
async fn me(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminMeError> {
    handlers::me(auth).await.map_err(AdminMeError::from)
}
#[allow(clippy::single_call_fn)]
#[frontend_contract::route_openapi(tag = "admin_auth")]
async fn change_own_password(
    auth: AdminAuthReq,
    request: AxumAdminJson<server_admin_contract::AdminChangeOwnPasswordReq>,
) -> Result<AxumAdminResponse, AdminChangeOwnPasswordError> {
    handlers::change_own_password(auth, request)
        .await
        .map_err(AdminChangeOwnPasswordError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_auth")]
async fn refresh(
    auth: AdminAuthReq,
    peer: AdminPeerAddr,
) -> Result<AxumAdminResponse, AdminRefreshError> {
    handlers::refresh(auth, peer)
        .await
        .map_err(AdminRefreshError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_auth")]
async fn sign_out(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminSignOutError> {
    handlers::sign_out(auth)
        .await
        .map_err(AdminSignOutError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(
    params(server_admin_contract::AdminTableQuery),
    tag = "admin_auth"
)]
async fn sessions(
    auth: AdminAuthReq,
    query: AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<AxumAdminResponse, AdminSessionsError> {
    handlers::sessions(auth, query)
        .await
        .map_err(AdminSessionsError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_auth")]
async fn revoke_session(
    auth: AdminAuthReq,
    session: AdminSessionPath,
) -> Result<AxumAdminResponse, AdminRevokeSessionError> {
    handlers::revoke_session(auth, session)
        .await
        .map_err(AdminRevokeSessionError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_auth")]
async fn revoke_all_sessions(
    auth: AdminAuthReq,
) -> Result<AxumAdminResponse, AdminRevokeAllSessionsError> {
    handlers::revoke_all_sessions(auth)
        .await
        .map_err(AdminRevokeAllSessionsError::from)
}
async fn authorize_custom(
    auth: &AdminAuthReq,
    permission: super::AdminPermission,
) -> Result<AuthenticatedAdmin, AdminError> {
    let authenticated = authorize_generated_request(
        auth.state.as_ref(),
        super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        permission.as_str(),
        super::StdAdminBool::from(true),
    )
    .await?;
    Ok(authenticated)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_users")]
async fn create_user(
    auth: AdminAuthReq,
    request: AxumAdminJson<server_admin_contract::AdminCreateUserReq>,
) -> Result<AxumAdminResponse, AdminCreateUserError> {
    handlers::create_user(auth, request)
        .await
        .map_err(AdminCreateUserError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_users")]
async fn update_user(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
    request: AxumAdminJson<server_admin_contract::AdminUpdateUserReq>,
) -> Result<AxumAdminResponse, AdminUpdateUserError> {
    handlers::update_user(auth, path, request)
        .await
        .map_err(AdminUpdateUserError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_users")]
async fn set_user_password(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
    request: AxumAdminJson<server_admin_contract::AdminSetUserPasswordReq>,
) -> Result<AxumAdminResponse, AdminSetUserPasswordError> {
    handlers::set_user_password(auth, path, request)
        .await
        .map_err(AdminSetUserPasswordError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_users")]
async fn set_user_ban(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
    request: AxumAdminJson<server_admin_contract::AdminSetUserBanReq>,
) -> Result<AxumAdminResponse, AdminSetUserBanError> {
    handlers::set_user_ban(auth, path, request)
        .await
        .map_err(AdminSetUserBanError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_users")]
async fn delete_user(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
) -> Result<AxumAdminResponse, AdminDeleteUserError> {
    handlers::delete_user(auth, path)
        .await
        .map_err(AdminDeleteUserError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_roles")]
async fn create_role(
    auth: AdminAuthReq,
    request: AxumAdminJson<server_admin_contract::AdminCreateRoleReq>,
) -> Result<AxumAdminResponse, AdminCreateRoleError> {
    handlers::create_role(auth, request)
        .await
        .map_err(AdminCreateRoleError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_roles")]
async fn update_role(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminRoleId>,
    request: AxumAdminJson<server_admin_contract::AdminUpdateRoleReq>,
) -> Result<AxumAdminResponse, AdminUpdateRoleError> {
    handlers::update_role(auth, path, request)
        .await
        .map_err(AdminUpdateRoleError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_roles")]
async fn delete_role(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminRoleId>,
) -> Result<AxumAdminResponse, AdminDeleteRoleError> {
    handlers::delete_role(auth, path)
        .await
        .map_err(AdminDeleteRoleError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_roles")]
async fn set_role_permissions(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminRoleId>,
    request: AxumAdminJson<server_admin_contract::AdminSetRolePermissionsReq>,
) -> Result<AxumAdminResponse, AdminSetRolePermissionsError> {
    handlers::set_role_permissions(auth, path, request)
        .await
        .map_err(AdminSetRolePermissionsError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_users")]
async fn set_user_roles(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
    request: AxumAdminJson<server_admin_contract::AdminSetUserRolesReq>,
) -> Result<AxumAdminResponse, AdminSetUserRolesError> {
    handlers::set_user_roles(auth, path, request)
        .await
        .map_err(AdminSetUserRolesError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(params(AdminAuditQuery), tag = "admin_audit")]
async fn audit_log(
    auth: AdminAuthReq,
    query: AxumAdminQuery<AdminAuditQuery>,
) -> Result<AxumAdminResponse, AdminAuditLogError> {
    audit::query_log(auth, query)
        .await
        .map_err(AdminAuditLogError::from)
}
#[allow(clippy::single_call_fn)]
#[frontend_contract::route_openapi(params(AdminAuditQuery), tag = "admin_audit")]
async fn export_audit_log(
    auth: AdminAuthReq,
    query: AxumAdminQuery<AdminAuditQuery>,
) -> Result<AxumAdminResponse, AdminAuditExportError> {
    audit::export_log(auth, query)
        .await
        .map_err(AdminAuditExportError::from)
}
#[allow(clippy::single_call_fn)]
#[frontend_contract::route_openapi(tag = "admin_settings")]
async fn branding(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminBrandingError> {
    handlers::branding(auth)
        .await
        .map_err(AdminBrandingError::from)
}
#[allow(clippy::single_call_fn)]
#[frontend_contract::route_openapi(tag = "admin_tables")]
async fn data_tables(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminDataTablesError> {
    handlers::data_tables(auth)
        .await
        .map_err(AdminDataTablesError::from)
}
#[allow(clippy::single_call_fn)]
#[frontend_contract::route_openapi(
    params(server_admin_contract::AdminDataTableQuery),
    tag = "admin_tables"
)]
async fn data_table(
    auth: AdminAuthReq,
    path: AxumAdminPath<server_admin_contract::AdminDataTable>,
    query: AxumAdminQuery<server_admin_contract::AdminDataTableQuery>,
) -> Result<AxumAdminResponse, AdminDataTableError> {
    handlers::data_table(auth, path, query)
        .await
        .map_err(AdminDataTableError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_settings")]
async fn update_settings(
    auth: AdminAuthReq,
    request: AxumAdminJson<server_admin_contract::AdminUpdateSettingsReq>,
) -> Result<AxumAdminResponse, AdminUpdateSettingsError> {
    handlers::update_settings(auth, request)
        .await
        .map_err(AdminUpdateSettingsError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(
    params(server_admin_contract::AdminTableQuery),
    tag = "admin_users"
)]
async fn list_users(
    auth: AdminAuthReq,
    query: AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<AxumAdminResponse, AdminListUsersError> {
    handlers::list_users(auth, query)
        .await
        .map_err(AdminListUsersError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(
    params(server_admin_contract::AdminTableQuery),
    tag = "admin_roles"
)]
async fn list_roles(
    auth: AdminAuthReq,
    query: AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<AxumAdminResponse, AdminListRolesError> {
    handlers::list_roles(auth, query)
        .await
        .map_err(AdminListRolesError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(
    params(server_admin_contract::AdminTableQuery),
    tag = "admin_roles"
)]
async fn list_permissions(
    auth: AdminAuthReq,
    query: AxumAdminQuery<server_admin_contract::AdminTableQuery>,
) -> Result<AxumAdminResponse, AdminListPermissionsError> {
    handlers::list_permissions(auth, query)
        .await
        .map_err(AdminListPermissionsError::from)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[frontend_contract::route_openapi(tag = "admin_settings")]
async fn settings(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminSettingsError> {
    handlers::settings(auth)
        .await
        .map_err(AdminSettingsError::from)
}
#[derive(Debug, Clone, newtype::IntoInnerFrom, newtype::FromInner)]
pub struct AxumAdminAuthRouter(axum::Router);
#[derive(Clone, newtype::IntoInnerFrom, newtype::FromInner)]
pub struct UtoipaAdminAuthOpenApi(utoipa::openapi::OpenApi);
impl std::fmt::Debug for UtoipaAdminAuthOpenApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(str_constants::UTOIPAADMINAUTHOPENAPI)
    }
}
#[must_use]
pub fn open_api() -> UtoipaAdminAuthOpenApi {
    routes::open_api()
}
#[must_use]
pub fn routes(state: StdSharedAdminAuthSvcState) -> AxumAdminAuthRouter {
    routes::routes(state)
}
#[must_use]
pub fn html_routes(state: StdSharedAdminAuthSvcState) -> AxumAdminAuthRouter {
    html::routes(state, AdminHtmlSwaggerEnabled::from(true))
}
#[derive(Clone, Copy, Debug, newtype::FromInner)]
pub struct AdminHtmlSwaggerEnabled(bool);
#[must_use]
pub fn html_routes_with_swagger(
    state: StdSharedAdminAuthSvcState,
    swagger_enabled: AdminHtmlSwaggerEnabled,
) -> AxumAdminAuthRouter {
    html::routes(state, swagger_enabled)
}
impl AdminAuthSvcState {
    pub fn try_new(
        pool: app_state::SqlxPgPool,
        jwt_secret: &config_lib::AdminJwtSecret,
        access_ttl: &config_lib::AdminAccessTokenTtlSeconds,
        refresh_ttl: &config_lib::AdminRefreshTokenTtlSeconds,
        session_limit: &config_lib::AdminSessionLimit,
        sign_in_rate_limit: &config_lib::AdminSignInRateLimit,
        password_hash_concurrency: &config_lib::AdminPasswordHashConcurrency,
        cookie_secure: &config_lib::AdminCookieSecure,
        issuer: &config_lib::AdminTokenIssuer,
        audience: &config_lib::AdminTokenAudience,
        allowed_origins: &config_lib::CorsAllowOrigin,
    ) -> Result<Self, AdminAuthSvcStateBuildError> {
        let secret = secrecy::ExposeSecret::expose_secret(
            jwt_secret
                .primary()
                .ok_or(AdminAuthSvcStateBuildError::JwtSecret)?
                .as_ref(),
        );
        let parsed_origins = allowed_origins
            .0
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
            .collect::<Vec<String>>();
        Ok(Self {
            access_ttl: StdAdminAccessTtlSeconds::try_from(access_ttl.get())
                .map_err(AdminAuthSvcStateBuildError::PositiveValue)?,
            allowed_origins: server_runtime::AllowedOrigins::try_from(parsed_origins)
                .map_err(|_error| AdminAuthSvcStateBuildError::AllowedOrigin)?,
            audience: audience.clone(),
            cookie_secure: super::AdminCookieSecure::from(**cookie_secure),
            decoding_keys: jwt_secret
                .verification_secrets()
                .iter()
                .map(|verification_secret| {
                    jsonwebtoken::DecodingKey::from_secret(
                        secrecy::ExposeSecret::expose_secret(verification_secret.as_ref())
                            .as_bytes(),
                    )
                })
                .collect::<Vec<_>>()
                .into(),
            encoding_key: JsonwebtokenAdminEncodingKey::from(
                jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
            ),
            issuer: issuer.clone(),
            password_hasher: super::AdminPasswordHasher::new(
                super::AdminPasswordHashConcurrency::from(super::StdAdminNonZeroUsize::from(
                    std::num::NonZeroUsize::new(password_hash_concurrency.get())
                        .ok_or(AdminAuthSvcStateBuildError::PasswordHashConcurrency)?,
                )),
            ),
            pool,
            refresh_ttl: StdAdminRefreshTtlSeconds::try_from(refresh_ttl.get())
                .map_err(AdminAuthSvcStateBuildError::PositiveValue)?,
            session_limit: StdAdminSessionLimit::try_from(session_limit.get())
                .map_err(AdminAuthSvcStateBuildError::PositiveValue)?,
            policy: AdminAuthPolicy::from_sign_in_limit(StdAdminRateLimitCount::from(
                i64::try_from(sign_in_rate_limit.get()).unwrap_or(i64::MAX),
            )),
        })
    }
}
#[derive(Debug)]
pub struct AdminSessionBundle {
    access_token: super::StdAdminAccessToken,
    csrf_token: super::AdminOpaqueToken,
    refresh_token: super::AdminRefreshToken,
    session_id: super::AdminSessionId,
}
impl AdminSessionBundle {
    #[must_use]
    pub const fn access_token(&self) -> &super::StdAdminAccessToken {
        &self.access_token
    }
    #[must_use]
    pub const fn csrf_token(&self) -> &super::AdminOpaqueToken {
        &self.csrf_token
    }
    #[must_use]
    pub const fn refresh_token(&self) -> &super::AdminRefreshToken {
        &self.refresh_token
    }
    #[must_use]
    pub const fn session_id(&self) -> super::AdminSessionId {
        self.session_id
    }
}
#[derive(Debug, thiserror::Error)]
pub enum AdminSessionError {
    #[error("administrator access token creation failed: {0:?}")]
    AccessToken(super::AdminAccessTokenError),
    #[error("administrator session database operation failed: {0:?}")]
    Pg(super::SqlxAdminError),
    #[error("system clock is before the Unix epoch")]
    SystemClock,
}
#[allow(clippy::single_call_fn)] // facade keeps session persistence private to the session module
async fn create_session_in_connection(
    state: &AdminAuthSvcState,
    user_id: super::AdminUserId,
    context_hash: &super::AdminTokenHash,
    connection: SqlxAdminPgConnectionRef<'_>,
) -> Result<AdminSessionBundle, AdminSessionError> {
    session::create_session_in_connection(state, user_id, context_hash, connection).await
}
#[allow(clippy::single_call_fn)] // facade keeps refreshed-session persistence private to the session module
async fn create_refreshed_session_in_connection(
    state: &AdminAuthSvcState,
    user_id: super::AdminUserId,
    context_hash: &super::AdminTokenHash,
    refresh_token: super::AdminRefreshToken,
    connection: SqlxAdminPgConnectionRef<'_>,
) -> Result<AdminSessionBundle, AdminSessionError> {
    session::create_refreshed_session_in_connection(
        state,
        user_id,
        context_hash,
        refresh_token,
        connection,
    )
    .await
}
#[cfg(test)]
mod tests {
    #[test]
    fn rate_limit_scopes_are_distinct() {
        let scopes = [
            super::rate_limit::AdminRateLimitScope::AuditExport,
            super::rate_limit::AdminRateLimitScope::Mutation,
            super::rate_limit::AdminRateLimitScope::RefreshIp,
            super::rate_limit::AdminRateLimitScope::SignInIp,
            super::rate_limit::AdminRateLimitScope::SignInIpLogin,
        ]
        .map(super::rate_limit::AdminRateLimitScope::as_str);
        assert_eq!(
            scopes[0].as_ref(),
            str_constants::SERVER_ADMIN_RATE_LIMIT_AUDIT_EXPORT
        );
        let unique = scopes.into_iter().collect::<std::collections::HashSet<_>>();
        assert_eq!(unique.len(), 5usize);
    }
    #[test]
    fn rate_limited_error_includes_retry_after_header() {
        let response = axum::response::IntoResponse::into_response(super::AdminError::RateLimited);
        assert_eq!(response.status(), http::StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(
            response.headers().get(http::header::RETRY_AFTER),
            Some(&http::HeaderValue::from_static("60")),
        );
        assert!(
            response
                .extensions()
                .get::<server_runtime::HttpErrorDiagnostic>()
                .is_none()
        );
    }
    #[test]
    fn server_error_response_preserves_http_diagnostic() {
        let response = axum::response::IntoResponse::into_response(super::AdminError::pg(
            super::super::SqlxAdminError::from(sqlx::Error::RowNotFound),
        ));
        assert_eq!(response.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
        assert!(
            response
                .extensions()
                .get::<server_runtime::HttpErrorDiagnostic>()
                .is_some()
        );
        assert_eq!(
            response.headers().get(http::header::CONTENT_TYPE),
            Some(&http::HeaderValue::from_static(
                str_constants::APPLICATION_PROBLEM_PLUS_JSON
            ))
        );
        let body =
            futures::executor::block_on(axum::body::to_bytes(response.into_body(), 16_384usize))
                .expect("8770f4d3");
        let contract_problem =
            serde_json::from_slice::<frontend_contract::ApiProblem>(&body).expect("4f705ab8");
        assert_eq!(
            contract_problem.kind(),
            frontend_contract::ApiProblemKind::Internal
        );
        let problem = serde_json::from_slice::<serde_json::Value>(&body).expect("1e7ec09d");
        [
            "location",
            "error_location",
            "backtrace",
            "error_chain",
            "span_trace",
        ]
        .into_iter()
        .for_each(|private_field| {
            assert!(problem.get(private_field).is_none());
        });
    }
    #[test]
    fn session_context_hash_is_bound_to_peer_and_user_agent() {
        let mut first_headers = http::HeaderMap::new();
        let _previous_user_agent = first_headers.insert(
            http::header::USER_AGENT,
            http::HeaderValue::from_static(str_constants::ADMIN_CLIENT_1),
        );
        let first_peer = super::AdminPeerAddr::from(super::super::StdAdminSocketAddr::from(
            str_constants::VALUE_192_0_2_10_443
                .parse::<std::net::SocketAddr>()
                .expect("f133a4ca"),
        ));
        let same_context_hash = super::session_context_hash(
            super::super::HttpAdminHeaderMapRef::from(&first_headers),
            first_peer,
        );
        let repeated_context_hash = super::session_context_hash(
            super::super::HttpAdminHeaderMapRef::from(&first_headers),
            first_peer,
        );
        assert_eq!(
            secrecy::ExposeSecret::expose_secret(same_context_hash.0.as_ref()),
            secrecy::ExposeSecret::expose_secret(repeated_context_hash.0.as_ref()),
        );
        let other_peer = super::AdminPeerAddr::from(super::super::StdAdminSocketAddr::from(
            str_constants::VALUE_192_0_2_11_443
                .parse::<std::net::SocketAddr>()
                .expect("5a831a2f"),
        ));
        let other_peer_hash = super::session_context_hash(
            super::super::HttpAdminHeaderMapRef::from(&first_headers),
            other_peer,
        );
        assert_ne!(
            secrecy::ExposeSecret::expose_secret(same_context_hash.0.as_ref()),
            secrecy::ExposeSecret::expose_secret(other_peer_hash.0.as_ref()),
        );
        let mut other_headers = http::HeaderMap::new();
        let _previous_other_user_agent = other_headers.insert(
            http::header::USER_AGENT,
            http::HeaderValue::from_static(str_constants::ADMIN_CLIENT_2),
        );
        let other_user_agent_hash = super::session_context_hash(
            super::super::HttpAdminHeaderMapRef::from(&other_headers),
            first_peer,
        );
        assert_ne!(
            secrecy::ExposeSecret::expose_secret(same_context_hash.0.as_ref()),
            secrecy::ExposeSecret::expose_secret(other_user_agent_hash.0.as_ref()),
        );
    }
    #[test]
    fn audit_resource_identifier_uses_target_identifier() {
        assert_eq!(
            super::AdminAuditResourceId::User(
                crate::AdminUserId::try_from(42i64).expect("423b91b9"),
            )
            .value()
            .as_ref(),
            "42"
        );
        assert_eq!(
            super::AdminAuditResourceId::Role(
                crate::AdminRoleId::try_from(7i64).expect("af8df9d2"),
            )
            .value()
            .as_ref(),
            "7"
        );
        assert_eq!(
            super::AdminAuditResourceId::SystemSettings.value().as_ref(),
            "1"
        );
    }
    #[test]
    fn open_api_contains_auth_and_user_security_contracts() {
        frontend_contract::validate_openapi_schema_references(&utoipa::openapi::OpenApi::from(
            super::open_api(),
        ))
        .expect("2151641d");
        let document = serde_json::to_value(utoipa::openapi::OpenApi::from(super::open_api()))
            .expect("869d28d7");
        let paths = document
            .get(str_constants::PATHS)
            .and_then(serde_json::Value::as_object)
            .expect("6e15edec");
        assert_eq!(paths.len(), 22usize);
        assert!(!paths.contains_key("/auth/mfa"));
        assert!(!paths.contains_key("/auth/mfa/enroll"));
        assert!(!paths.contains_key("/auth/mfa/confirm"));
        assert!(!paths.contains_key("/auth/mfa/step-up"));
        let documented_route_contracts = paths
            .iter()
            .flat_map(|(path, path_item)| {
                path_item
                    .as_object()
                    .into_iter()
                    .flat_map(|operation_map| operation_map.iter())
                    .map(move |(method, operation)| {
                        (
                            method.to_owned(),
                            operation
                                .get(str_constants::OPERATION_ID_JSON)
                                .and_then(serde_json::Value::as_str)
                                .expect("4252acc8")
                                .to_owned(),
                            path.to_owned(),
                        )
                    })
            })
            .collect::<std::collections::BTreeSet<_>>();
        let contracted_route_contracts = <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::coverage_descriptors()
            .as_ref()
            .iter()
            .copied()
            .map(|descriptor| {
                let metadata = descriptor.metadata();
                (
                    metadata.method().as_ref().to_ascii_lowercase(),
                    metadata.openapi_operation_id().as_ref().to_owned(),
                    metadata.path().as_ref().to_owned(),
                )
            })
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(documented_route_contracts, contracted_route_contracts);
        assert!(paths.contains_key("/auth/sign_in"));
        assert!(paths.contains_key("/auth/sessions/{session_id}"));
        assert!(paths.contains_key("/users/{user_id}/password"));
        assert!(paths.contains_key("/roles/{role_id}/permissions"));
        assert!(paths.contains_key("/permissions"));
        assert!(paths.contains_key("/audit_log"));
        assert!(paths.contains_key("/system_settings"));
        assert_eq!(
            document
                .pointer(str_constants::ADMIN_OPENAPI_SIGN_IN_OPERATION_ID_POINTER)
                .and_then(serde_json::Value::as_str),
            Some(<server_admin_contract::AdminSignInRoute as frontend_contract::TypedRoute>::metadata().openapi_operation_id().as_ref()),
        );
        assert_eq!(
            document
                .pointer(str_constants::ADMIN_OPENAPI_REFRESH_OPERATION_ID_POINTER)
                .and_then(serde_json::Value::as_str),
            Some(<server_admin_contract::AdminRefreshRoute as frontend_contract::TypedRoute>::metadata().openapi_operation_id().as_ref()),
        );
        assert_eq!(
            document
                .pointer(str_constants::ADMIN_OPENAPI_ME_OPERATION_ID_POINTER)
                .and_then(serde_json::Value::as_str),
            Some(
                <server_admin_contract::AdminMeRoute as frontend_contract::TypedRoute>::metadata()
                    .openapi_operation_id()
                    .as_ref()
            ),
        );
        assert!(
            paths
                .values()
                .all(|path| path
                    .as_object()
                    .is_some_and(|operations| operations.values().all(|operation| operation
                        .pointer("/responses/429/headers/Retry-After")
                        .is_some())))
        );
        assert!(
            document
                .pointer("/components/securitySchemes/admin_cookie")
                .is_some()
        );
        assert!(
            document
                .pointer("/components/securitySchemes/admin_csrf")
                .is_some()
        );
        assert_eq!(
            document
                .pointer("/components/schemas/AdminPassword/writeOnly")
                .and_then(serde_json::Value::as_bool),
            Some(true),
        );
        let expected_body_limit_description = format!(
            "{}{}",
            str_constants::OPENAPI_REQUEST_BODY_MAXIMUM_BYTES_PREFIX,
            <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::body_limit()
                .expect("be105d90")
                .get()
        );
        let request_body_descriptions = paths
            .values()
            .filter_map(|path| path.as_object())
            .flat_map(|operations| operations.values())
            .filter_map(|operation| {
                operation.pointer(str_constants::OPENAPI_REQUEST_BODY_DESCRIPTION_POINTER)
            })
            .filter_map(serde_json::Value::as_str)
            .collect::<Vec<_>>();
        assert!(!request_body_descriptions.is_empty());
        assert!(
            request_body_descriptions
                .into_iter()
                .all(|description| description == expected_body_limit_description)
        );
    }
}
mod audit;
mod handlers;
mod rate_limit;
mod routes;
mod session;
