#![allow(clippy::needless_for_each)] // utoipa 4 generated OpenAPI registration uses iterator callbacks
#[derive(newtype::DebugTransparent)]
pub struct JsonwebtokenAdminEncodingKey(jsonwebtoken::EncodingKey);
#[derive(newtype::DebugTransparent)]
pub struct JsonwebtokenAdminDecodingKey(jsonwebtoken::DecodingKey);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
pub struct StdAdminAccessTtlSeconds(u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
pub struct StdAdminRefreshTtlSeconds(u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::FromInner)]
pub struct StdAdminSessionLimit(usize);
#[derive(Debug)]
pub struct AdminAuthSvcState {
    access_ttl: StdAdminAccessTtlSeconds,
    allowed_origins: server_runtime::AllowedOrigins,
    audience: super::AdminTokenAudience,
    cookie_secure: super::AdminCookieSecure,
    decoding_keys: Vec<JsonwebtokenAdminDecodingKey>,
    encoding_key: JsonwebtokenAdminEncodingKey,
    issuer: super::AdminTokenIssuer,
    password_hasher: super::AdminPasswordHasher,
    pool: app_state::SqlxPgPool,
    refresh_ttl: StdAdminRefreshTtlSeconds,
    session_limit: StdAdminSessionLimit,
    sign_in_rate_limit: rate_limit::StdAdminRateLimitCount,
}
#[derive(Clone, Debug, newtype::AsRefOwned, newtype::FromInner)]
pub struct StdSharedAdminAuthSvcState(std::sync::Arc<AdminAuthSvcState>);
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum AdminAuthSvcStateBuildError {
    #[error("administrator allowed origin is invalid")]
    AllowedOrigin,
    #[error("administrator token audience is invalid")]
    Audience,
    #[error("administrator token issuer is invalid")]
    Issuer,
    #[error("administrator JWT secret list is empty")]
    JwtSecret,
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
    value: server_admin_contract::AdminPassword,
) -> Result<super::AdminPassword, AdminApiError> {
    let raw_value = value.into_inner();
    server_runtime::validate_password_policy(
        server_runtime::PasswordTextRef::from(raw_value.as_str()),
        server_runtime::PasswordLengthRange::from_prevalidated(
            server_runtime::PasswordLength::from(12usize),
            server_runtime::PasswordLength::from(server_admin_contract::ADMIN_PASSWORD_MAX_CHARS),
        ),
    )
    .map_err(|_error| AdminApiError::Validation)?;
    Ok(super::AdminPassword::new(super::SecrecyAdminString::from(
        secrecy::SecretBox::new(Box::new(raw_value)),
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
#[derive(Debug, newtype::AsRefOwned, newtype::FromInner)]
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
#[derive(Debug, Clone, Copy, newtype::FromInner)]
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
) -> Result<AuthenticatedAdmin, AdminApiError> {
    let token = super::find_admin_cookie(headers, super::AdminCookieKind::Access)
        .ok_or(AdminApiError::Authentication)?;
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[state.issuer.as_ref()]);
    validation.set_audience(&[state.audience.as_ref()]);
    let claims = state
        .decoding_keys
        .iter()
        .find_map(|decoding_key| {
            jsonwebtoken::decode::<super::AdminAccessClaims>(
                token.as_ref(),
                &decoding_key.0,
                &validation,
            )
            .ok()
            .map(|data| data.claims)
        })
        .ok_or(AdminApiError::Authentication)?;
    let context_hash = session_context_hash(headers, peer);
    let active = sqlx::query_scalar::<_, bool>(
        str_constants::SELECT_EXISTS_SELECT_1_FROM_ADMIN_ACCESS_SESSIONS_SESSION_JOIN_ADMIN_USERS,
    )
    .bind(claims.session_id().0.0)
    .bind(claims.user_id().0)
    .bind(secrecy::ExposeSecret::expose_secret(
        context_hash.0.as_ref(),
    ))
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
        .get(http::HeaderName::from_static(
            str_constants::X_CSRF_TOKEN_ALT,
        ))
        .and_then(|value| value.to_str().ok())
        .ok_or(AdminApiError::Csrf)?;
    let provided_token = super::AdminOpaqueToken::new(super::SecrecyAdminString::from(
        secrecy::SecretBox::new(Box::new(provided.to_owned())),
    ));
    let provided_hash = super::hash_opaque_token(&provided_token);
    let expected = sqlx::query_scalar::<_, String>(
        str_constants::SELECT_CSRF_TOKEN_HASH_FROM_ADMIN_ACCESS_SESSIONS_WHERE_ID_DOLLAR_1,
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
    permission: server_admin_contract::AdminPermissionStrRef<'_>,
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
#[derive(newtype::DebugTransparent, newtype::FromInner)]
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
#[derive(Debug, newtype::IntoInnerFrom)]
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
            http::HeaderValue::from_static(str_constants::APPLICATION_PROBLEM_PLUS_JSON),
        );
        if rate_limited {
            let _previous_retry_after = response.headers_mut().insert(
                http::header::RETRY_AFTER,
                http::HeaderValue::from_static(str_constants::VALUE_60),
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
    let _result = sqlx::query(
        str_constants::WITH_ATTEMPT_AS_INSERT_INTO_ADMIN_LOGIN_ATTEMPTS_LOGIN_IP_ADDRESS_SUCCEEDED,
    )
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
            Self::SystemSettings => super::StdAdminString(str_constants::VALUE_1.to_owned()),
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
#[derive(newtype::AsMut)]
struct SqlxAdminPgConnectionRef<'connection_lt>(&'connection_lt mut sqlx::PgConnection);
impl<'connection_lt> From<&'connection_lt mut sqlx::PgConnection>
    for SqlxAdminPgConnectionRef<'connection_lt>
{
    fn from(value: &'connection_lt mut sqlx::PgConnection) -> Self {
        Self(value)
    }
}
enum AdminAuthDbRef<'connection_lt, 'pool_lt> {
    Connection(SqlxAdminPgConnectionRef<'connection_lt>),
    Pool(app_state::SqlxPgPoolRef<'pool_lt>),
}
async fn load_authenticated_admin(
    state: &AdminAuthSvcState,
    user_id: super::AdminUserId,
    session_id: super::AdminSessionId,
) -> Result<AuthenticatedAdmin, AdminApiError> {
    let mut db = AdminAuthDbRef::Pool(app_state::SqlxPgPoolRef::from(state.pool.as_ref()));
    load_authenticated_admin_from_db(&mut db, user_id, session_id).await
}
async fn load_authenticated_admin_from_db(
    db: &mut AdminAuthDbRef<'_, '_>,
    user_id: super::AdminUserId,
    session_id: super::AdminSessionId,
) -> Result<AuthenticatedAdmin, AdminApiError> {
    let user_query = sqlx::query_as::<_, (String, String)>(
        str_constants::SELECT_LOGIN_DISPLAY_NAME_FROM_ADMIN_USERS_WHERE_ID_DOLLAR_1_AND,
    )
    .bind(user_id.0);
    let user = match db {
        AdminAuthDbRef::Connection(connection) => {
            user_query.fetch_optional(connection.as_mut()).await
        }
        AdminAuthDbRef::Pool(pool) => user_query.fetch_optional(pool.as_ref()).await,
    }
    .map_err(|error| AdminApiError::Pg(super::SqlxAdminError::from(error)))?
    .ok_or(AdminApiError::Authentication)?;
    let roles_query = sqlx::query_scalar::<_, String>(
        str_constants::SELECT_ROLE_NAME_FROM_ADMIN_ROLES_ROLE_JOIN_ADMIN_USER_ROLES_LINK,
    )
    .bind(user_id.0);
    let roles = match db {
        AdminAuthDbRef::Connection(connection) => roles_query.fetch_all(connection.as_mut()).await,
        AdminAuthDbRef::Pool(pool) => roles_query.fetch_all(pool.as_ref()).await,
    }
    .map_err(|error| AdminApiError::Pg(super::SqlxAdminError::from(error)))?
    .into_iter()
    .map(super::AdminRoleName::try_from)
    .collect::<Result<Vec<super::AdminRoleName>, _>>()
    .map_err(|_error| AdminApiError::Authentication)?;
    let permissions_query =
        sqlx::query_scalar::<_, String>(str_constants::SELECT_DISTINCT_PERMISSION_NAME_FROM_ADMIN_PERMISSIONS_PERMISSION_JOIN_ADMIN_ROLE_PERMISSIONS).bind(user_id.0);
    let permissions = match db {
        AdminAuthDbRef::Connection(connection) => {
            permissions_query.fetch_all(connection.as_mut()).await
        }
        AdminAuthDbRef::Pool(pool) => permissions_query.fetch_all(pool.as_ref()).await,
    }
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
#[allow(clippy::single_call_fn)] // sign-in alone creates the long-lived refresh cookie
fn append_session_cookies(
    response: &mut AxumAdminResponse,
    state: &AdminAuthSvcState,
    session: &AdminSessionBundle,
) -> Result<(), AdminApiError> {
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
        .map_err(|error| AdminApiError::Header(HttpAdminHeaderValueError::from(error)))
}
fn append_access_session_cookies(
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
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(get, path = "/auth/sessions", responses((status = 200, body = [server_admin_contract::AdminSessionView]), (status = 401, body = frontend_contract::ApiProblem), (status = 500, body = frontend_contract::ApiProblem)), security(("admin_cookie" = [])), tag = "admin_auth")]
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
#[derive(Debug, Clone, newtype::IntoInnerFrom)]
pub struct AxumAdminAuthRouter(axum::Router);
#[derive(Clone, newtype::IntoInnerFrom)]
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
            access_ttl: StdAdminAccessTtlSeconds::from(access_ttl.get()),
            allowed_origins: server_runtime::AllowedOrigins::try_from(parsed_origins)
                .map_err(|_error| AdminAuthSvcStateBuildError::AllowedOrigin)?,
            audience: super::AdminTokenAudience::try_from(audience.as_ref().clone())
                .map_err(|_error| AdminAuthSvcStateBuildError::Audience)?,
            cookie_secure: super::AdminCookieSecure::from(**cookie_secure),
            decoding_keys: jwt_secret
                .verification_secrets()
                .iter()
                .map(|verification_secret| {
                    JsonwebtokenAdminDecodingKey(jsonwebtoken::DecodingKey::from_secret(
                        secrecy::ExposeSecret::expose_secret(verification_secret.as_ref())
                            .as_bytes(),
                    ))
                })
                .collect(),
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
            .get(str_constants::PATHS)
            .and_then(serde_json::Value::as_object)
            .expect("6e15edec");
        assert_eq!(paths.len(), 17usize);
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
            .into_iter()
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
        assert!(paths.contains_key("/auth/sign-in"));
        assert!(paths.contains_key("/auth/sessions/{session_id}"));
        assert!(paths.contains_key("/users/{user_id}/password"));
        assert!(paths.contains_key("/roles/{role_id}/permissions"));
        assert!(paths.contains_key("/permissions"));
        assert!(paths.contains_key("/audit-log"));
        assert!(paths.contains_key("/system-settings"));
        assert_eq!(
            document
                .pointer(str_constants::ADMIN_OPENAPI_SIGN_IN_OPERATION_ID_POINTER)
                .and_then(serde_json::Value::as_str),
            Some(str_constants::ADMIN_OPERATION_SIGN_IN),
        );
        assert_eq!(
            document
                .pointer(str_constants::ADMIN_OPENAPI_REFRESH_OPERATION_ID_POINTER)
                .and_then(serde_json::Value::as_str),
            Some(str_constants::ADMIN_OPERATION_REFRESH),
        );
        assert_eq!(
            document
                .pointer(str_constants::ADMIN_OPENAPI_ME_OPERATION_ID_POINTER)
                .and_then(serde_json::Value::as_str),
            Some(str_constants::ADMIN_OPERATION_ME),
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
    }
}
mod audit;
mod handlers;
mod rate_limit;
mod routes;
mod session;
