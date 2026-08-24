#![allow(clippy::needless_for_each)] // utoipa 4 generated OpenAPI registration uses iterator callbacks
mod account;
mod api;
mod authn;
mod data_tables;
mod html;
mod roles;
mod sessions;
mod settings;
mod shared;
mod users;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, newtype::DebugTransparent, newtype::FromInner,
)]
pub struct JsonwebtokenAdminEncodingKey(jsonwebtoken::EncodingKey);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::AsRefTarget, newtype::FromInner,
)]
struct JsonwebtokenAdminDecodingKeys(Vec<jsonwebtoken::DecodingKey>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::IntoInnerFrom,
    newtype::TryFrom,
)]
#[try_from(
    error = AdminAuthPositiveValueError,
    validator = StdAdminAccessTtlSeconds::validate
)]
pub struct StdAdminAccessTtlSeconds(u64);
impl StdAdminAccessTtlSeconds {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &u64) -> Result<(), AdminAuthPositiveValueError> {
        if *value == constants_u64::ZERO {
            Err(AdminAuthPositiveValueError)
        } else {
            Ok(())
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::IntoInnerFrom,
    newtype::TryFrom,
)]
#[try_from(
    error = AdminAuthPositiveValueError,
    validator = StdAdminRefreshTtlSeconds::validate
)]
pub struct StdAdminRefreshTtlSeconds(u64);
impl StdAdminRefreshTtlSeconds {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &u64) -> Result<(), AdminAuthPositiveValueError> {
        if *value == constants_u64::ZERO {
            Err(AdminAuthPositiveValueError)
        } else {
            Ok(())
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::IntoInnerFrom,
    newtype::TryFrom,
)]
#[try_from(
    error = AdminAuthPositiveValueError,
    validator = StdAdminSessionLimit::validate
)]
pub struct StdAdminSessionLimit(usize);
impl StdAdminSessionLimit {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &usize) -> Result<(), AdminAuthPositiveValueError> {
        if *value == constants_usize::ZERO {
            Err(AdminAuthPositiveValueError)
        } else {
            Ok(())
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::IntoInnerFrom,
    newtype::TryFrom,
)]
#[try_from(
    error = AdminAuthPositiveValueError,
    validator = StdAdminFailureThreshold::validate
)]
pub struct StdAdminFailureThreshold(i64);
impl StdAdminFailureThreshold {
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &i64) -> Result<(), AdminAuthPositiveValueError> {
        if *value <= constants_i64::ZERO {
            Err(AdminAuthPositiveValueError)
        } else {
            Ok(())
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{self:?}")]
pub struct AdminAuthPositiveValueError;
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub struct StdAdminFailureDelayMillis(u64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct StdAdminRateLimitCount(i64);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct StdAdminRateLimitWindowSeconds(i32);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub struct AdminAuthPolicy {
    audit_export_limit: StdAdminRateLimitCount,
    failure_delay: StdAdminFailureDelayMillis,
    failure_threshold: StdAdminFailureThreshold,
    mutation_limit: StdAdminRateLimitCount,
    refresh_limit: StdAdminRateLimitCount,
    sign_in_ip_limit: StdAdminRateLimitCount,
    sign_in_limit: StdAdminRateLimitCount,
    audit_export_window: StdAdminRateLimitWindowSeconds,
    mutation_window: StdAdminRateLimitWindowSeconds,
    refresh_window: StdAdminRateLimitWindowSeconds,
    sign_in_window: StdAdminRateLimitWindowSeconds,
}
impl AdminAuthPolicy {
    #[allow(
        clippy::single_call_fn,
        reason = "keeps every administrator authentication threshold in one immutable policy constructor"
    )]
    fn from_limits(
        failure_threshold: StdAdminFailureThreshold,
        sign_in_limit: StdAdminRateLimitCount,
    ) -> Self {
        Self {
            audit_export_limit: StdAdminRateLimitCount::from(60i64),
            audit_export_window: StdAdminRateLimitWindowSeconds::from(60i32),
            failure_delay: StdAdminFailureDelayMillis::from(200u64),
            failure_threshold,
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
pub struct AdminAuthSvcState {
    access_ttl: StdAdminAccessTtlSeconds,
    allowed_origins: server_runtime_http::AllowedOrigins,
    audience: config_lib::AdminTokenAudience,
    decoding_keys: JsonwebtokenAdminDecodingKeys,
    encoding_key: JsonwebtokenAdminEncodingKey,
    issuer: config_lib::AdminTokenIssuer,
    password_hasher: super::AdminPasswordHasher,
    policy: AdminAuthPolicy,
    pool: app_state::SqlxPgPool,
    refresh_ttl: StdAdminRefreshTtlSeconds,
    session_limit: StdAdminSessionLimit,
    cookie_secure: super::AdminCookieSecure,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub struct StdSharedAdminAuthSvcState(std::sync::Arc<AdminAuthSvcState>);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, thiserror::Error)]
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
fn admin_password_from_contract(
    value: server_admin_contract::AdminPassword,
) -> Result<super::AdminPassword, super::AdminPasswordTryFromStringError> {
    super::AdminPassword::try_from(value.into_inner())
}
fn admin_new_password_from_contract(
    value: server_admin_contract::AdminNewPassword,
) -> Result<super::AdminPassword, super::AdminPasswordTryFromStringError> {
    super::AdminPassword::try_from(value.into_inner())
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct AuthenticatedAdmin {
    display_name: super::AdminDisplayName,
    id: super::AdminUserId,
    login: super::AdminLogin,
    permissions: super::AdminPermissions,
    roles: super::AdminRoleNames,
    session_id: super::AdminSessionId,
    #[schema(value_type = bool)]
    password_change_required: super::AdminPasswordChangeRequired,
}
impl AuthenticatedAdmin {
    #[must_use]
    pub const fn id(&self) -> super::AdminUserId {
        self.id
    }
    #[must_use]
    pub(crate) const fn password_change_required(&self) -> super::AdminPasswordChangeRequired {
        self.password_change_required
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, serde::Deserialize, utoipa::IntoParams,
)]
#[into_params(parameter_in = Query)]
pub struct AdminAuditQuery {
    created_after: Option<server_admin_contract::AdminAuditTimestamp>,
    created_before: Option<server_admin_contract::AdminAuditTimestamp>,
    cursor_created_at: Option<server_admin_contract::AdminAuditTimestamp>,
    cursor_id: Option<server_admin_contract::AdminAuditLogId>,
    resource_id: Option<server_admin_contract::AdminText>,
    #[param(inline)]
    user_id: Option<super::AdminUserId>,
    user_login: Option<server_admin_contract::AdminLogin>,
    #[serde(default)]
    #[param(value_type = u32)]
    offset: server_admin_contract::AdminPageOffset,
    #[serde(default)]
    #[param(value_type = u16, minimum = 1, maximum = 100)]
    limit: server_admin_contract::AdminPageLimit,
    #[param(inline)]
    resource: Option<super::AdminAuditResource>,
    succeeded: Option<server_admin_contract::AdminBool>,
    #[param(inline)]
    action: Option<super::AdminAuditAction>,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(clippy::field_scoped_visibility_modifiers)] // repository query binding consumes this internal cross-module DTO field-by-field
pub(crate) struct AdminAuditQueryParts {
    pub(crate) created_after: Option<server_admin_contract::AdminAuditTimestamp>,
    pub(crate) created_before: Option<server_admin_contract::AdminAuditTimestamp>,
    pub(crate) cursor_created_at: Option<server_admin_contract::AdminAuditTimestamp>,
    pub(crate) cursor_id: Option<server_admin_contract::AdminAuditLogId>,
    pub(crate) resource_id: Option<server_admin_contract::AdminText>,
    pub(crate) user_id: Option<super::AdminUserId>,
    pub(crate) user_login: Option<server_admin_contract::AdminLogin>,
    pub(crate) offset: server_admin_contract::AdminPageOffset,
    pub(crate) limit: server_admin_contract::AdminPageLimit,
    pub(crate) resource: Option<super::AdminAuditResource>,
    pub(crate) succeeded: Option<server_admin_contract::AdminBool>,
    pub(crate) action: Option<super::AdminAuditAction>,
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::AsRefOwned,
    newtype::FromInner,
)]
pub struct HttpAdminHeaderMap(http::HeaderMap);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone)]
pub(crate) struct AdminAuthReq {
    headers: HttpAdminHeaderMap,
    state: StdSharedAdminAuthSvcState,
    peer: AdminPeerAddr,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AdminSignInJson(server_admin_contract::AdminSignInReq);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumAdminJson<Value>(Value);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumAdminForm<Value>(Value);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumAdminPath<Value>(Value);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumAdminQuery<Value>(Value);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
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
) -> Result<super::AdminTokenHash, super::AdminSecretTextError> {
    let mut context = String::with_capacity(352usize);
    context.push_str(constants_str::CLIENT_ADDRESS);
    let client_address = peer.0.as_ref().ip().to_string();
    context.extend(client_address.chars().take(256usize));
    context.push_str(constants_str::USER_AGENT);
    let user_agent = headers
        .0
        .get(http::header::USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|candidate| {
            !candidate.is_empty() && candidate.len() <= constants_usize::VALUE_8_192
        });
    match user_agent {
        Some(normalized_user_agent) => {
            context.extend(normalized_user_agent.chars().take(256usize));
        }
        None => context.push_str(constants_str::UNKNOWN_USER_AGENT),
    }
    let token = super::SecrecyAdminString::try_from(context).map(super::AdminOpaqueToken::new)?;
    super::hash_opaque_token(&token)
}
fn hash_refresh_token_with_context(
    token: &super::AdminOpaqueToken,
    context_hash: &super::AdminTokenHash,
) -> Result<super::AdminTokenHash, super::AdminSecretTextError> {
    let token_text = secrecy::ExposeSecret::expose_secret(token.0.as_ref());
    let context_hash_text = secrecy::ExposeSecret::expose_secret(context_hash.0.as_ref());
    let mut token_with_context =
        String::with_capacity(token_text.len().saturating_add(context_hash_text.len()));
    token_with_context.push_str(token_text);
    token_with_context.push_str(context_hash_text);
    let combined_token = super::SecrecyAdminString::try_from(token_with_context)
        .map(super::AdminOpaqueToken::new)?;
    super::hash_opaque_token(&combined_token)
}
fn origin_is_present_and_allowed(
    state: &AdminAuthSvcState,
    headers: super::HttpAdminHeaderMapRef<'_>,
) -> super::StdAdminBool {
    super::StdAdminBool::from(bool::from(server_runtime_http::request_origin_allowed(
        server_runtime_http::HttpOriginHeadersRef::from(headers.0),
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
    let context_hash = session_context_hash(headers, peer).map_err(AdminError::secret_text)?;
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
            constants_str::X_CSRF_TOKEN_ALT,
        ))
        .and_then(|value| value.to_str().ok())
        .ok_or(AdminError::Csrf)?;
    let provided_token = super::SecrecyAdminString::try_from(provided.to_owned())
        .map(super::AdminOpaqueToken::new)
        .map_err(super::AdminSecretTextError::from)
        .map_err(AdminError::csrf_secret_text)?;
    let provided_hash =
        super::hash_opaque_token(&provided_token).map_err(AdminError::csrf_secret_text)?;
    let expected = super::repository::sessions::read_csrf_hash(
        super::repository::SqlxAdminRepositoryPoolRef::from(state.pool.as_ref()),
        authenticated.session_id,
        authenticated.id,
    )
    .await
    .map_err(AdminError::pg)?
    .ok_or(AdminError::Csrf)?;
    let provided_text = provided_hash.expose();
    let provided_secret = match server_runtime_http::SecretTextRef::try_from(provided_text.get()) {
        Ok(secret) => secret,
        Err(_error) => return Err(AdminError::Csrf),
    };
    let expected_text = expected.expose();
    let expected_secret = match server_runtime_http::SecretTextRef::try_from(expected_text.get()) {
        Ok(secret) => secret,
        Err(_error) => return Err(AdminError::Csrf),
    };
    if server_runtime_http::secret_texts_match(expected_secret, provided_secret)
        != server_runtime_http::SecretTextMatch::Equal
    {
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
    if *authenticated.password_change_required {
        return Err(AdminError::Authorization);
    }
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DebugTransparent,
    thiserror::Error,
    newtype::FromInner,
)]
#[error(transparent)]
pub struct HttpAdminHeaderValueError(http::header::InvalidHeaderValue);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, Eq)]
enum AdminObservedErrorCode {
    AuthenticationSecretText,
    CsrfSecretText,
    Database,
    Header,
    PasswordHash,
    PasswordText,
    SecretText,
    Session,
}
impl AdminObservedErrorCode {
    const fn get(self) -> &'static str {
        match self {
            Self::AuthenticationSecretText => constants_str::ADMIN_OBSERVED_ERROR_AUTH_SECRET_TEXT,
            Self::CsrfSecretText => constants_str::ADMIN_OBSERVED_ERROR_CSRF_SECRET_TEXT,
            Self::Database => constants_str::ADMIN_OBSERVED_ERROR_DATABASE,
            Self::Header => constants_str::ADMIN_OBSERVED_ERROR_RESPONSE_HEADER,
            Self::PasswordHash => constants_str::ADMIN_OBSERVED_ERROR_PASSWORD_HASH,
            Self::PasswordText => constants_str::ADMIN_OBSERVED_ERROR_PASSWORD_TEXT,
            Self::SecretText => constants_str::ADMIN_OBSERVED_ERROR_SECRET_TEXT,
            Self::Session => constants_str::ADMIN_OBSERVED_ERROR_SESSION,
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub(crate) enum AdminError {
    #[error("administrator authentication failed")]
    Authentication,
    #[error("administrator authentication secret text is invalid")]
    AuthenticationSecretText(
        #[source] server_runtime_http::ObservedError<super::AdminSecretTextError>,
    ),
    #[error("administrator authorization failed")]
    Authorization,
    #[error("administrator operation conflicts with current state")]
    Conflict,
    #[error("administrator request failed CSRF validation")]
    Csrf,
    #[error("administrator CSRF secret text is invalid")]
    CsrfSecretText(#[source] server_runtime_http::ObservedError<super::AdminSecretTextError>),
    #[error("administrator authentication is temporarily rate limited")]
    RateLimited,
    #[error("administrator request validation failed")]
    Validation,
    #[error("administrator API database operation failed: {0:?}")]
    Pg(#[source] server_runtime_http::ObservedError<super::SqlxAdminError>),
    #[error("administrator password hashing failed: {0}")]
    PasswordHash(#[source] server_runtime_http::ObservedError<super::AdminPasswordHashError>),
    #[error("administrator password text is invalid")]
    PasswordText(
        #[source] server_runtime_http::ObservedError<super::AdminPasswordTryFromStringError>,
    ),
    #[error("administrator request body is too large")]
    PayloadTooLarge,
    #[error("administrator secret text is invalid")]
    SecretText(#[source] server_runtime_http::ObservedError<super::AdminSecretTextError>),
    #[error("administrator route does not support this HTTP method")]
    MethodNotAllowed,
    #[error("administrator session operation failed: {0}")]
    Session(#[source] server_runtime_http::ObservedError<AdminSessionError>),
    #[error("administrator response header is invalid: {0:?}")]
    Header(#[source] server_runtime_http::ObservedError<HttpAdminHeaderValueError>),
}
impl AdminError {
    #[track_caller]
    fn observed<Source>(
        source: Source,
        code: AdminObservedErrorCode,
    ) -> server_runtime_http::ObservedError<Source>
    where
        Source: std::error::Error + 'static,
    {
        server_runtime_http::ObservedError::capture(
            source,
            server_runtime_http::ObservedErrorCode::from(code.get()),
        )
    }

    const fn route_error_status(&self) -> frontend_contract::RouteErrorStatus {
        match self {
            Self::Authentication | Self::AuthenticationSecretText(_) => {
                frontend_contract::RouteErrorStatus::Authentication
            }
            Self::Authorization | Self::Csrf | Self::CsrfSecretText(_) => {
                frontend_contract::RouteErrorStatus::Authorization
            }
            Self::Conflict => frontend_contract::RouteErrorStatus::Conflict,
            Self::MethodNotAllowed => frontend_contract::RouteErrorStatus::MethodNotAllowed,
            Self::PayloadTooLarge => frontend_contract::RouteErrorStatus::PayloadTooLarge,
            Self::RateLimited => frontend_contract::RouteErrorStatus::RateLimited,
            Self::Validation | Self::PasswordText(_) | Self::SecretText(_) => {
                frontend_contract::RouteErrorStatus::Validation
            }
            Self::Pg(_) | Self::PasswordHash(_) | Self::Session(_) | Self::Header(_) => {
                frontend_contract::RouteErrorStatus::Internal
            }
        }
    }

    #[track_caller]
    fn authentication_secret_text(source: super::AdminSecretTextError) -> Self {
        Self::AuthenticationSecretText(Self::observed(
            source,
            AdminObservedErrorCode::AuthenticationSecretText,
        ))
    }

    #[track_caller]
    fn csrf_secret_text(source: super::AdminSecretTextError) -> Self {
        Self::CsrfSecretText(Self::observed(
            source,
            AdminObservedErrorCode::CsrfSecretText,
        ))
    }

    #[track_caller]
    fn header(source: HttpAdminHeaderValueError) -> Self {
        Self::Header(Self::observed(source, AdminObservedErrorCode::Header))
    }

    #[track_caller]
    fn password_hash(source: super::AdminPasswordHashError) -> Self {
        Self::PasswordHash(Self::observed(source, AdminObservedErrorCode::PasswordHash))
    }

    #[track_caller]
    fn password_text(source: super::AdminPasswordTryFromStringError) -> Self {
        Self::PasswordText(Self::observed(source, AdminObservedErrorCode::PasswordText))
    }

    #[track_caller]
    fn pg(source: super::SqlxAdminError) -> Self {
        Self::Pg(Self::observed(source, AdminObservedErrorCode::Database))
    }

    #[track_caller]
    fn session(source: AdminSessionError) -> Self {
        Self::Session(Self::observed(source, AdminObservedErrorCode::Session))
    }

    #[track_caller]
    fn secret_text(source: super::AdminSecretTextError) -> Self {
        Self::SecretText(Self::observed(source, AdminObservedErrorCode::SecretText))
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::IntoInnerFrom, newtype::FromInner,
)]
pub struct AxumAdminResponse(axum::response::Response);
impl axum::response::IntoResponse for AdminError {
    fn into_response(self) -> axum::response::Response {
        let route_error_status = self.route_error_status();
        let error_type =
            server_runtime_http::HttpErrorType::from(constants_str::ADMIN_API_ERROR_TYPE);
        let optional_diagnostic = match &self {
            Self::Pg(source) => Some(server_runtime_http::HttpErrorDiagnostic::from_observed(
                error_type, source,
            )),
            Self::PasswordHash(source) => Some(
                server_runtime_http::HttpErrorDiagnostic::from_observed(error_type, source),
            ),
            Self::Session(source) => Some(server_runtime_http::HttpErrorDiagnostic::from_observed(
                error_type, source,
            )),
            Self::Header(source) => Some(server_runtime_http::HttpErrorDiagnostic::from_observed(
                error_type, source,
            )),
            Self::AuthenticationSecretText(source)
            | Self::CsrfSecretText(source)
            | Self::SecretText(source) => Some(
                server_runtime_http::HttpErrorDiagnostic::from_observed(error_type, source),
            ),
            Self::PasswordText(source) => Some(
                server_runtime_http::HttpErrorDiagnostic::from_observed(error_type, source),
            ),
            Self::Authentication
            | Self::Authorization
            | Self::Conflict
            | Self::Csrf
            | Self::MethodNotAllowed
            | Self::PayloadTooLarge
            | Self::RateLimited
            | Self::Validation => None,
        };
        admin_error_response_parts(route_error_status, optional_diagnostic)
    }
}
fn admin_error_response_parts(
    route_error_status: frontend_contract::RouteErrorStatus,
    optional_diagnostic: Option<server_runtime_http::HttpErrorDiagnostic>,
) -> axum::response::Response {
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
frontend_contract::api_operation_error!(AdminAuditLogError,);
frontend_contract::api_operation_error!(AdminAuditExportError,);
frontend_contract::api_operation_error!(AdminBrandingError,);
frontend_contract::api_operation_error!(AdminChangeOwnPasswordError,);
frontend_contract::api_operation_error!(AdminCreateRoleError,);
frontend_contract::api_operation_error!(AdminCreateUserError,);
frontend_contract::api_operation_error!(AdminDataTableError,);
frontend_contract::api_operation_error!(AdminDataTablesError,);
frontend_contract::api_operation_error!(AdminDeleteRoleError,);
frontend_contract::api_operation_error!(AdminDeleteUserError,);
frontend_contract::api_operation_error!(AdminListPermissionsError,);
frontend_contract::api_operation_error!(AdminListRolesError,);
frontend_contract::api_operation_error!(AdminListUsersError,);
frontend_contract::api_operation_error!(AdminMeError,);
frontend_contract::api_operation_error!(AdminRefreshError,);
frontend_contract::api_operation_error!(AdminRevokeAllSessionsError,);
frontend_contract::api_operation_error!(AdminRevokeSessionError,);
frontend_contract::api_operation_error!(AdminSessionsError,);
frontend_contract::api_operation_error!(AdminSetRolePermissionsError,);
frontend_contract::api_operation_error!(AdminSetUserBanError,);
frontend_contract::api_operation_error!(AdminSetUserPasswordError,);
frontend_contract::api_operation_error!(AdminSetUserRolesError,);
frontend_contract::api_operation_error!(AdminSettingsError,);
frontend_contract::api_operation_error!(AdminSignInError,);
frontend_contract::api_operation_error!(AdminSignOutError,);
frontend_contract::api_operation_error!(AdminUpdateRoleError,);
frontend_contract::api_operation_error!(AdminUpdateSettingsError,);
frontend_contract::api_operation_error!(AdminUpdateUserError,);
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
struct AdminAuditSuccessRef<'value_lt> {
    action: super::AdminAuditAction,
    login: &'value_lt super::AdminLogin,
    resource: super::AdminAuditResource,
    resource_id: AdminAuditResourceId,
    user_id: super::AdminUserId,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
enum AdminAuditResourceId {
    Role(super::AdminRoleId),
    Session(super::AdminSessionId),
    SystemSettings,
    User(super::AdminUserId),
}
impl AdminAuditResourceId {
    fn value(self) -> super::StdAdminString {
        match self {
            Self::User(value) => super::StdAdminString::from_positive_i64(value.value()),
            Self::Role(value) => super::StdAdminString::from_positive_i64(value.value()),
            Self::Session(value) => super::StdAdminString::from_uuid(value.0),
            Self::SystemSettings => super::StdAdminString::system_settings_resource(),
        }
    }
}
async fn record_audit_success_in_connection(
    connection: SqlxAdminPgConnectionRef<'_>,
    event: AdminAuditSuccessRef<'_>,
) -> Result<(), AdminError> {
    audit::record_success_in_connection(connection, event).await
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::AsMut, newtype::FromInner)]
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
    let (display_name, login, password_change_required, permissions, roles) = record.into_parts();
    Ok(AuthenticatedAdmin {
        display_name,
        id: user_id,
        login,
        password_change_required,
        permissions,
        roles,
        session_id,
    })
}
#[allow(clippy::single_call_fn)] // authentication flows create and rotate the long-lived refresh cookie
fn append_session_cookies(
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
    })?;
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
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    newtype::IntoInnerFrom,
    newtype::FromInner,
)]
pub struct AxumAdminAuthRouter(axum::Router);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::IntoInnerFrom, newtype::FromInner,
)]
pub struct UtoipaAdminAuthOpenApi(utoipa::openapi::OpenApi);
impl std::fmt::Debug for UtoipaAdminAuthOpenApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::UTOIPAADMINAUTHOPENAPI)
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
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
        login_failure_limit: &config_lib::AdminLoginFailureLimit,
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
            .map(str::to_owned)
            .collect::<Vec<String>>();
        Ok(Self {
            access_ttl: StdAdminAccessTtlSeconds::try_from(access_ttl.get())
                .map_err(AdminAuthSvcStateBuildError::PositiveValue)?,
            allowed_origins: server_runtime_http::AllowedOrigins::try_from(parsed_origins)
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
            policy: AdminAuthPolicy::from_limits(
                StdAdminFailureThreshold::try_from(
                    i64::try_from(login_failure_limit.get()).unwrap_or(i64::MAX),
                )
                .map_err(AdminAuthSvcStateBuildError::PositiveValue)?,
                StdAdminRateLimitCount::from(
                    i64::try_from(sign_in_rate_limit.get()).unwrap_or(i64::MAX),
                ),
            ),
        })
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
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
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error)]
pub enum AdminSessionError {
    #[error("administrator access token creation failed: {0:?}")]
    AccessToken(super::AdminAccessTokenError),
    #[error("administrator session database operation failed: {0:?}")]
    Pg(super::SqlxAdminError),
    #[error("administrator session secret text is invalid: {0}")]
    SecretText(super::AdminSecretTextError),
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
mod audit;
mod rate_limit;
mod routes;
mod session;
#[cfg(test)]
mod tests;
