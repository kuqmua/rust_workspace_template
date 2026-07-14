#![allow(clippy::needless_for_each)] // utoipa 4 generated OpenAPI registration uses iterator callbacks
#[derive(newtype::Newtype)]
#[newtype(debug_transparent)]
pub struct JsonwebtokenAdminEncodingKey(jsonwebtoken::EncodingKey);
#[derive(newtype::Newtype)]
#[newtype(debug_transparent)]
pub struct JsonwebtokenAdminDecodingKey(jsonwebtoken::DecodingKey);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(from_inner)]
pub struct StdAdminAccessTtlSeconds(u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(from_inner)]
pub struct StdAdminRefreshTtlSeconds(u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Newtype)]
#[newtype(from_inner)]
pub struct StdAdminSessionLimit(usize);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StdAdminAllowedOrigins(Vec<super::StdAdminString>);
#[derive(Debug)]
pub struct AdminAuthSvcState {
    access_ttl: StdAdminAccessTtlSeconds,
    allowed_origins: StdAdminAllowedOrigins,
    audience: super::AdminTokenAudience,
    cookie_secure: super::AdminCookieSecure,
    decoding_key: JsonwebtokenAdminDecodingKey,
    encoding_key: JsonwebtokenAdminEncodingKey,
    issuer: super::AdminTokenIssuer,
    password_hasher: super::AdminPasswordHasher,
    pool: app_state::SqlxPgPool,
    refresh_ttl: StdAdminRefreshTtlSeconds,
    session_limit: StdAdminSessionLimit,
    sign_in_rate_limit: rate_limit::StdAdminRateLimitCount,
}
#[derive(Clone, Debug, newtype::Newtype)]
#[newtype(as_ref_owned, from_inner)]
pub struct StdSharedAdminAuthSvcState(std::sync::Arc<AdminAuthSvcState>);
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum AdminAuthSvcStateBuildError {
    #[error("administrator token audience is invalid")]
    Audience,
    #[error("administrator token issuer is invalid")]
    Issuer,
}
fn admin_password_from_contract(
    value: server_admin_contract::AdminPassword,
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
    permissions: Vec<super::AdminPermission>,
    roles: Vec<super::AdminRoleName>,
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
) -> Result<server_admin_contract::AuthenticatedAdmin, AdminApiError> {
    let permissions = value
        .permissions
        .iter()
        .map(|permission| {
            server_admin_contract::AdminPermissionValue::try_from(
                permission.as_str().as_ref().to_owned(),
            )
            .map_err(|_error| AdminApiError::Validation)
        })
        .collect::<Result<Vec<_>, AdminApiError>>()?;
    let roles = value
        .roles
        .iter()
        .map(|role| {
            server_admin_contract::AdminRoleName::try_from(role.as_ref().to_owned())
                .map_err(|_error| AdminApiError::Validation)
        })
        .collect::<Result<Vec<_>, AdminApiError>>()?;
    Ok(server_admin_contract::AuthenticatedAdmin::new(
        server_admin_contract::AdminDisplayName::try_from(value.display_name.as_ref().to_owned())
            .map_err(|_error| AdminApiError::Validation)?,
        server_admin_contract::AdminUserId::from(value.id.0),
        server_admin_contract::AdminLogin::try_from(value.login.as_ref().to_owned())
            .map_err(|_error| AdminApiError::Validation)?,
        permissions,
        roles,
    ))
}
#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct AdminAuditQuery {
    action: Option<super::AdminAuditAction>,
    created_after: Option<server_admin_contract::AdminAuditTimestamp>,
    created_before: Option<server_admin_contract::AdminAuditTimestamp>,
    resource: Option<super::AdminAuditResource>,
    user_id: Option<super::AdminUserId>,
}
#[derive(Debug, newtype::Newtype)]
#[newtype(as_ref_owned, from_inner)]
pub struct HttpAdminHeaderMap(http::HeaderMap);
#[derive(Debug)]
pub struct AdminAuthReq {
    headers: HttpAdminHeaderMap,
    peer: AdminPeerAddr,
    state: StdSharedAdminAuthSvcState,
}
#[derive(Debug, Clone, Copy)]
pub struct AdminPeerAddr(super::StdAdminSocketAddr);
impl From<super::StdAdminSocketAddr> for AdminPeerAddr {
    fn from(value: super::StdAdminSocketAddr) -> Self {
        Self(value)
    }
}
impl<State> axum::extract::FromRequestParts<State> for AdminPeerAddr
where
    State: Send + Sync,
{
    type Rejection = AdminApiError;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &State,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(
            parts
                .extensions
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|value| Self(super::StdAdminSocketAddr::from(value.0)))
                .ok_or(AdminApiError::Authentication),
        )
    }
}
#[derive(Debug)]
pub struct AdminSignInJson(server_admin_contract::AdminSignInReq);
#[derive(Debug)]
pub struct AxumAdminJson<Value>(Value);
#[derive(Debug)]
pub struct AxumAdminPath<Value>(Value);
#[derive(Debug)]
pub struct AxumAdminQuery<Value>(Value);
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(from_inner)]
pub struct AdminSessionPath(super::AdminSessionId);
impl<S> axum::extract::FromRequestParts<S> for HttpAdminHeaderMap
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self(parts.headers.clone())))
    }
}
impl axum::extract::FromRequestParts<StdSharedAdminAuthSvcState> for AdminAuthReq {
    type Rejection = AdminApiError;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &StdSharedAdminAuthSvcState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(
            parts
                .extensions
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|peer| Self {
                    headers: HttpAdminHeaderMap(parts.headers.clone()),
                    peer: AdminPeerAddr(super::StdAdminSocketAddr::from(peer.0)),
                    state: state.clone(),
                })
                .ok_or(AdminApiError::Authentication),
        )
    }
}
impl<S> axum::extract::FromRequest<S> for AdminSignInJson
where
    S: Send + Sync,
{
    type Rejection = AdminApiError;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        axum::Json::<server_admin_contract::AdminSignInReq>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self(value))
            .map_err(|error| {
                if error.status() == http::StatusCode::PAYLOAD_TOO_LARGE {
                    AdminApiError::PayloadTooLarge
                } else {
                    AdminApiError::Validation
                }
            })
    }
}
impl<S, Value> axum::extract::FromRequest<S> for AxumAdminJson<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = AdminApiError;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        axum::Json::<Value>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self(value))
            .map_err(|error| {
                if error.status() == http::StatusCode::PAYLOAD_TOO_LARGE {
                    AdminApiError::PayloadTooLarge
                } else {
                    AdminApiError::Validation
                }
            })
    }
}
impl<S, Value> axum::extract::FromRequestParts<S> for AxumAdminPath<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = AdminApiError;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Path::<Value>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Path(value)| Self(value))
            .map_err(|_error| AdminApiError::Validation)
    }
}
impl<S, Value> axum::extract::FromRequestParts<S> for AxumAdminQuery<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = AdminApiError;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Query::<Value>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Query(value)| Self(value))
            .map_err(|_error| AdminApiError::Validation)
    }
}
impl axum::extract::FromRequestParts<StdSharedAdminAuthSvcState> for AdminSessionPath {
    type Rejection = AdminApiError;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &StdSharedAdminAuthSvcState,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Path::<uuid::Uuid>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Path(value)| {
                Self(super::AdminSessionId::from(super::UuidAdminValue::from(
                    value,
                )))
            })
            .map_err(|_error| AdminApiError::Validation)
    }
}
fn origin_authority(
    value: super::StdAdminStrRef<'_>,
    allow_suffix: super::StdAdminBool,
) -> Option<(super::StdAdminStrRef<'_>, super::StdAdminStrRef<'_>)> {
    let (scheme, remainder) = value.0.split_once("://")?;
    if !scheme.eq_ignore_ascii_case("http") && !scheme.eq_ignore_ascii_case("https") {
        return None;
    }
    let authority_end = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
    let authority = remainder.get(..authority_end)?;
    if authority.is_empty() || (!allow_suffix.0 && authority_end != remainder.len()) {
        None
    } else {
        Some((
            super::StdAdminStrRef::from(scheme),
            super::StdAdminStrRef::from(authority),
        ))
    }
}
fn origin_value_is_allowed(
    value: super::StdAdminStrRef<'_>,
    allow_suffix: super::StdAdminBool,
    allowed_origins: &[super::StdAdminString],
) -> super::StdAdminBool {
    let Some((scheme, authority)) =
        origin_authority(super::StdAdminStrRef::from(value.0.trim()), allow_suffix)
    else {
        return super::StdAdminBool::from(false);
    };
    super::StdAdminBool::from(allowed_origins.iter().any(|allowed_origin| {
        origin_authority(
            super::StdAdminStrRef::from(allowed_origin.as_ref().as_str()),
            super::StdAdminBool::from(false),
        )
        .is_some_and(|(allowed_scheme, allowed_authority)| {
            allowed_scheme.0.eq_ignore_ascii_case(scheme.0)
                && allowed_authority.0.eq_ignore_ascii_case(authority.0)
        })
    }))
}
fn session_context_hash(
    headers: super::HttpAdminHeaderMapRef<'_>,
    peer: AdminPeerAddr,
) -> super::AdminTokenHash {
    let mut context = String::with_capacity(352usize);
    context.push_str("client-address=");
    let client_address = peer.0.as_ref().ip().to_string();
    context.extend(client_address.chars().take(256usize));
    context.push_str("|user-agent=");
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
        None => context.push_str("unknown-user-agent"),
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
    let allowed = headers.0.get(http::header::ORIGIN).map_or_else(
        || {
            headers
                .0
                .get(http::header::REFERER)
                .and_then(|value| value.to_str().ok())
                .is_some_and(|referer| {
                    origin_value_is_allowed(
                        super::StdAdminStrRef::from(referer),
                        super::StdAdminBool::from(true),
                        &state.allowed_origins.0,
                    )
                    .0
                })
        },
        |origin_header| {
            origin_header.to_str().is_ok_and(|origin_value| {
                origin_value_is_allowed(
                    super::StdAdminStrRef::from(origin_value),
                    super::StdAdminBool::from(false),
                    &state.allowed_origins.0,
                )
                .0
            })
        },
    );
    super::StdAdminBool::from(allowed)
}
async fn authenticate(
    state: &AdminAuthSvcState,
    headers: super::HttpAdminHeaderMapRef<'_>,
    peer: AdminPeerAddr,
) -> Result<AuthenticatedAdmin, AdminApiError> {
    let token = super::find_admin_cookie(headers, super::AdminCookieKind::Access)
        .ok_or(AdminApiError::Authentication)?;
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[state.issuer.as_ref()]);
    validation.set_audience(&[state.audience.as_ref()]);
    let claims = jsonwebtoken::decode::<super::AdminAccessClaims>(
        token.as_ref(),
        &state.decoding_key.0,
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|_error| AdminApiError::Authentication)?;
    let context_hash = session_context_hash(headers, peer);
    let active = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (SELECT 1 FROM admin_access_sessions session JOIN admin_users users ON users.id = session.user_id WHERE session.id = $1 AND session.user_id = $2 AND session.token_context_hash = $3 AND session.revoked_at IS NULL AND session.expires_at > NOW() AND users.is_banned = FALSE)",
    )
    .bind(claims.session_id().0.0)
    .bind(claims.user_id().0)
    .bind(secrecy::ExposeSecret::expose_secret(context_hash.0.as_ref()))
    .fetch_one(state.pool.as_ref())
    .await
    .map_err(|error| AdminApiError::Pg(super::SqlxAdminError::from(error)))?;
    if !active {
        return Err(AdminApiError::Authentication);
    }
    load_authenticated_admin(state, claims.user_id(), claims.session_id()).await
}
async fn validate_csrf(
    state: &AdminAuthSvcState,
    headers: super::HttpAdminHeaderMapRef<'_>,
    authenticated: &AuthenticatedAdmin,
) -> Result<(), AdminApiError> {
    if !origin_is_present_and_allowed(state, headers).0 {
        return Err(AdminApiError::Csrf);
    }
    let provided = headers
        .0
        .get(http::HeaderName::from_static("x-csrf-token"))
        .and_then(|value| value.to_str().ok())
        .ok_or(AdminApiError::Csrf)?;
    let provided_token = super::AdminOpaqueToken::new(super::SecrecyAdminString::from(
        secrecy::SecretBox::new(Box::new(provided.to_owned())),
    ));
    let provided_hash = super::hash_opaque_token(&provided_token);
    let expected = sqlx::query_scalar::<_, String>(
        "SELECT csrf_token_hash FROM admin_access_sessions WHERE id = $1 AND user_id = $2 AND revoked_at IS NULL AND expires_at > NOW()",
    )
    .bind(authenticated.session_id.0.0)
    .bind(authenticated.id.0)
    .fetch_optional(state.pool.as_ref())
    .await
    .map_err(|error| AdminApiError::Pg(super::SqlxAdminError::from(error)))?
    .ok_or(AdminApiError::Csrf)?;
    if secrecy::ExposeSecret::expose_secret(provided_hash.0.as_ref()) != &expected {
        return Err(AdminApiError::Csrf);
    }
    Ok(())
}
pub async fn authorize_generated_request(
    state: &AdminAuthSvcState,
    headers: super::HttpAdminHeaderMapRef<'_>,
    peer: AdminPeerAddr,
    permission: super::StdAdminStrRef<'_>,
    mutates: super::StdAdminBool,
) -> Result<AuthenticatedAdmin, AdminApiError> {
    let authenticated = authenticate(state, headers, peer).await?;
    let required_permission = super::AdminPermission::try_from(permission.as_ref())
        .map_err(|_error| AdminApiError::Authorization)?;
    if !authenticated.permissions.contains(&required_permission) {
        return Err(AdminApiError::Authorization);
    }
    if mutates.0 {
        let subject = super::StdAdminString::try_from(authenticated.id.0.to_string())
            .map_err(|_error| AdminApiError::Validation)?;
        rate_limit::enforce_rate_limit(
            state,
            rate_limit::AdminRateLimitScope::Mutation,
            &subject,
            rate_limit::StdAdminRateLimitCount::from(300i64),
            rate_limit::StdAdminRateLimitWindowSeconds::from(60i32),
        )
        .await?;
        validate_csrf(state, headers, &authenticated).await?;
    }
    Ok(authenticated)
}
#[derive(newtype::Newtype)]
#[newtype(debug_transparent, from_inner)]
pub struct HttpAdminHeaderValueError(http::header::InvalidHeaderValue);
#[derive(Debug, thiserror::Error)]
pub enum AdminApiError {
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
    Pg(super::SqlxAdminError),
    #[error("administrator password hashing failed: {0}")]
    PasswordHash(super::AdminPasswordHashError),
    #[error("administrator request body is too large")]
    PayloadTooLarge,
    #[error("administrator route does not support this HTTP method")]
    MethodNotAllowed,
    #[error("administrator session operation failed: {0}")]
    Session(AdminSessionError),
    #[error("administrator response header is invalid: {0:?}")]
    Header(HttpAdminHeaderValueError),
}
impl From<sqlx::Error> for AdminApiError {
    fn from(value: sqlx::Error) -> Self {
        Self::Pg(super::SqlxAdminError::from(value))
    }
}
#[derive(Debug, newtype::Newtype)]
#[newtype(into_inner_from)]
pub struct AxumAdminResponse(axum::response::Response);
impl axum::response::IntoResponse for AdminApiError {
    fn into_response(self) -> axum::response::Response {
        let rate_limited = matches!(&self, Self::RateLimited);
        let status = match self {
            Self::Authentication => http::StatusCode::UNAUTHORIZED,
            Self::Authorization | Self::Csrf => http::StatusCode::FORBIDDEN,
            Self::Conflict => http::StatusCode::CONFLICT,
            Self::MethodNotAllowed => http::StatusCode::METHOD_NOT_ALLOWED,
            Self::PayloadTooLarge => http::StatusCode::PAYLOAD_TOO_LARGE,
            Self::RateLimited => http::StatusCode::TOO_MANY_REQUESTS,
            Self::Validation => http::StatusCode::UNPROCESSABLE_ENTITY,
            Self::Pg(_) | Self::PasswordHash(_) | Self::Session(_) | Self::Header(_) => {
                http::StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        let mut response = axum::response::IntoResponse::into_response((
            status,
            axum::Json(frontend_contract::ApiProblem::from_status(
                frontend_contract::ApiProblemStatus::from(status.as_u16()),
            )),
        ));
        let _previous_content_type = response.headers_mut().insert(
            http::header::CONTENT_TYPE,
            http::HeaderValue::from_static("application/problem+json"),
        );
        if rate_limited {
            let _previous_retry_after = response.headers_mut().insert(
                http::header::RETRY_AFTER,
                http::HeaderValue::from_static("60"),
            );
        }
        response
    }
}
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
) -> Result<(), AdminApiError> {
    let _result = sqlx::query("WITH attempt AS (INSERT INTO admin_login_attempts (login, ip_address, succeeded) VALUES ($1, $2, $3)) INSERT INTO admin_audit_log (user_login, action, resource, resource_id, request_id, succeeded, details) SELECT $1, 'sign_in', 'session', $1, $4, FALSE, jsonb_build_object('ip_address', $2::INET::TEXT) WHERE $3 = FALSE")
    .bind(login.as_ref())
    .bind(peer.0.0.ip())
    .bind(succeeded.0)
    .bind(uuid::Uuid::new_v4())
    .execute(state.pool.as_ref())
    .await
    .map_err(|error| AdminApiError::Pg(super::SqlxAdminError::from(error)))?;
    Ok(())
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
        match self {
            Self::Role(value) => super::StdAdminString(value.0.to_string()),
            Self::Session(value) => super::StdAdminString(value.0.0.to_string()),
            Self::SystemSettings => super::StdAdminString("1".to_owned()),
            Self::User(value) => super::StdAdminString(value.0.to_string()),
        }
    }
}
async fn record_audit_success_in_connection(
    connection: SqlxAdminPgConnectionRef<'_>,
    event: AdminAuditSuccessRef<'_>,
) -> Result<(), AdminApiError> {
    audit::record_success_in_connection(connection, event).await
}
struct SqlxAdminPgConnectionRef<'connection_lt>(&'connection_lt mut sqlx::PgConnection);
impl<'connection_lt> From<&'connection_lt mut sqlx::PgConnection>
    for SqlxAdminPgConnectionRef<'connection_lt>
{
    fn from(value: &'connection_lt mut sqlx::PgConnection) -> Self {
        Self(value)
    }
}
impl AsMut<sqlx::PgConnection> for SqlxAdminPgConnectionRef<'_> {
    fn as_mut(&mut self) -> &mut sqlx::PgConnection {
        self.0
    }
}
async fn load_authenticated_admin(
    state: &AdminAuthSvcState,
    user_id: super::AdminUserId,
    session_id: super::AdminSessionId,
) -> Result<AuthenticatedAdmin, AdminApiError> {
    let user = sqlx::query_as::<_, (String, String)>(
        "SELECT login, display_name FROM admin_users WHERE id = $1 AND is_banned = FALSE",
    )
    .bind(user_id.0)
    .fetch_optional(state.pool.as_ref())
    .await
    .map_err(|error| AdminApiError::Pg(super::SqlxAdminError::from(error)))?
    .ok_or(AdminApiError::Authentication)?;
    let roles = sqlx::query_scalar::<_, String>(
        "SELECT role.name FROM admin_roles role JOIN admin_user_roles link ON link.role_id = role.id WHERE link.user_id = $1 ORDER BY role.name",
    )
    .bind(user_id.0)
    .fetch_all(state.pool.as_ref())
    .await
    .map_err(|error| AdminApiError::Pg(super::SqlxAdminError::from(error)))?
    .into_iter()
    .map(super::AdminRoleName::try_from)
    .collect::<Result<Vec<super::AdminRoleName>, _>>()
    .map_err(|_error| AdminApiError::Authentication)?;
    let permissions = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT permission.name FROM admin_permissions permission JOIN admin_role_permissions role_permission ON role_permission.permission_id = permission.id JOIN admin_user_roles user_role ON user_role.role_id = role_permission.role_id WHERE user_role.user_id = $1 ORDER BY permission.name",
    )
    .bind(user_id.0)
    .fetch_all(state.pool.as_ref())
    .await
    .map_err(|error| AdminApiError::Pg(super::SqlxAdminError::from(error)))?
    .into_iter()
    .map(|permission| super::AdminPermission::try_from(permission.as_str()))
    .collect::<Result<Vec<super::AdminPermission>, _>>()
    .map_err(|_error| AdminApiError::Authentication)?;
    Ok(AuthenticatedAdmin {
        display_name: super::AdminDisplayName::try_from(user.1)
            .map_err(|_error| AdminApiError::Authentication)?,
        id: user_id,
        login: super::AdminLogin::try_from(user.0)
            .map_err(|_error| AdminApiError::Authentication)?,
        permissions,
        roles,
        session_id,
    })
}
fn append_session_cookies(
    response: &mut AxumAdminResponse,
    state: &AdminAuthSvcState,
    session: &AdminSessionBundle,
) -> Result<(), AdminApiError> {
    let access = super::build_admin_cookie(
        super::AdminCookieKind::Access,
        super::StdAdminStrRef::from(session.access_token.as_ref().as_str()),
        super::AdminCookieMaxAgeSeconds::from(state.access_ttl.0),
        state.cookie_secure,
    );
    let refresh = super::build_admin_cookie(
        super::AdminCookieKind::Refresh,
        session.refresh_token.expose(),
        super::AdminCookieMaxAgeSeconds::from(state.refresh_ttl.0),
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
    [access, refresh, csrf].into_iter().try_for_each(|cookie| {
        http::HeaderValue::from_str(cookie.as_ref())
            .map(|header| {
                response
                    .0
                    .headers_mut()
                    .append(http::header::SET_COOKIE, header)
            })
            .map(drop)
            .map_err(|error| AdminApiError::Header(HttpAdminHeaderValueError::from(error)))
    })
}
fn append_cleared_session_cookies(
    response: &mut AxumAdminResponse,
    state: &AdminAuthSvcState,
) -> Result<(), AdminApiError> {
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
            .map_err(|error| AdminApiError::Header(HttpAdminHeaderValueError::from(error)))
    })
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(post, path = "/auth/sign-in", request_body = server_admin_contract::AdminSignInReq, responses((status = 200, body = server_admin_contract::AdminSignInRes), (status = 401, body = frontend_contract::ApiProblem), (status = 429, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), tag = "admin_auth")]
async fn sign_in(
    auth: AdminAuthReq,
    peer: AdminPeerAddr,
    request_json: AdminSignInJson,
) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::sign_in(auth, peer, request_json).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(get, path = "/auth/me", responses((status = 200, body = server_admin_contract::AuthenticatedAdmin), (status = 401, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = [])), tag = "admin_auth")]
async fn me(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::me(auth).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(post, path = "/auth/refresh", responses((status = 200, body = server_admin_contract::AdminSignInRes), (status = 401, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), tag = "admin_auth")]
async fn refresh(
    auth: AdminAuthReq,
    peer: AdminPeerAddr,
) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::refresh(auth, peer).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(post, path = "/auth/sign-out", responses((status = 204), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_auth")]
async fn sign_out(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::sign_out(auth).await
}
#[derive(Debug, Clone, serde::Serialize, newtype::BoundedString, newtype::Newtype)]
#[bounded_string(
    max = 64,
    chars,
    description = "administrator session timestamp",
    utoipa
)]
#[newtype(as_ref_owned)]
pub struct AdminSessionTimestamp(String);
#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct AdminSessionView {
    created_at: AdminSessionTimestamp,
    expires_at: AdminSessionTimestamp,
    id: super::AdminSessionId,
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(get, path = "/auth/sessions", responses((status = 200, body = [AdminSessionView]), (status = 401, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = [])), tag = "admin_auth")]
async fn sessions(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::sessions(auth).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(delete, path = "/auth/sessions/{session_id}", responses((status = 204), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_auth")]
async fn revoke_session(
    auth: AdminAuthReq,
    session: AdminSessionPath,
) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::revoke_session(auth, session).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(delete, path = "/auth/sessions", responses((status = 204), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_auth")]
async fn revoke_all_sessions(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::revoke_all_sessions(auth).await
}
async fn authorize_custom(
    auth: &AdminAuthReq,
    permission: super::AdminPermission,
) -> Result<AuthenticatedAdmin, AdminApiError> {
    authorize_generated_request(
        auth.state.as_ref(),
        super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        permission.as_str(),
        super::StdAdminBool::from(true),
    )
    .await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(post, path = "/users", request_body = server_admin_contract::AdminCreateUserReq, responses((status = 201, body = server_admin_contract::AdminCreateUserRes), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 409, body = frontend_contract::ApiProblem), (status = 422, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_users")]
async fn create_user(
    auth: AdminAuthReq,
    request: AxumAdminJson<server_admin_contract::AdminCreateUserReq>,
) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::create_user(auth, request).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(patch, path = "/users/{user_id}", request_body = server_admin_contract::AdminUpdateUserReq, responses((status = 204), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 409, body = frontend_contract::ApiProblem), (status = 422, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_users")]
async fn update_user(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
    request: AxumAdminJson<server_admin_contract::AdminUpdateUserReq>,
) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::update_user(auth, path, request).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(post, path = "/users/{user_id}/password", request_body = server_admin_contract::AdminSetUserPasswordReq, responses((status = 204), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 409, body = frontend_contract::ApiProblem), (status = 422, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_users")]
async fn set_user_password(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
    request: AxumAdminJson<server_admin_contract::AdminSetUserPasswordReq>,
) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::set_user_password(auth, path, request).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(post, path = "/users/{user_id}/ban", request_body = server_admin_contract::AdminSetUserBanReq, responses((status = 204), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 409, body = frontend_contract::ApiProblem), (status = 422, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_users")]
async fn set_user_ban(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
    request: AxumAdminJson<server_admin_contract::AdminSetUserBanReq>,
) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::set_user_ban(auth, path, request).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(delete, path = "/users/{user_id}", responses((status = 204), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 409, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_users")]
async fn delete_user(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::delete_user(auth, path).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(post, path = "/roles", request_body = server_admin_contract::AdminCreateRoleReq, responses((status = 201, body = server_admin_contract::AdminCreateRoleRes), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 409, body = frontend_contract::ApiProblem), (status = 422, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_roles")]
async fn create_role(
    auth: AdminAuthReq,
    request: AxumAdminJson<server_admin_contract::AdminCreateRoleReq>,
) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::create_role(auth, request).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(patch, path = "/roles/{role_id}", request_body = server_admin_contract::AdminUpdateRoleReq, responses((status = 204), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 409, body = frontend_contract::ApiProblem), (status = 422, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_roles")]
async fn update_role(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminRoleId>,
    request: AxumAdminJson<server_admin_contract::AdminUpdateRoleReq>,
) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::update_role(auth, path, request).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(delete, path = "/roles/{role_id}", responses((status = 204), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 409, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_roles")]
async fn delete_role(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminRoleId>,
) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::delete_role(auth, path).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(put, path = "/roles/{role_id}/permissions", request_body = server_admin_contract::AdminSetRolePermissionsReq, responses((status = 204), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 409, body = frontend_contract::ApiProblem), (status = 422, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_roles")]
async fn set_role_permissions(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminRoleId>,
    request: AxumAdminJson<server_admin_contract::AdminSetRolePermissionsReq>,
) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::set_role_permissions(auth, path, request).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(put, path = "/users/{user_id}/roles", request_body = server_admin_contract::AdminSetUserRolesReq, responses((status = 204), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 409, body = frontend_contract::ApiProblem), (status = 422, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_users")]
async fn set_user_roles(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
    request: AxumAdminJson<server_admin_contract::AdminSetUserRolesReq>,
) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::set_user_roles(auth, path, request).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(get, path = "/audit-log", params(AdminAuditQuery), responses((status = 200, body = [server_admin_contract::AdminAuditView]), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 422, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = [])), tag = "admin_audit")]
async fn audit_log(
    auth: AdminAuthReq,
    query: AxumAdminQuery<AdminAuditQuery>,
) -> Result<AxumAdminResponse, AdminApiError> {
    audit::query_log(auth, query).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(patch, path = "/system-settings", request_body = server_admin_contract::AdminUpdateSettingsReq, responses((status = 204), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 409, body = frontend_contract::ApiProblem), (status = 422, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_settings")]
async fn update_settings(
    auth: AdminAuthReq,
    request: AxumAdminJson<server_admin_contract::AdminUpdateSettingsReq>,
) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::update_settings(auth, request).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(get, path = "/users", responses((status = 200, body = [server_admin_contract::AdminUserSummary]), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = [])), tag = "admin_users")]
async fn list_users(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::list_users(auth).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(get, path = "/roles", responses((status = 200, body = [server_admin_contract::AdminRoleSummary]), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = [])), tag = "admin_roles")]
async fn list_roles(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::list_roles(auth).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(get, path = "/permissions", responses((status = 200, body = [server_admin_contract::AdminPermissionSummary]), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = [])), tag = "admin_roles")]
async fn list_permissions(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::list_permissions(auth).await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(get, path = "/system-settings", responses((status = 200, body = server_admin_contract::AdminSettingsView), (status = 401, body = frontend_contract::ApiProblem), (status = 403, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = [])), tag = "admin_settings")]
async fn settings(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiError> {
    handlers::settings(auth).await
}
#[derive(Debug, Clone, newtype::Newtype)]
#[newtype(into_inner_from)]
pub struct AxumAdminAuthRouter(axum::Router);
#[derive(Clone, newtype::Newtype)]
#[newtype(into_inner_from)]
pub struct UtoipaAdminAuthOpenApi(utoipa::openapi::OpenApi);
impl std::fmt::Debug for UtoipaAdminAuthOpenApi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("UtoipaAdminAuthOpenApi")
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
        let secret = secrecy::ExposeSecret::expose_secret(jwt_secret.as_ref().as_ref());
        let parsed_origins = allowed_origins
            .0
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| super::StdAdminString(value.to_owned()))
            .collect::<Vec<super::StdAdminString>>();
        Ok(Self {
            access_ttl: StdAdminAccessTtlSeconds::from(access_ttl.get()),
            allowed_origins: StdAdminAllowedOrigins(parsed_origins),
            audience: super::AdminTokenAudience::try_from(audience.as_ref().clone())
                .map_err(|_error| AdminAuthSvcStateBuildError::Audience)?,
            cookie_secure: super::AdminCookieSecure::from(**cookie_secure),
            decoding_key: JsonwebtokenAdminDecodingKey(jsonwebtoken::DecodingKey::from_secret(
                secret.as_bytes(),
            )),
            encoding_key: JsonwebtokenAdminEncodingKey(jsonwebtoken::EncodingKey::from_secret(
                secret.as_bytes(),
            )),
            issuer: super::AdminTokenIssuer::try_from(issuer.as_ref().clone())
                .map_err(|_error| AdminAuthSvcStateBuildError::Issuer)?,
            password_hasher: super::AdminPasswordHasher::new(
                super::AdminPasswordHashConcurrency::from(super::StdAdminNonZeroUsize::from(
                    std::num::NonZeroUsize::new(password_hash_concurrency.get())
                        .ok_or(AdminAuthSvcStateBuildError::Issuer)?,
                )),
            ),
            pool,
            refresh_ttl: StdAdminRefreshTtlSeconds::from(refresh_ttl.get()),
            session_limit: StdAdminSessionLimit::from(session_limit.get()),
            sign_in_rate_limit: rate_limit::StdAdminRateLimitCount::from(
                i64::try_from(sign_in_rate_limit.get()).unwrap_or(i64::MAX),
            ),
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
async fn create_session_in_connection(
    state: &AdminAuthSvcState,
    user_id: super::AdminUserId,
    context_hash: &super::AdminTokenHash,
    connection: SqlxAdminPgConnectionRef<'_>,
) -> Result<AdminSessionBundle, AdminSessionError> {
    session::create_session_in_connection(state, user_id, context_hash, connection).await
}
#[cfg(test)]
mod tests {
    #[test]
    fn rate_limit_scopes_are_distinct() {
        let scopes = [
            super::rate_limit::AdminRateLimitScope::AuditRead,
            super::rate_limit::AdminRateLimitScope::Mutation,
            super::rate_limit::AdminRateLimitScope::RefreshIp,
            super::rate_limit::AdminRateLimitScope::SignInIp,
            super::rate_limit::AdminRateLimitScope::SignInIpLogin,
        ]
        .map(super::rate_limit::AdminRateLimitScope::as_str);
        let unique = scopes.into_iter().collect::<std::collections::HashSet<_>>();
        assert_eq!(unique.len(), 5usize);
    }
    #[test]
    fn rate_limited_error_includes_retry_after_header() {
        let response =
            axum::response::IntoResponse::into_response(super::AdminApiError::RateLimited);
        assert_eq!(response.status(), http::StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(
            response.headers().get(http::header::RETRY_AFTER),
            Some(&http::HeaderValue::from_static("60")),
        );
    }
    #[test]
    fn origin_policy_accepts_referer_suffix_and_rejects_origin_suffix() {
        let allowed_origins =
            [
                crate::StdAdminString::try_from("https://admin.example.com".to_owned())
                    .expect("7c9e8046"),
            ];
        assert!(
            super::origin_value_is_allowed(
                crate::StdAdminStrRef::from(
                    "HTTPS://ADMIN.EXAMPLE.COM/settings?tab=security#roles"
                ),
                crate::StdAdminBool::from(true),
                &allowed_origins,
            )
            .0
        );
        assert!(
            !super::origin_value_is_allowed(
                crate::StdAdminStrRef::from("https://admin.example.com/settings"),
                crate::StdAdminBool::from(false),
                &allowed_origins,
            )
            .0
        );
        assert!(
            !super::origin_value_is_allowed(
                crate::StdAdminStrRef::from("https://blocked.example.com"),
                crate::StdAdminBool::from(false),
                &allowed_origins,
            )
            .0
        );
        assert!(
            !super::origin_value_is_allowed(
                crate::StdAdminStrRef::from("javascript://admin.example.com"),
                crate::StdAdminBool::from(false),
                &allowed_origins,
            )
            .0
        );
    }
    #[test]
    fn session_context_hash_is_bound_to_peer_and_user_agent() {
        let mut first_headers = http::HeaderMap::new();
        let _previous_user_agent = first_headers.insert(
            http::header::USER_AGENT,
            http::HeaderValue::from_static("admin-client/1"),
        );
        let first_peer = super::AdminPeerAddr::from(super::super::StdAdminSocketAddr::from(
            "192.0.2.10:443"
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
            "192.0.2.11:443"
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
            http::HeaderValue::from_static("admin-client/2"),
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
            super::AdminAuditResourceId::User(crate::AdminUserId::from(42i64))
                .value()
                .as_ref(),
            "42"
        );
        assert_eq!(
            super::AdminAuditResourceId::Role(crate::AdminRoleId::from(7i64))
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
        let document = serde_json::to_value(utoipa::openapi::OpenApi::from(super::open_api()))
            .expect("869d28d7");
        let paths = document
            .get("paths")
            .and_then(serde_json::Value::as_object)
            .expect("6e15edec");
        assert_eq!(paths.len(), 17usize);
        let documented_method_paths = paths
            .iter()
            .flat_map(|(path, path_item)| {
                path_item
                    .as_object()
                    .into_iter()
                    .flat_map(|operation_map| operation_map.keys())
                    .map(move |method| (method.to_owned(), path.to_owned()))
            })
            .collect::<std::collections::BTreeSet<_>>();
        let contracted_method_paths = server_admin_contract::AdminRoute::auth_routes()
            .into_iter()
            .map(|route| {
                let contract = route.contract();
                let method = match contract.method() {
                    frontend_contract::HttpMethod::Delete => "delete",
                    frontend_contract::HttpMethod::Get => "get",
                    frontend_contract::HttpMethod::Patch => "patch",
                    frontend_contract::HttpMethod::Post => "post",
                    frontend_contract::HttpMethod::Put => "put",
                };
                (method.to_owned(), contract.path().as_ref().to_owned())
            })
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(documented_method_paths, contracted_method_paths);
        assert!(paths.contains_key("/auth/sign-in"));
        assert!(paths.contains_key("/auth/sessions/{session_id}"));
        assert!(paths.contains_key("/users/{user_id}/password"));
        assert!(paths.contains_key("/roles/{role_id}/permissions"));
        assert!(paths.contains_key("/permissions"));
        assert!(paths.contains_key("/audit-log"));
        assert!(paths.contains_key("/system-settings"));
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
    }
}
mod audit;
mod handlers;
mod rate_limit;
mod routes;
mod session;
