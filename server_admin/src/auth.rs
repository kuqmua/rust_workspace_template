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
    sign_in_rate_limit: StdAdminRateLimitCount,
}
#[derive(Clone, Debug, newtype::Newtype)]
#[newtype(as_ref_owned, from_inner)]
pub struct StdSharedAdminAuthSvcState(std::sync::Arc<AdminAuthSvcState>);
#[derive(Clone, Copy, Debug, thiserror::Error)]
pub enum AdminAuthSvcStateBuildEr {
    #[error("administrator token audience is invalid")]
    Audience,
    #[error("administrator token issuer is invalid")]
    Issuer,
}
#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminSignInReq {
    login: super::AdminLogin,
    password: super::AdminPassword,
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
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct AdminSignInRes {
    user: AuthenticatedAdmin,
}
#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminCreateUserReq {
    display_name: super::AdminDisplayName,
    login: super::AdminLogin,
    password: super::AdminPassword,
}
#[derive(Debug, Clone, Copy, serde::Serialize, utoipa::ToSchema)]
pub struct AdminCreateUserRes {
    id: super::AdminUserId,
}
#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminUpdateUserReq {
    display_name: Option<super::AdminDisplayName>,
    login: Option<super::AdminLogin>,
}
#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminSetUserPasswordReq {
    password: super::AdminPassword,
}
#[derive(Debug, Clone, Copy, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminSetUserBanReq {
    is_banned: super::StdAdminBool,
}
#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminCreateRoleReq {
    name: super::AdminRoleName,
}
#[derive(Debug, Clone, Copy, serde::Serialize, utoipa::ToSchema)]
pub struct AdminCreateRoleRes {
    id: super::AdminRoleId,
}
#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminUpdateRoleReq {
    name: super::AdminRoleName,
}
#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminSetUserRolesReq {
    role_ids: Vec<super::AdminRoleId>,
}
#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminSetRolePermissionsReq {
    permission_ids: Vec<super::AdminPermissionId>,
}
#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema, newtype::Newtype)]
#[newtype(from_inner)]
pub struct SerdeJsonAdminValue(serde_json::Value);
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, newtype::BoundedString, newtype::Newtype,
)]
#[bounded_string(max = 64, chars, description = "administrator audit timestamp", utoipa)]
#[newtype(as_ref_owned)]
pub struct AdminAuditTimestamp(String);
#[derive(Debug, serde::Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct AdminAuditQuery {
    action: Option<super::AdminAuditAction>,
    created_after: Option<AdminAuditTimestamp>,
    created_before: Option<AdminAuditTimestamp>,
    resource: Option<super::AdminAuditResource>,
    user_id: Option<super::AdminUserId>,
}
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct AdminAuditView {
    action: super::AdminAuditAction,
    created_at: AdminAuditTimestamp,
    details: Option<SerdeJsonAdminValue>,
    id: super::AdminAuditLogId,
    resource: super::AdminAuditResource,
    resource_id: Option<super::StdAdminString>,
    succeeded: super::StdAdminBool,
    user_id: Option<super::AdminUserId>,
    user_login: Option<super::AdminLogin>,
}
#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, newtype::BoundedString, newtype::Newtype,
)]
#[bounded_string(max = 8192, chars, description = "administrator setting text", utoipa)]
#[newtype(as_ref_owned)]
pub struct AdminSettingText(String);
#[derive(Debug, serde::Deserialize, utoipa::ToSchema)]
pub struct AdminUpdateSettingsReq {
    default_admin_route: Option<AdminSettingText>,
    main_logo: Option<AdminSettingText>,
    organization_contacts: Option<AdminSettingText>,
    organization_name: Option<AdminSettingText>,
    primary_color: Option<AdminSettingText>,
    site_name: Option<AdminSettingText>,
    support_url: Option<AdminSettingText>,
    tab_title: Option<AdminSettingText>,
}
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct AdminUserSummary {
    display_name: super::AdminDisplayName,
    id: super::AdminUserId,
    is_banned: super::StdAdminBool,
    login: super::AdminLogin,
}
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct AdminRoleSummary {
    id: super::AdminRoleId,
    is_system: super::StdAdminBool,
    name: super::AdminRoleName,
}
#[derive(Debug, Clone, Copy, serde::Serialize, utoipa::ToSchema)]
pub struct AdminPermissionSummary {
    id: super::AdminPermissionId,
    name: super::AdminPermission,
}
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
pub struct AdminSettingsView {
    default_admin_route: AdminSettingText,
    main_logo: Option<AdminSettingText>,
    organization_contacts: Option<AdminSettingText>,
    organization_name: Option<AdminSettingText>,
    primary_color: Option<AdminSettingText>,
    site_name: AdminSettingText,
    support_url: Option<AdminSettingText>,
    tab_title: Option<AdminSettingText>,
}
#[derive(Clone, Copy, Debug, serde::Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AdminApiErCode {
    AuthenticationFailed,
    AuthorizationFailed,
    CsrfFailed,
    Conflict,
    InternalError,
    RateLimited,
    ValidationFailed,
}
#[derive(Clone, Copy, Debug, serde::Serialize, utoipa::ToSchema)]
pub struct AdminApiErBody {
    code: AdminApiErCode,
}
#[derive(Debug, newtype::Newtype)]
#[newtype(as_ref_owned, from_inner)]
pub struct HttpAdminHeaderMap(http::HeaderMap);
#[derive(Debug)]
pub struct AdminAuthReq {
    headers: HttpAdminHeaderMap,
    state: StdSharedAdminAuthSvcState,
}
#[derive(Debug, Clone, Copy)]
pub struct AdminPeerAddr(super::StdAdminSocketAddr);
impl<State> axum::extract::FromRequestParts<State> for AdminPeerAddr
where
    State: Send + Sync,
{
    type Rejection = AdminApiEr;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &State,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(
            parts
                .extensions
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|value| Self(super::StdAdminSocketAddr::from(value.0)))
                .ok_or(AdminApiEr::Authentication),
        )
    }
}
#[derive(Debug)]
pub struct AdminSignInJson(AdminSignInReq);
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
    type Rejection = std::convert::Infallible;
    fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &StdSharedAdminAuthSvcState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self {
            headers: HttpAdminHeaderMap(parts.headers.clone()),
            state: state.clone(),
        }))
    }
}
impl<S> axum::extract::FromRequest<S> for AdminSignInJson
where
    S: Send + Sync,
{
    type Rejection = axum::extract::rejection::JsonRejection;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        axum::Json::<AdminSignInReq>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self(value))
    }
}
impl<S, Value> axum::extract::FromRequest<S> for AxumAdminJson<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = axum::extract::rejection::JsonRejection;
    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        axum::Json::<Value>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self(value))
    }
}
impl<S, Value> axum::extract::FromRequestParts<S> for AxumAdminPath<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = axum::extract::rejection::PathRejection;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Path::<Value>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Path(value)| Self(value))
    }
}
impl<S, Value> axum::extract::FromRequestParts<S> for AxumAdminQuery<Value>
where
    S: Send + Sync,
    Value: serde::de::DeserializeOwned + Send,
{
    type Rejection = axum::extract::rejection::QueryRejection;
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        axum::extract::Query::<Value>::from_request_parts(parts, state)
            .await
            .map(|axum::extract::Query(value)| Self(value))
    }
}
impl axum::extract::FromRequestParts<StdSharedAdminAuthSvcState> for AdminSessionPath {
    type Rejection = axum::extract::rejection::PathRejection;
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
    }
}
fn origin_is_allowed(
    state: &AdminAuthSvcState,
    headers: super::HttpAdminHeaderMapRef<'_>,
) -> super::StdAdminBool {
    let allowed = headers
        .0
        .get(http::header::ORIGIN)
        .and_then(|value| value.to_str().ok())
        .is_none_or(|origin| {
            state
                .allowed_origins
                .0
                .iter()
                .any(|allowed_origin| allowed_origin.as_ref() == origin)
        });
    super::StdAdminBool::from(allowed)
}
#[allow(clippy::single_call_fn)] // CSRF origin validation stays isolated from token validation
fn origin_is_present_and_allowed(
    state: &AdminAuthSvcState,
    headers: super::HttpAdminHeaderMapRef<'_>,
) -> super::StdAdminBool {
    super::StdAdminBool::from(
        headers
            .0
            .get(http::header::ORIGIN)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|origin| {
                state
                    .allowed_origins
                    .0
                    .iter()
                    .any(|allowed_origin| allowed_origin.as_ref() == origin)
            }),
    )
}
async fn authenticate(
    state: &AdminAuthSvcState,
    headers: super::HttpAdminHeaderMapRef<'_>,
) -> Result<AuthenticatedAdmin, AdminApiEr> {
    let token = super::find_admin_cookie(headers, super::AdminCookieKind::Access)
        .ok_or(AdminApiEr::Authentication)?;
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[state.issuer.as_ref()]);
    validation.set_audience(&[state.audience.as_ref()]);
    let claims = jsonwebtoken::decode::<super::AdminAccessClaims>(
        token.as_ref(),
        &state.decoding_key.0,
        &validation,
    )
    .map(|data| data.claims)
    .map_err(|_er| AdminApiEr::Authentication)?;
    let active = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (SELECT 1 FROM admin_access_sessions session JOIN admin_users users ON users.id = session.user_id WHERE session.id = $1 AND session.user_id = $2 AND session.revoked_at IS NULL AND session.expires_at > NOW() AND users.is_banned = FALSE)",
    )
    .bind(claims.session_id().0.0)
    .bind(claims.user_id().0)
    .fetch_one(state.pool.as_ref())
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    if !active {
        return Err(AdminApiEr::Authentication);
    }
    load_authenticated_admin(state, claims.user_id(), claims.session_id()).await
}
async fn validate_csrf(
    state: &AdminAuthSvcState,
    headers: super::HttpAdminHeaderMapRef<'_>,
    authenticated: &AuthenticatedAdmin,
) -> Result<(), AdminApiEr> {
    if !origin_is_present_and_allowed(state, headers).0 {
        return Err(AdminApiEr::Csrf);
    }
    let provided = headers
        .0
        .get(http::HeaderName::from_static("x-csrf-token"))
        .and_then(|value| value.to_str().ok())
        .ok_or(AdminApiEr::Csrf)?;
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
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?
    .ok_or(AdminApiEr::Csrf)?;
    if secrecy::ExposeSecret::expose_secret(provided_hash.0.as_ref()) != &expected {
        return Err(AdminApiEr::Csrf);
    }
    Ok(())
}
pub async fn authorize_generated_request(
    state: &AdminAuthSvcState,
    headers: super::HttpAdminHeaderMapRef<'_>,
    permission: super::StdAdminStrRef<'_>,
    mutates: super::StdAdminBool,
) -> Result<AuthenticatedAdmin, AdminApiEr> {
    let authenticated = authenticate(state, headers).await?;
    let required_permission = super::AdminPermission::try_from(permission.as_ref())
        .map_err(|_er| AdminApiEr::Authorization)?;
    if !authenticated.permissions.contains(&required_permission) {
        return Err(AdminApiEr::Authorization);
    }
    if mutates.0 {
        let subject = super::StdAdminString::try_from(authenticated.id.0.to_string())
            .map_err(|_er| AdminApiEr::Validation)?;
        enforce_rate_limit(
            state,
            AdminRateLimitScope::Mutation,
            &subject,
            StdAdminRateLimitCount::from(300i64),
            StdAdminRateLimitWindowSeconds::from(60i32),
        )
        .await?;
        validate_csrf(state, headers, &authenticated).await?;
    }
    Ok(authenticated)
}
#[derive(newtype::Newtype)]
#[newtype(debug_transparent, from_inner)]
pub struct HttpAdminHeaderValueEr(http::header::InvalidHeaderValue);
#[derive(Debug, thiserror::Error)]
pub enum AdminApiEr {
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
    Pg(super::SqlxAdminEr),
    #[error("administrator password hashing failed: {0}")]
    PasswordHash(super::AdminPasswordHashEr),
    #[error("administrator session operation failed: {0}")]
    Session(AdminSessionEr),
    #[error("administrator response header is invalid: {0:?}")]
    Header(HttpAdminHeaderValueEr),
}
#[derive(Debug, newtype::Newtype)]
#[newtype(into_inner_from)]
pub struct AxumAdminResponse(axum::response::Response);
impl axum::response::IntoResponse for AdminApiEr {
    fn into_response(self) -> axum::response::Response {
        let (status, code) = match self {
            Self::Authentication => (
                http::StatusCode::UNAUTHORIZED,
                AdminApiErCode::AuthenticationFailed,
            ),
            Self::Authorization => (
                http::StatusCode::FORBIDDEN,
                AdminApiErCode::AuthorizationFailed,
            ),
            Self::Conflict => (http::StatusCode::CONFLICT, AdminApiErCode::Conflict),
            Self::Csrf => (http::StatusCode::FORBIDDEN, AdminApiErCode::CsrfFailed),
            Self::RateLimited => (
                http::StatusCode::TOO_MANY_REQUESTS,
                AdminApiErCode::RateLimited,
            ),
            Self::Validation => (
                http::StatusCode::UNPROCESSABLE_ENTITY,
                AdminApiErCode::ValidationFailed,
            ),
            Self::Pg(_) | Self::PasswordHash(_) | Self::Session(_) | Self::Header(_) => (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                AdminApiErCode::InternalError,
            ),
        };
        axum::response::IntoResponse::into_response((status, axum::Json(AdminApiErBody { code })))
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
) -> Result<(), AdminApiEr> {
    let _result = sqlx::query("WITH attempt AS (INSERT INTO admin_login_attempts (login, ip_address, succeeded) VALUES ($1, $2, $3)) INSERT INTO admin_audit_log (user_login, action, resource, resource_id, request_id, succeeded, details) SELECT $1, 'sign_in', 'session', $1, $4, FALSE, jsonb_build_object('ip_address', $2::INET::TEXT) WHERE $3 = FALSE")
    .bind(login.as_ref())
    .bind(peer.0.0.ip())
    .bind(succeeded.0)
    .bind(uuid::Uuid::new_v4())
    .execute(state.pool.as_ref())
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(())
}
#[derive(Debug, Clone, Copy)]
enum AdminRateLimitScope {
    AuditRead,
    Mutation,
    RefreshIp,
    SignInIp,
    SignInIpLogin,
}
impl AdminRateLimitScope {
    #[allow(clippy::single_call_fn)] // scope serialization is shared by persistence and exhaustive contract tests
    const fn as_str(self) -> super::StdAdminStrRef<'static> {
        match self {
            Self::AuditRead => super::StdAdminStrRef("audit_read"),
            Self::Mutation => super::StdAdminStrRef("mutation"),
            Self::RefreshIp => super::StdAdminStrRef("refresh_ip"),
            Self::SignInIp => super::StdAdminStrRef("sign_in_ip"),
            Self::SignInIpLogin => super::StdAdminStrRef("sign_in_ip_login"),
        }
    }
}
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(from_inner)]
struct StdAdminRateLimitCount(i64);
#[derive(Debug, Clone, Copy, newtype::Newtype)]
#[newtype(from_inner)]
struct StdAdminRateLimitWindowSeconds(i32);
async fn enforce_rate_limit(
    state: &AdminAuthSvcState,
    scope: AdminRateLimitScope,
    subject: &super::StdAdminString,
    limit: StdAdminRateLimitCount,
    window_seconds: StdAdminRateLimitWindowSeconds,
) -> Result<(), AdminApiEr> {
    let allowed = sqlx::query_scalar::<_, bool>("INSERT INTO admin_rate_limits (scope, subject, window_started_at, request_count) VALUES ($1, $2, NOW(), 1) ON CONFLICT (scope, subject) DO UPDATE SET window_started_at = CASE WHEN admin_rate_limits.window_started_at <= NOW() - make_interval(secs => $4) THEN NOW() ELSE admin_rate_limits.window_started_at END, request_count = CASE WHEN admin_rate_limits.window_started_at <= NOW() - make_interval(secs => $4) THEN 1 ELSE admin_rate_limits.request_count + 1 END RETURNING request_count <= $3")
    .bind(scope.as_str().as_ref())
        .bind(subject.as_ref())
    .bind(limit.0)
    .bind(window_seconds.0)
        .fetch_one(state.pool.as_ref())
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    if allowed {
        Ok(())
    } else {
        Err(AdminApiEr::RateLimited)
    }
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
    mut connection: SqlxAdminPgConnectionRef<'_>,
    event: AdminAuditSuccessRef<'_>,
) -> Result<(), AdminApiEr> {
    let details = serde_json::json!({ "operation": event.action.as_str().as_ref(), "target_id": event.resource_id.value().as_ref() });
    let _result = sqlx::query(
        "INSERT INTO admin_audit_log (user_id, user_login, action, resource, resource_id, request_id, succeeded, details) VALUES ($1, $2, $3, $4, $5, $6, TRUE, $7)",
    )
    .bind(event.user_id.0)
    .bind(event.login.as_ref())
    .bind(event.action.as_str().as_ref())
    .bind(event.resource.as_str().as_ref())
    .bind(event.resource_id.value().as_ref())
    .bind(uuid::Uuid::new_v4())
    .bind(details)
    .execute(connection.as_mut())
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(())
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
) -> Result<AuthenticatedAdmin, AdminApiEr> {
    let user = sqlx::query_as::<_, (String, String)>(
        "SELECT login, display_name FROM admin_users WHERE id = $1 AND is_banned = FALSE",
    )
    .bind(user_id.0)
    .fetch_optional(state.pool.as_ref())
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?
    .ok_or(AdminApiEr::Authentication)?;
    let roles = sqlx::query_scalar::<_, String>(
        "SELECT role.name FROM admin_roles role JOIN admin_user_roles link ON link.role_id = role.id WHERE link.user_id = $1 ORDER BY role.name",
    )
    .bind(user_id.0)
    .fetch_all(state.pool.as_ref())
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?
    .into_iter()
    .map(super::AdminRoleName::try_from)
    .collect::<Result<Vec<super::AdminRoleName>, _>>()
    .map_err(|_er| AdminApiEr::Authentication)?;
    let permissions = sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT permission.name FROM admin_permissions permission JOIN admin_role_permissions role_permission ON role_permission.permission_id = permission.id JOIN admin_user_roles user_role ON user_role.role_id = role_permission.role_id WHERE user_role.user_id = $1 ORDER BY permission.name",
    )
    .bind(user_id.0)
    .fetch_all(state.pool.as_ref())
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?
    .into_iter()
    .map(|permission| super::AdminPermission::try_from(permission.as_str()))
    .collect::<Result<Vec<super::AdminPermission>, _>>()
    .map_err(|_er| AdminApiEr::Authentication)?;
    Ok(AuthenticatedAdmin {
        display_name: super::AdminDisplayName::try_from(user.1)
            .map_err(|_er| AdminApiEr::Authentication)?,
        id: user_id,
        login: super::AdminLogin::try_from(user.0).map_err(|_er| AdminApiEr::Authentication)?,
        permissions,
        roles,
        session_id,
    })
}
fn append_session_cookies(
    response: &mut AxumAdminResponse,
    state: &AdminAuthSvcState,
    session: &AdminSessionBundle,
) -> Result<(), AdminApiEr> {
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
            .map_err(|er| AdminApiEr::Header(HttpAdminHeaderValueEr::from(er)))
    })
}
fn append_cleared_session_cookies(
    response: &mut AxumAdminResponse,
    state: &AdminAuthSvcState,
) -> Result<(), AdminApiEr> {
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
            .map_err(|er| AdminApiEr::Header(HttpAdminHeaderValueEr::from(er)))
    })
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(post, path = "/auth/sign-in", request_body = AdminSignInReq, responses((status = 200, body = AdminSignInRes), (status = 401, body = AdminApiErBody), (status = 429, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), tag = "admin_auth")]
async fn sign_in(
    auth: AdminAuthReq,
    peer: AdminPeerAddr,
    request_json: AdminSignInJson,
) -> Result<AxumAdminResponse, AdminApiEr> {
    let state = auth.state;
    let headers = auth.headers;
    let request = request_json.0;
    let peer_subject = super::StdAdminString::try_from(peer.0.as_ref().ip().to_string())
        .map_err(|_er| AdminApiEr::Validation)?;
    enforce_rate_limit(
        state.as_ref(),
        AdminRateLimitScope::SignInIp,
        &peer_subject,
        StdAdminRateLimitCount::from(state.as_ref().sign_in_rate_limit.0.saturating_mul(5i64)),
        StdAdminRateLimitWindowSeconds::from(900i32),
    )
    .await?;
    let pair_subject = super::StdAdminString::try_from(format!(
        "{}|{}",
        peer.0.as_ref().ip(),
        request.login.as_ref()
    ))
    .map_err(|_er| AdminApiEr::Validation)?;
    enforce_rate_limit(
        state.as_ref(),
        AdminRateLimitScope::SignInIpLogin,
        &pair_subject,
        state.as_ref().sign_in_rate_limit,
        StdAdminRateLimitWindowSeconds::from(900i32),
    )
    .await?;
    if !origin_is_allowed(
        state.as_ref(),
        super::HttpAdminHeaderMapRef::from(headers.as_ref()),
    )
    .0
    {
        return Err(AdminApiEr::Authentication);
    }
    let recent_failures = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM admin_login_attempts WHERE login = $1 AND succeeded = FALSE AND attempted_at > NOW() - INTERVAL '15 minutes'",
    )
    .bind(request.login.as_ref())
    .fetch_one(state.as_ref().pool.as_ref())
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    if recent_failures >= 10i64 {
        return Err(AdminApiEr::RateLimited);
    }
    let user = sqlx::query_as::<_, (i64, String, bool)>(
        "SELECT id, password_hash, is_banned FROM admin_users WHERE lower(login) = lower($1)",
    )
    .bind(request.login.as_ref())
    .fetch_optional(state.as_ref().pool.as_ref())
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let Some((user_id, password_hash, is_banned)) = user else {
        drop(
            state
                .as_ref()
                .password_hasher
                .hash(request.password)
                .await
                .map_err(AdminApiEr::PasswordHash)?,
        );
        record_login_attempt(
            state.as_ref(),
            &request.login,
            peer,
            super::StdAdminBool::from(false),
        )
        .await?;
        return Err(AdminApiEr::Authentication);
    };
    let verified = state
        .as_ref()
        .password_hasher
        .verify(
            request.password,
            super::AdminPasswordHash::new(pg_types_text_misc::StringAsNnTextSecret::from(
                password_hash,
            )),
        )
        .await
        .map_err(|_er| AdminApiEr::Authentication)?;
    if !verified.0 || is_banned {
        record_login_attempt(
            state.as_ref(),
            &request.login,
            peer,
            super::StdAdminBool::from(false),
        )
        .await?;
        return Err(AdminApiEr::Authentication);
    }
    record_login_attempt(
        state.as_ref(),
        &request.login,
        peer,
        super::StdAdminBool::from(true),
    )
    .await?;
    let admin_user_id = super::AdminUserId::from(user_id);
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let session = create_session_in_connection(
        state.as_ref(),
        admin_user_id,
        SqlxAdminPgConnectionRef::from(&mut *tx),
    )
    .await
    .map_err(AdminApiEr::Session)?;
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::SignIn,
            login: &request.login,
            resource: super::AdminAuditResource::Session,
            resource_id: AdminAuditResourceId::Session(session.session_id()),
            user_id: admin_user_id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let authenticated =
        load_authenticated_admin(state.as_ref(), admin_user_id, session.session_id()).await?;
    let mut response = AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
        AdminSignInRes {
            user: authenticated,
        },
    )));
    append_session_cookies(&mut response, state.as_ref(), &session)?;
    Ok(response)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(get, path = "/auth/me", responses((status = 200, body = AuthenticatedAdmin), (status = 401, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = [])), tag = "admin_auth")]
async fn me(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiEr> {
    authenticate(
        auth.state.as_ref(),
        super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
    )
    .await
    .map(|authenticated| {
        AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            authenticated,
        )))
    })
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(post, path = "/auth/refresh", responses((status = 200, body = AdminSignInRes), (status = 401, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), tag = "admin_auth")]
async fn refresh(auth: AdminAuthReq, peer: AdminPeerAddr) -> Result<AxumAdminResponse, AdminApiEr> {
    let state = auth.state;
    let headers = auth.headers;
    let peer_subject = super::StdAdminString::try_from(peer.0.as_ref().ip().to_string())
        .map_err(|_er| AdminApiEr::Validation)?;
    enforce_rate_limit(
        state.as_ref(),
        AdminRateLimitScope::RefreshIp,
        &peer_subject,
        StdAdminRateLimitCount::from(60i64),
        StdAdminRateLimitWindowSeconds::from(900i32),
    )
    .await?;
    if !origin_is_allowed(
        state.as_ref(),
        super::HttpAdminHeaderMapRef::from(headers.as_ref()),
    )
    .0
    {
        return Err(AdminApiEr::Authentication);
    }
    let raw_token = super::find_admin_cookie(
        super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        super::AdminCookieKind::Refresh,
    )
    .ok_or(AdminApiEr::Authentication)?;
    let token = super::AdminOpaqueToken::new(super::SecrecyAdminString::from(
        secrecy::SecretBox::new(Box::new(raw_token.as_ref().to_owned())),
    ));
    let token_hash = super::hash_opaque_token(&token);
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let user_id = sqlx::query_scalar::<_, i64>(
        "UPDATE admin_refresh_tokens SET revoked_at = NOW() WHERE token_hash = $1 AND revoked_at IS NULL AND expires_at > NOW() RETURNING user_id",
    )
    .bind(secrecy::ExposeSecret::expose_secret(token_hash.0.as_ref()))
    .fetch_optional(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?
    .ok_or(AdminApiEr::Authentication)?;
    let admin_user_id = super::AdminUserId::from(user_id);
    let session = create_session_in_connection(
        state.as_ref(),
        admin_user_id,
        SqlxAdminPgConnectionRef::from(&mut *tx),
    )
    .await
    .map_err(AdminApiEr::Session)?;
    let login = sqlx::query_scalar::<_, String>(
        "SELECT login FROM admin_users WHERE id = $1 AND is_banned = FALSE",
    )
    .bind(admin_user_id.0)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?
    .ok_or(AdminApiEr::Authentication)
    .and_then(|value| super::AdminLogin::try_from(value).map_err(|_er| AdminApiEr::Validation))?;
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::Refresh,
            login: &login,
            resource: super::AdminAuditResource::Session,
            resource_id: AdminAuditResourceId::Session(session.session_id()),
            user_id: admin_user_id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let authenticated =
        load_authenticated_admin(state.as_ref(), admin_user_id, session.session_id()).await?;
    let mut response = AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
        AdminSignInRes {
            user: authenticated,
        },
    )));
    append_session_cookies(&mut response, state.as_ref(), &session)?;
    Ok(response)
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(post, path = "/auth/sign-out", responses((status = 204), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_auth")]
async fn sign_out(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiEr> {
    let state = auth.state;
    let headers = auth.headers;
    let authenticated = authenticate(
        state.as_ref(),
        super::HttpAdminHeaderMapRef::from(headers.as_ref()),
    )
    .await?;
    validate_csrf(
        state.as_ref(),
        super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        &authenticated,
    )
    .await?;
    let mut tx = state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _access_result = sqlx::query(
        "UPDATE admin_access_sessions SET revoked_at = NOW() WHERE id = $1 AND user_id = $2 AND revoked_at IS NULL",
    )
    .bind(authenticated.session_id.0.0)
    .bind(authenticated.id.0)
    .execute(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    if let Some(raw_refresh) = super::find_admin_cookie(
        super::HttpAdminHeaderMapRef::from(headers.as_ref()),
        super::AdminCookieKind::Refresh,
    ) {
        let refresh = super::AdminOpaqueToken::new(super::SecrecyAdminString::from(
            secrecy::SecretBox::new(Box::new(raw_refresh.as_ref().to_owned())),
        ));
        let refresh_hash = super::hash_opaque_token(&refresh);
        let _refresh_result = sqlx::query(
            "UPDATE admin_refresh_tokens SET revoked_at = NOW() WHERE token_hash = $1 AND user_id = $2 AND revoked_at IS NULL",
        )
        .bind(secrecy::ExposeSecret::expose_secret(refresh_hash.0.as_ref()))
        .bind(authenticated.id.0)
        .execute(&mut *tx)
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    }
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::SignOut,
            login: &authenticated.login,
            resource: super::AdminAuditResource::Session,
            resource_id: AdminAuditResourceId::Session(authenticated.session_id),
            user_id: authenticated.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let mut response = AxumAdminResponse(axum::response::IntoResponse::into_response(
        http::StatusCode::NO_CONTENT,
    ));
    append_cleared_session_cookies(&mut response, state.as_ref())?;
    Ok(response)
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
#[utoipa::path(get, path = "/auth/sessions", responses((status = 200, body = [AdminSessionView]), (status = 401, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = [])), tag = "admin_auth")]
async fn sessions(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiEr> {
    let authenticated = authenticate(
        auth.state.as_ref(),
        super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
    )
    .await?;
    sqlx::query_as::<_, (uuid::Uuid, String, String)>(
        "SELECT id, created_at::TEXT, expires_at::TEXT FROM admin_access_sessions WHERE user_id = $1 AND revoked_at IS NULL AND expires_at > NOW() ORDER BY created_at DESC",
    )
    .bind(authenticated.id.0)
    .fetch_all(auth.state.as_ref().pool.as_ref())
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?
    .into_iter()
    .map(|row| {
        Ok(AdminSessionView {
            created_at: AdminSessionTimestamp::try_from(row.1)
                .map_err(|_er| AdminApiEr::Authentication)?,
            expires_at: AdminSessionTimestamp::try_from(row.2)
                .map_err(|_er| AdminApiEr::Authentication)?,
            id: super::AdminSessionId::from(super::UuidAdminValue::from(row.0)),
        })
    })
    .collect::<Result<Vec<AdminSessionView>, AdminApiEr>>()
    .map(|sessions| {
        AxumAdminResponse(axum::response::IntoResponse::into_response(axum::Json(
            sessions,
        )))
    })
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(delete, path = "/auth/sessions/{session_id}", responses((status = 204), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_auth")]
async fn revoke_session(
    auth: AdminAuthReq,
    session: AdminSessionPath,
) -> Result<AxumAdminResponse, AdminApiEr> {
    let authenticated = authenticate(
        auth.state.as_ref(),
        super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
    )
    .await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    validate_csrf(
        auth.state.as_ref(),
        super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        &authenticated,
    )
    .await?;
    let _result = sqlx::query(
        "UPDATE admin_access_sessions SET revoked_at = NOW() WHERE id = $1 AND user_id = $2 AND revoked_at IS NULL",
    )
    .bind(session.0.0.0)
    .bind(authenticated.id.0)
    .execute(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::Delete,
            login: &authenticated.login,
            resource: super::AdminAuditResource::Session,
            resource_id: AdminAuditResourceId::Session(session.0),
            user_id: authenticated.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(delete, path = "/auth/sessions", responses((status = 204), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_auth")]
async fn revoke_all_sessions(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiEr> {
    let authenticated = authenticate(
        auth.state.as_ref(),
        super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
    )
    .await?;
    validate_csrf(
        auth.state.as_ref(),
        super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        &authenticated,
    )
    .await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _access_result = sqlx::query(
        "UPDATE admin_access_sessions SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL",
    )
    .bind(authenticated.id.0)
    .execute(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _refresh_result = sqlx::query(
        "UPDATE admin_refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL",
    )
    .bind(authenticated.id.0)
    .execute(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::Delete,
            login: &authenticated.login,
            resource: super::AdminAuditResource::Session,
            resource_id: AdminAuditResourceId::Session(authenticated.session_id),
            user_id: authenticated.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let mut response = AxumAdminResponse(axum::response::IntoResponse::into_response(
        http::StatusCode::NO_CONTENT,
    ));
    append_cleared_session_cookies(&mut response, auth.state.as_ref())?;
    Ok(response)
}
async fn authorize_custom(
    auth: &AdminAuthReq,
    permission: super::AdminPermission,
) -> Result<AuthenticatedAdmin, AdminApiEr> {
    authorize_generated_request(
        auth.state.as_ref(),
        super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        permission.as_str(),
        super::StdAdminBool::from(true),
    )
    .await
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(post, path = "/users", request_body = AdminCreateUserReq, responses((status = 201, body = AdminCreateUserRes), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 409, body = AdminApiErBody), (status = 422, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_users")]
async fn create_user(
    auth: AdminAuthReq,
    request: AxumAdminJson<AdminCreateUserReq>,
) -> Result<AxumAdminResponse, AdminApiEr> {
    let actor = authorize_custom(&auth, super::AdminPermission::UsersCreate).await?;
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(request.0.password)
        .await
        .map_err(AdminApiEr::PasswordHash)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let user_id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO admin_users (login, display_name, password_hash) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(request.0.login.as_ref())
    .bind(request.0.display_name.as_ref())
    .bind(password_hash.0.as_ref())
    .fetch_one(&mut *tx)
    .await
    .map_err(|er| {
        if er
            .as_database_error()
            .is_some_and(sqlx::error::DatabaseError::is_unique_violation)
        {
            AdminApiEr::Conflict
        } else {
            AdminApiEr::Pg(super::SqlxAdminEr::from(er))
        }
    })?;
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::Create,
            login: &actor.login,
            resource: super::AdminAuditResource::User,
            resource_id: AdminAuditResourceId::User(super::AdminUserId::from(user_id)),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(AdminCreateUserRes {
                id: super::AdminUserId::from(user_id),
            }),
        )),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(patch, path = "/users/{user_id}", request_body = AdminUpdateUserReq, responses((status = 204), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 409, body = AdminApiErBody), (status = 422, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_users")]
async fn update_user(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
    request: AxumAdminJson<AdminUpdateUserReq>,
) -> Result<AxumAdminResponse, AdminApiEr> {
    let actor = authorize_custom(&auth, super::AdminPermission::UsersUpdate).await?;
    if request.0.login.is_none() && request.0.display_name.is_none() {
        return Err(AdminApiEr::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    sqlx::query_scalar::<_, bool>(
        "UPDATE admin_users SET login = COALESCE($2, login), display_name = COALESCE($3, display_name) WHERE id = $1 RETURNING TRUE",
    )
    .bind(path.0.0)
    .bind(request.0.login.as_ref().map(|value| value.as_ref().as_str()))
    .bind(
        request
            .0
            .display_name
            .as_ref()
            .map(|value| value.as_ref().as_str()),
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|er| {
        if er
            .as_database_error()
            .is_some_and(sqlx::error::DatabaseError::is_unique_violation)
        {
            AdminApiEr::Conflict
        } else {
            AdminApiEr::Pg(super::SqlxAdminEr::from(er))
        }
    })?
    .ok_or(AdminApiEr::Conflict)
    .map(drop)?;
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::AdminAuditResource::User,
            resource_id: AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(post, path = "/users/{user_id}/password", request_body = AdminSetUserPasswordReq, responses((status = 204), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 409, body = AdminApiErBody), (status = 422, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_users")]
async fn set_user_password(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
    request: AxumAdminJson<AdminSetUserPasswordReq>,
) -> Result<AxumAdminResponse, AdminApiEr> {
    let actor = authorize_custom(&auth, super::AdminPermission::UsersUpdate).await?;
    let password_hash = auth
        .state
        .as_ref()
        .password_hasher
        .hash(request.0.password)
        .await
        .map_err(AdminApiEr::PasswordHash)?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    sqlx::query_scalar::<_, bool>(
        "UPDATE admin_users SET password_hash = $2 WHERE id = $1 RETURNING TRUE",
    )
    .bind(path.0.0)
    .bind(password_hash.0.as_ref())
    .fetch_optional(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?
    .ok_or(AdminApiEr::Conflict)
    .map(drop)?;
    let _access = sqlx::query(
        "UPDATE admin_access_sessions SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL",
    )
    .bind(path.0.0)
    .execute(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _refresh = sqlx::query(
        "UPDATE admin_refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL",
    )
    .bind(path.0.0)
    .execute(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::AdminAuditResource::User,
            resource_id: AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(post, path = "/users/{user_id}/ban", request_body = AdminSetUserBanReq, responses((status = 204), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 409, body = AdminApiErBody), (status = 422, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_users")]
async fn set_user_ban(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
    request: AxumAdminJson<AdminSetUserBanReq>,
) -> Result<AxumAdminResponse, AdminApiEr> {
    let actor = authorize_custom(&auth, super::AdminPermission::UsersUpdate).await?;
    if request.0.is_banned.0 && actor.id == path.0 {
        return Err(AdminApiEr::Conflict);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _lock =
        sqlx::query("SELECT pg_advisory_xact_lock(hashtext('admin_last_active_administrator'))")
            .execute(&mut *tx)
            .await
            .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    if request.0.is_banned.0 {
        let target_is_admin = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (SELECT 1 FROM admin_user_roles user_role JOIN admin_roles role ON role.id = user_role.role_id WHERE user_role.user_id = $1 AND role.name = 'admin')",
        )
        .bind(path.0.0)
        .fetch_one(&mut *tx)
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
        let active_admin_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(DISTINCT users.id) FROM admin_users users JOIN admin_user_roles user_role ON user_role.user_id = users.id JOIN admin_roles role ON role.id = user_role.role_id WHERE role.name = 'admin' AND users.is_banned = FALSE",
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
        if target_is_admin && active_admin_count <= 1i64 {
            return Err(AdminApiEr::Conflict);
        }
    }
    sqlx::query_scalar::<_, bool>(
        "UPDATE admin_users SET is_banned = $2 WHERE id = $1 RETURNING TRUE",
    )
    .bind(path.0.0)
    .bind(request.0.is_banned.0)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?
    .ok_or(AdminApiEr::Conflict)
    .map(drop)?;
    if request.0.is_banned.0 {
        let _access = sqlx::query(
            "UPDATE admin_access_sessions SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL",
        )
        .bind(path.0.0)
        .execute(&mut *tx)
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
        let _refresh = sqlx::query(
            "UPDATE admin_refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL",
        )
        .bind(path.0.0)
        .execute(&mut *tx)
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    }
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::AdminAuditResource::User,
            resource_id: AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(delete, path = "/users/{user_id}", responses((status = 204), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 409, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_users")]
async fn delete_user(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
) -> Result<AxumAdminResponse, AdminApiEr> {
    let actor = authorize_custom(&auth, super::AdminPermission::UsersDelete).await?;
    if actor.id == path.0 {
        return Err(AdminApiEr::Conflict);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _lock =
        sqlx::query("SELECT pg_advisory_xact_lock(hashtext('admin_last_active_administrator'))")
            .execute(&mut *tx)
            .await
            .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let target_is_admin = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (SELECT 1 FROM admin_user_roles user_role JOIN admin_roles role ON role.id = user_role.role_id WHERE user_role.user_id = $1 AND role.name = 'admin')",
    )
    .bind(path.0.0)
    .fetch_one(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let active_admin_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(DISTINCT users.id) FROM admin_users users JOIN admin_user_roles user_role ON user_role.user_id = users.id JOIN admin_roles role ON role.id = user_role.role_id WHERE role.name = 'admin' AND users.is_banned = FALSE",
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    if target_is_admin && active_admin_count <= 1i64 {
        return Err(AdminApiEr::Conflict);
    }
    sqlx::query_scalar::<_, bool>("DELETE FROM admin_users WHERE id = $1 RETURNING TRUE")
        .bind(path.0.0)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?
        .ok_or(AdminApiEr::Conflict)
        .map(drop)?;
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::Delete,
            login: &actor.login,
            resource: super::AdminAuditResource::User,
            resource_id: AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(post, path = "/roles", request_body = AdminCreateRoleReq, responses((status = 201, body = AdminCreateRoleRes), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 409, body = AdminApiErBody), (status = 422, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_roles")]
async fn create_role(
    auth: AdminAuthReq,
    request: AxumAdminJson<AdminCreateRoleReq>,
) -> Result<AxumAdminResponse, AdminApiEr> {
    let actor = authorize_custom(&auth, super::AdminPermission::RolesCreate).await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let role_id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO admin_roles (name, is_system) VALUES ($1, FALSE) RETURNING id",
    )
    .bind(request.0.name.as_ref())
    .fetch_one(&mut *tx)
    .await
    .map_err(|er| {
        if er
            .as_database_error()
            .is_some_and(sqlx::error::DatabaseError::is_unique_violation)
        {
            AdminApiEr::Conflict
        } else {
            AdminApiEr::Pg(super::SqlxAdminEr::from(er))
        }
    })?;
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::Create,
            login: &actor.login,
            resource: super::AdminAuditResource::Role,
            resource_id: AdminAuditResourceId::Role(super::AdminRoleId::from(role_id)),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response((
            http::StatusCode::CREATED,
            axum::Json(AdminCreateRoleRes {
                id: super::AdminRoleId::from(role_id),
            }),
        )),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(patch, path = "/roles/{role_id}", request_body = AdminUpdateRoleReq, responses((status = 204), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 409, body = AdminApiErBody), (status = 422, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_roles")]
async fn update_role(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminRoleId>,
    request: AxumAdminJson<AdminUpdateRoleReq>,
) -> Result<AxumAdminResponse, AdminApiEr> {
    let actor = authorize_custom(&auth, super::AdminPermission::RolesUpdate).await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    sqlx::query_scalar::<_, bool>(
        "UPDATE admin_roles SET name = $2 WHERE id = $1 AND is_system = FALSE RETURNING TRUE",
    )
    .bind(path.0.0)
    .bind(request.0.name.as_ref())
    .fetch_optional(&mut *tx)
    .await
    .map_err(|er| {
        if er
            .as_database_error()
            .is_some_and(sqlx::error::DatabaseError::is_unique_violation)
        {
            AdminApiEr::Conflict
        } else {
            AdminApiEr::Pg(super::SqlxAdminEr::from(er))
        }
    })?
    .ok_or(AdminApiEr::Conflict)
    .map(drop)?;
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::AdminAuditResource::Role,
            resource_id: AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(delete, path = "/roles/{role_id}", responses((status = 204), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 409, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_roles")]
async fn delete_role(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminRoleId>,
) -> Result<AxumAdminResponse, AdminApiEr> {
    let actor = authorize_custom(&auth, super::AdminPermission::RolesDelete).await?;
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    sqlx::query_scalar::<_, bool>(
        "DELETE FROM admin_roles WHERE id = $1 AND is_system = FALSE RETURNING TRUE",
    )
    .bind(path.0.0)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?
    .ok_or(AdminApiEr::Conflict)
    .map(drop)?;
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::Delete,
            login: &actor.login,
            resource: super::AdminAuditResource::Role,
            resource_id: AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(put, path = "/roles/{role_id}/permissions", request_body = AdminSetRolePermissionsReq, responses((status = 204), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 409, body = AdminApiErBody), (status = 422, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_roles")]
async fn set_role_permissions(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminRoleId>,
    request: AxumAdminJson<AdminSetRolePermissionsReq>,
) -> Result<AxumAdminResponse, AdminApiEr> {
    let actor = authorize_custom(&auth, super::AdminPermission::RolePermissionsUpdate).await?;
    if request
        .0
        .permission_ids
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len()
        != request.0.permission_ids.len()
    {
        return Err(AdminApiEr::Validation);
    }
    let permission_ids = request
        .0
        .permission_ids
        .into_iter()
        .map(|permission_id| permission_id.0)
        .collect::<Vec<i64>>();
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let role_is_system =
        sqlx::query_scalar::<_, bool>("SELECT is_system FROM admin_roles WHERE id = $1 FOR UPDATE")
            .bind(path.0.0)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?
            .ok_or(AdminApiEr::Conflict)?;
    if role_is_system {
        return Err(AdminApiEr::Conflict);
    }
    let existing_permissions =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM admin_permissions WHERE id = ANY($1)")
            .bind(&permission_ids)
            .fetch_one(&mut *tx)
            .await
            .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    if usize::try_from(existing_permissions).ok() != Some(permission_ids.len()) {
        return Err(AdminApiEr::Validation);
    }
    let _deleted = sqlx::query("DELETE FROM admin_role_permissions WHERE role_id = $1")
        .bind(path.0.0)
        .execute(&mut *tx)
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _inserted = sqlx::query(
        "INSERT INTO admin_role_permissions (role_id, permission_id) SELECT $1, permission_id FROM UNNEST($2::BIGINT[]) AS permission_id",
    )
    .bind(path.0.0)
    .bind(&permission_ids)
    .execute(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::AdminAuditResource::Role,
            resource_id: AdminAuditResourceId::Role(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(put, path = "/users/{user_id}/roles", request_body = AdminSetUserRolesReq, responses((status = 204), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 409, body = AdminApiErBody), (status = 422, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_users")]
async fn set_user_roles(
    auth: AdminAuthReq,
    path: AxumAdminPath<super::AdminUserId>,
    request: AxumAdminJson<AdminSetUserRolesReq>,
) -> Result<AxumAdminResponse, AdminApiEr> {
    let actor = authorize_custom(&auth, super::AdminPermission::UserRolesUpdate).await?;
    if request
        .0
        .role_ids
        .iter()
        .collect::<std::collections::HashSet<_>>()
        .len()
        != request.0.role_ids.len()
    {
        return Err(AdminApiEr::Validation);
    }
    let role_ids = request
        .0
        .role_ids
        .into_iter()
        .map(|role_id| role_id.0)
        .collect::<Vec<i64>>();
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _lock =
        sqlx::query("SELECT pg_advisory_xact_lock(hashtext('admin_last_active_administrator'))")
            .execute(&mut *tx)
            .await
            .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let target_is_active = sqlx::query_scalar::<_, bool>(
        "SELECT NOT is_banned FROM admin_users WHERE id = $1 FOR UPDATE",
    )
    .bind(path.0.0)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?
    .ok_or(AdminApiEr::Conflict)?;
    let existing_roles =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM admin_roles WHERE id = ANY($1)")
            .bind(&role_ids)
            .fetch_one(&mut *tx)
            .await
            .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    if usize::try_from(existing_roles).ok() != Some(role_ids.len()) {
        return Err(AdminApiEr::Validation);
    }
    let admin_role_id = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM admin_roles WHERE name = 'admin' AND is_system = TRUE",
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let target_was_admin = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (SELECT 1 FROM admin_user_roles WHERE user_id = $1 AND role_id = $2)",
    )
    .bind(path.0.0)
    .bind(admin_role_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    if target_is_active && target_was_admin && !role_ids.contains(&admin_role_id) {
        let active_admin_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(DISTINCT users.id) FROM admin_users users JOIN admin_user_roles user_role ON user_role.user_id = users.id WHERE user_role.role_id = $1 AND users.is_banned = FALSE",
        )
        .bind(admin_role_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
        if active_admin_count <= 1i64 {
            return Err(AdminApiEr::Conflict);
        }
    }
    let _deleted = sqlx::query("DELETE FROM admin_user_roles WHERE user_id = $1")
        .bind(path.0.0)
        .execute(&mut *tx)
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _inserted = sqlx::query(
        "INSERT INTO admin_user_roles (user_id, role_id) SELECT $1, role_id FROM UNNEST($2::BIGINT[]) AS role_id",
    )
    .bind(path.0.0)
    .bind(&role_ids)
    .execute(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _access = sqlx::query(
        "UPDATE admin_access_sessions SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL",
    )
    .bind(path.0.0)
    .execute(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _refresh = sqlx::query(
        "UPDATE admin_refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL",
    )
    .bind(path.0.0)
    .execute(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::AdminAuditResource::User,
            resource_id: AdminAuditResourceId::User(path.0),
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(get, path = "/audit-log", params(AdminAuditQuery), responses((status = 200, body = [AdminAuditView]), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 422, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = [])), tag = "admin_audit")]
async fn audit_log(
    auth: AdminAuthReq,
    query: AxumAdminQuery<AdminAuditQuery>,
) -> Result<AxumAdminResponse, AdminApiEr> {
    let actor = authorize_generated_request(
        auth.state.as_ref(),
        super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        super::AdminPermission::AuditLogRead.as_str(),
        super::StdAdminBool::from(false),
    )
    .await?;
    let rate_subject = super::StdAdminString::try_from(actor.id.0.to_string())
        .map_err(|_er| AdminApiEr::Validation)?;
    enforce_rate_limit(
        auth.state.as_ref(),
        AdminRateLimitScope::AuditRead,
        &rate_subject,
        StdAdminRateLimitCount::from(60i64),
        StdAdminRateLimitWindowSeconds::from(60i32),
    )
    .await?;
    let action = query.0.action.map(super::AdminAuditAction::as_str);
    let resource = query.0.resource.map(super::AdminAuditResource::as_str);
    let rows = sqlx::query_as::<
        _,
        (
            i64,
            Option<i64>,
            Option<String>,
            String,
            String,
            Option<String>,
            bool,
            Option<serde_json::Value>,
            String,
        ),
    >(
        "SELECT id, user_id, user_login, action, resource, resource_id, succeeded, details, created_at::TEXT FROM admin_audit_log WHERE ($1::BIGINT IS NULL OR user_id = $1) AND ($2::TEXT IS NULL OR action = $2) AND ($3::TEXT IS NULL OR resource = $3) AND ($4::TIMESTAMPTZ IS NULL OR created_at >= $4::TIMESTAMPTZ) AND ($5::TIMESTAMPTZ IS NULL OR created_at <= $5::TIMESTAMPTZ) ORDER BY created_at DESC LIMIT 200",
    )
    .bind(query.0.user_id.map(|user_id| user_id.0))
    .bind(action.map(|value| value.as_ref().to_owned()))
    .bind(resource.map(|value| value.as_ref().to_owned()))
    .bind(query.0.created_after.as_ref().map(|value| value.as_ref().as_str()))
    .bind(query.0.created_before.as_ref().map(|value| value.as_ref().as_str()))
    .fetch_all(auth.state.as_ref().pool.as_ref())
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let views = rows
        .into_iter()
        .map(|row| {
            Ok(AdminAuditView {
                action: row.3.parse().map_err(|_er| AdminApiEr::Validation)?,
                created_at: AdminAuditTimestamp::try_from(row.8)
                    .map_err(|_er| AdminApiEr::Validation)?,
                details: row.7.map(SerdeJsonAdminValue::from),
                id: super::AdminAuditLogId::from(row.0),
                resource: row.4.parse().map_err(|_er| AdminApiEr::Validation)?,
                resource_id: row
                    .5
                    .map(super::StdAdminString::try_from)
                    .transpose()
                    .map_err(|_er| AdminApiEr::Validation)?,
                succeeded: super::StdAdminBool::from(row.6),
                user_id: row.1.map(super::AdminUserId::from),
                user_login: row
                    .2
                    .map(super::AdminLogin::try_from)
                    .transpose()
                    .map_err(|_er| AdminApiEr::Validation)?,
            })
        })
        .collect::<Result<Vec<AdminAuditView>, AdminApiEr>>()?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(axum::Json(views)),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(patch, path = "/system-settings", request_body = AdminUpdateSettingsReq, responses((status = 204), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 409, body = AdminApiErBody), (status = 422, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = []), ("admin_csrf" = [])), tag = "admin_settings")]
async fn update_settings(
    auth: AdminAuthReq,
    request: AxumAdminJson<AdminUpdateSettingsReq>,
) -> Result<AxumAdminResponse, AdminApiEr> {
    let actor = authorize_custom(&auth, super::AdminPermission::SystemSettingsUpdate).await?;
    let has_field = [
        request.0.default_admin_route.is_some(),
        request.0.main_logo.is_some(),
        request.0.organization_contacts.is_some(),
        request.0.organization_name.is_some(),
        request.0.primary_color.is_some(),
        request.0.site_name.is_some(),
        request.0.support_url.is_some(),
        request.0.tab_title.is_some(),
    ]
    .into_iter()
    .any(std::convert::identity);
    let site_name_is_valid = request
        .0
        .site_name
        .as_ref()
        .is_none_or(|value| !value.as_ref().trim().is_empty());
    let route_is_valid = request
        .0
        .default_admin_route
        .as_ref()
        .is_none_or(|value| value.as_ref().starts_with("/admin"));
    if !has_field || !site_name_is_valid || !route_is_valid {
        return Err(AdminApiEr::Validation);
    }
    let mut tx = auth
        .state
        .as_ref()
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    sqlx::query_scalar::<_, bool>(
        "UPDATE admin_system_settings SET site_name = COALESCE($1, site_name), tab_title = COALESCE($2, tab_title), main_logo = COALESCE($3, main_logo), primary_color = COALESCE($4, primary_color), default_admin_route = COALESCE($5, default_admin_route), organization_name = COALESCE($6, organization_name), organization_contacts = COALESCE($7, organization_contacts), support_url = COALESCE($8, support_url) WHERE id = 1 RETURNING TRUE",
    )
    .bind(request.0.site_name.as_ref().map(|value| value.as_ref().as_str()))
    .bind(request.0.tab_title.as_ref().map(|value| value.as_ref().as_str()))
    .bind(request.0.main_logo.as_ref().map(|value| value.as_ref().as_str()))
    .bind(request.0.primary_color.as_ref().map(|value| value.as_ref().as_str()))
    .bind(request.0.default_admin_route.as_ref().map(|value| value.as_ref().as_str()))
    .bind(request.0.organization_name.as_ref().map(|value| value.as_ref().as_str()))
    .bind(request.0.organization_contacts.as_ref().map(|value| value.as_ref().as_str()))
    .bind(request.0.support_url.as_ref().map(|value| value.as_ref().as_str()))
    .fetch_optional(&mut *tx)
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?
    .ok_or(AdminApiEr::Conflict)
    .map(drop)?;
    record_audit_success_in_connection(
        SqlxAdminPgConnectionRef::from(&mut *tx),
        AdminAuditSuccessRef {
            action: super::AdminAuditAction::Update,
            login: &actor.login,
            resource: super::AdminAuditResource::SystemSettings,
            resource_id: AdminAuditResourceId::SystemSettings,
            user_id: actor.id,
        },
    )
    .await?;
    tx.commit()
        .await
        .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(http::StatusCode::NO_CONTENT),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(get, path = "/users", responses((status = 200, body = [AdminUserSummary]), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = [])), tag = "admin_users")]
async fn list_users(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiEr> {
    let _actor = authorize_generated_request(
        auth.state.as_ref(),
        super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        super::AdminPermission::UsersRead.as_str(),
        super::StdAdminBool::from(false),
    )
    .await?;
    let rows = sqlx::query_as::<_, (i64, String, String, bool)>(
        "SELECT id, login, display_name, is_banned FROM admin_users ORDER BY login LIMIT 500",
    )
    .fetch_all(auth.state.as_ref().pool.as_ref())
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let users = rows
        .into_iter()
        .map(|row| {
            Ok(AdminUserSummary {
                display_name: super::AdminDisplayName::try_from(row.2)
                    .map_err(|_er| AdminApiEr::Validation)?,
                id: super::AdminUserId::from(row.0),
                is_banned: super::StdAdminBool::from(row.3),
                login: super::AdminLogin::try_from(row.1).map_err(|_er| AdminApiEr::Validation)?,
            })
        })
        .collect::<Result<Vec<AdminUserSummary>, AdminApiEr>>()?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(axum::Json(users)),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(get, path = "/roles", responses((status = 200, body = [AdminRoleSummary]), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = [])), tag = "admin_roles")]
async fn list_roles(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiEr> {
    let _actor = authorize_generated_request(
        auth.state.as_ref(),
        super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        super::AdminPermission::RolesRead.as_str(),
        super::StdAdminBool::from(false),
    )
    .await?;
    let rows = sqlx::query_as::<_, (i64, String, bool)>(
        "SELECT id, name, is_system FROM admin_roles ORDER BY name",
    )
    .fetch_all(auth.state.as_ref().pool.as_ref())
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let roles = rows
        .into_iter()
        .map(|row| {
            Ok(AdminRoleSummary {
                id: super::AdminRoleId::from(row.0),
                is_system: super::StdAdminBool::from(row.2),
                name: super::AdminRoleName::try_from(row.1)
                    .map_err(|_er| AdminApiEr::Validation)?,
            })
        })
        .collect::<Result<Vec<AdminRoleSummary>, AdminApiEr>>()?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(axum::Json(roles)),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(get, path = "/permissions", responses((status = 200, body = [AdminPermissionSummary]), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = [])), tag = "admin_roles")]
async fn list_permissions(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiEr> {
    let _actor = authorize_generated_request(
        auth.state.as_ref(),
        super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        super::AdminPermission::PermissionsRead.as_str(),
        super::StdAdminBool::from(false),
    )
    .await?;
    let rows =
        sqlx::query_as::<_, (i64, String)>("SELECT id, name FROM admin_permissions ORDER BY name")
            .fetch_all(auth.state.as_ref().pool.as_ref())
            .await
            .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let permissions = rows
        .into_iter()
        .map(|row| {
            Ok(AdminPermissionSummary {
                id: super::AdminPermissionId::from(row.0),
                name: super::AdminPermission::try_from(row.1.as_str())
                    .map_err(|_er| AdminApiEr::Validation)?,
            })
        })
        .collect::<Result<Vec<AdminPermissionSummary>, AdminApiEr>>()?;
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(axum::Json(permissions)),
    ))
}
#[allow(clippy::single_call_fn)] // Axum route handler is registered once by the route inventory
#[utoipa::path(get, path = "/system-settings", responses((status = 200, body = AdminSettingsView), (status = 401, body = AdminApiErBody), (status = 403, body = AdminApiErBody), (status = 500, body = AdminApiErBody)), security(("admin_cookie" = [])), tag = "admin_settings")]
async fn settings(auth: AdminAuthReq) -> Result<AxumAdminResponse, AdminApiEr> {
    let _actor = authorize_generated_request(
        auth.state.as_ref(),
        super::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        super::AdminPermission::SystemSettingsRead.as_str(),
        super::StdAdminBool::from(false),
    )
    .await?;
    let row = sqlx::query_as::<
        _,
        (
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    >(
        "SELECT site_name, tab_title, main_logo, primary_color, default_admin_route, organization_name, organization_contacts, support_url FROM admin_system_settings WHERE id = 1",
    )
    .fetch_one(auth.state.as_ref().pool.as_ref())
    .await
    .map_err(|er| AdminApiEr::Pg(super::SqlxAdminEr::from(er)))?;
    let view = AdminSettingsView {
        default_admin_route: AdminSettingText::try_from(row.4)
            .map_err(|_er| AdminApiEr::Validation)?,
        main_logo: row
            .2
            .map(AdminSettingText::try_from)
            .transpose()
            .map_err(|_er| AdminApiEr::Validation)?,
        organization_contacts: row
            .6
            .map(AdminSettingText::try_from)
            .transpose()
            .map_err(|_er| AdminApiEr::Validation)?,
        organization_name: row
            .5
            .map(AdminSettingText::try_from)
            .transpose()
            .map_err(|_er| AdminApiEr::Validation)?,
        primary_color: row
            .3
            .map(AdminSettingText::try_from)
            .transpose()
            .map_err(|_er| AdminApiEr::Validation)?,
        site_name: AdminSettingText::try_from(row.0).map_err(|_er| AdminApiEr::Validation)?,
        support_url: row
            .7
            .map(AdminSettingText::try_from)
            .transpose()
            .map_err(|_er| AdminApiEr::Validation)?,
        tab_title: row
            .1
            .map(AdminSettingText::try_from)
            .transpose()
            .map_err(|_er| AdminApiEr::Validation)?,
    };
    Ok(AxumAdminResponse(
        axum::response::IntoResponse::into_response(axum::Json(view)),
    ))
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
#[allow(clippy::needless_for_each)] // utoipa 4 generated component registration uses iterator callbacks
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(sign_in, refresh, sign_out, me, sessions, revoke_session, revoke_all_sessions, list_users, create_user, update_user, set_user_password, set_user_ban, delete_user, set_user_roles, list_roles, create_role, update_role, delete_role, set_role_permissions, list_permissions, audit_log, settings, update_settings),
    components(schemas(AdminSignInReq, AdminSignInRes, AuthenticatedAdmin, AdminSessionView, AdminApiErBody, AdminApiErCode, AdminCreateUserReq, AdminCreateUserRes, AdminUpdateUserReq, AdminSetUserPasswordReq, AdminSetUserBanReq, AdminSetUserRolesReq, AdminCreateRoleReq, AdminCreateRoleRes, AdminUpdateRoleReq, AdminSetRolePermissionsReq, AdminAuditView, AdminAuditTimestamp, SerdeJsonAdminValue, AdminUpdateSettingsReq, AdminSettingText, AdminUserSummary, AdminRoleSummary, AdminPermissionSummary, AdminSettingsView, super::AdminPassword, super::AdminLogin, super::AdminDisplayName, super::AdminRoleName, super::AdminUserId, super::AdminRoleId, super::AdminPermissionId, super::AdminPermission, super::AdminSessionId, super::AdminAuditLogId, super::AdminAuditAction, super::AdminAuditResource)),
    tags((name = "admin_auth", description = "Administrator authentication and sessions"), (name = "admin_users", description = "Administrator user security operations"), (name = "admin_roles", description = "Administrator role security operations"), (name = "admin_audit", description = "Administrator audit log"), (name = "admin_settings", description = "Administrator system settings"))
)]
struct AdminAuthOpenApi;
#[must_use]
pub fn open_api() -> UtoipaAdminAuthOpenApi {
    let mut document = <AdminAuthOpenApi as utoipa::OpenApi>::openapi();
    document
        .paths
        .paths
        .values_mut()
        .flat_map(|path| path.operations.values_mut())
        .for_each(|operation| {
            let _response = operation
                .responses
                .responses
                .entry("429".to_owned())
                .or_insert_with(|| {
                    utoipa::openapi::RefOr::T(utoipa::openapi::response::Response::new(
                        "Request rate limit exceeded",
                    ))
                });
        });
    if let Some(components) = document.components.as_mut() {
        components.add_security_scheme(
            "admin_cookie",
            utoipa::openapi::security::SecurityScheme::ApiKey(
                utoipa::openapi::security::ApiKey::Cookie(
                    utoipa::openapi::security::ApiKeyValue::with_description(
                        "admin_access_token",
                        "HttpOnly administrator access token cookie",
                    ),
                ),
            ),
        );
        components.add_security_scheme(
            "admin_csrf",
            utoipa::openapi::security::SecurityScheme::ApiKey(
                utoipa::openapi::security::ApiKey::Header(
                    utoipa::openapi::security::ApiKeyValue::with_description(
                        "X-CSRF-Token",
                        "CSRF token bound to the administrator access session",
                    ),
                ),
            ),
        );
    }
    UtoipaAdminAuthOpenApi(document)
}
#[must_use]
pub fn routes(state: StdSharedAdminAuthSvcState) -> AxumAdminAuthRouter {
    AxumAdminAuthRouter(
        axum::Router::new()
            .route("/auth/sign-in", axum::routing::post(sign_in))
            .route("/auth/refresh", axum::routing::post(refresh))
            .route("/auth/sign-out", axum::routing::post(sign_out))
            .route("/auth/me", axum::routing::get(me))
            .route(
                "/auth/sessions",
                axum::routing::get(sessions).delete(revoke_all_sessions),
            )
            .route(
                "/auth/sessions/{session_id}",
                axum::routing::delete(revoke_session),
            )
            .route("/users", axum::routing::get(list_users).post(create_user))
            .route(
                "/users/{user_id}",
                axum::routing::patch(update_user).delete(delete_user),
            )
            .route(
                "/users/{user_id}/password",
                axum::routing::post(set_user_password),
            )
            .route("/users/{user_id}/ban", axum::routing::post(set_user_ban))
            .route("/roles", axum::routing::get(list_roles).post(create_role))
            .route(
                "/roles/{role_id}",
                axum::routing::patch(update_role).delete(delete_role),
            )
            .route(
                "/roles/{role_id}/permissions",
                axum::routing::put(set_role_permissions),
            )
            .route("/users/{user_id}/roles", axum::routing::put(set_user_roles))
            .route("/permissions", axum::routing::get(list_permissions))
            .route("/audit-log", axum::routing::get(audit_log))
            .route(
                "/system-settings",
                axum::routing::get(settings).patch(update_settings),
            )
            .with_state(state),
    )
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
    ) -> Result<Self, AdminAuthSvcStateBuildEr> {
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
                .map_err(|_er| AdminAuthSvcStateBuildEr::Audience)?,
            cookie_secure: super::AdminCookieSecure::from(**cookie_secure),
            decoding_key: JsonwebtokenAdminDecodingKey(jsonwebtoken::DecodingKey::from_secret(
                secret.as_bytes(),
            )),
            encoding_key: JsonwebtokenAdminEncodingKey(jsonwebtoken::EncodingKey::from_secret(
                secret.as_bytes(),
            )),
            issuer: super::AdminTokenIssuer::try_from(issuer.as_ref().clone())
                .map_err(|_er| AdminAuthSvcStateBuildEr::Issuer)?,
            password_hasher: super::AdminPasswordHasher::new(
                super::AdminPasswordHashConcurrency::from(super::StdAdminNonZeroUsize::from(
                    std::num::NonZeroUsize::new(password_hash_concurrency.get())
                        .ok_or(AdminAuthSvcStateBuildEr::Issuer)?,
                )),
            ),
            pool,
            refresh_ttl: StdAdminRefreshTtlSeconds::from(refresh_ttl.get()),
            session_limit: StdAdminSessionLimit::from(session_limit.get()),
            sign_in_rate_limit: StdAdminRateLimitCount::from(
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
pub enum AdminSessionEr {
    #[error("administrator access token creation failed: {0:?}")]
    AccessToken(super::AdminAccessTokenEr),
    #[error("administrator session database operation failed: {0:?}")]
    Pg(super::SqlxAdminEr),
    #[error("system clock is before the Unix epoch")]
    SystemClock,
}
#[allow(clippy::single_call_fn)] // clock failure mapping remains isolated from session persistence
fn unix_now() -> Result<super::AdminUnixTs, AdminSessionEr> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| super::AdminUnixTs::from(duration.as_secs()))
        .map_err(|_er| AdminSessionEr::SystemClock)
}
#[allow(clippy::single_call_fn)] // token identifier conversion keeps secret construction explicit
fn opaque_token_from_uuid(value: super::UuidAdminValue) -> super::AdminOpaqueToken {
    super::AdminOpaqueToken::new(super::SecrecyAdminString::from(secrecy::SecretBox::new(
        Box::new(value.0.to_string()),
    )))
}
async fn create_session_in_connection(
    state: &AdminAuthSvcState,
    user_id: super::AdminUserId,
    mut connection: SqlxAdminPgConnectionRef<'_>,
) -> Result<AdminSessionBundle, AdminSessionEr> {
    let now = unix_now()?;
    let session_uuid = uuid::Uuid::new_v4();
    let session_id = super::AdminSessionId::from(super::UuidAdminValue::from(session_uuid));
    let refresh_id = uuid::Uuid::new_v4();
    let refresh_generated = super::AdminGeneratedToken::generate();
    let refresh_token = super::AdminRefreshToken::new(super::AdminOpaqueToken::new(
        super::SecrecyAdminString::from(secrecy::SecretBox::new(Box::new(
            secrecy::ExposeSecret::expose_secret(refresh_generated.token().0.as_ref()).to_owned(),
        ))),
    ));
    let csrf_generated = super::AdminGeneratedToken::generate();
    let token_identifier_hash = super::hash_opaque_token(&opaque_token_from_uuid(
        super::UuidAdminValue::from(session_uuid),
    ));
    let expires_at = super::AdminUnixTs::from(now.0.saturating_add(state.access_ttl.0));
    let claims = super::AdminAccessClaims::new(
        user_id,
        session_id,
        now,
        expires_at,
        state.issuer.clone(),
        state.audience.clone(),
    );
    let access_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &state.encoding_key.0,
    )
    .map(super::StdAdminAccessToken)
    .map_err(|er| {
        AdminSessionEr::AccessToken(super::AdminAccessTokenEr(super::JsonwebtokenAdminEr::from(
            er,
        )))
    })?;
    let session_offset =
        i64::try_from(state.session_limit.0.saturating_sub(1usize)).unwrap_or(i64::MAX);
    let _expired_access = sqlx::query("UPDATE admin_access_sessions SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL AND id IN (SELECT id FROM admin_access_sessions WHERE user_id = $1 AND revoked_at IS NULL ORDER BY created_at DESC OFFSET $2)")
        .bind(user_id.0)
        .bind(session_offset)
        .execute(connection.as_mut())
        .await
        .map_err(|er| AdminSessionEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _expired_refresh = sqlx::query("UPDATE admin_refresh_tokens SET revoked_at = NOW() WHERE user_id = $1 AND revoked_at IS NULL AND id IN (SELECT id FROM admin_refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL ORDER BY created_at DESC OFFSET $2)")
        .bind(user_id.0)
        .bind(session_offset)
        .execute(connection.as_mut())
        .await
        .map_err(|er| AdminSessionEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _access_result = sqlx::query(
        "INSERT INTO admin_access_sessions (id, user_id, token_identifier_hash, csrf_token_hash, expires_at) VALUES ($1, $2, $3, $4, NOW() + ($5 * INTERVAL '1 second'))",
    )
    .bind(session_uuid)
    .bind(user_id.0)
    .bind(secrecy::ExposeSecret::expose_secret(token_identifier_hash.0.as_ref()))
    .bind(secrecy::ExposeSecret::expose_secret(csrf_generated.hash().0.as_ref()))
    .bind(i64::try_from(state.access_ttl.0).unwrap_or(i64::MAX))
    .execute(connection.as_mut())
    .await
    .map_err(|er| AdminSessionEr::Pg(super::SqlxAdminEr::from(er)))?;
    let _refresh_result = sqlx::query(
        "INSERT INTO admin_refresh_tokens (id, user_id, token_hash, expires_at) VALUES ($1, $2, $3, NOW() + ($4 * INTERVAL '1 second'))",
    )
    .bind(refresh_id)
    .bind(user_id.0)
    .bind(secrecy::ExposeSecret::expose_secret(refresh_generated.hash().0.as_ref()))
    .bind(i64::try_from(state.refresh_ttl.0).unwrap_or(i64::MAX))
    .execute(connection.as_mut())
    .await
    .map_err(|er| AdminSessionEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(AdminSessionBundle {
        access_token,
        csrf_token: super::AdminOpaqueToken::new(super::SecrecyAdminString::from(
            secrecy::SecretBox::new(Box::new(
                secrecy::ExposeSecret::expose_secret(csrf_generated.token().0.as_ref()).to_owned(),
            )),
        )),
        refresh_token,
        session_id,
    })
}
pub async fn create_session(
    state: &AdminAuthSvcState,
    user_id: super::AdminUserId,
) -> Result<AdminSessionBundle, AdminSessionEr> {
    let mut tx = state
        .pool
        .as_ref()
        .begin()
        .await
        .map_err(|er| AdminSessionEr::Pg(super::SqlxAdminEr::from(er)))?;
    let session =
        create_session_in_connection(state, user_id, SqlxAdminPgConnectionRef::from(&mut *tx))
            .await?;
    tx.commit()
        .await
        .map_err(|er| AdminSessionEr::Pg(super::SqlxAdminEr::from(er)))?;
    Ok(session)
}
#[cfg(test)]
mod tests {
    #[test]
    fn rate_limit_scopes_are_distinct() {
        let scopes = [
            super::AdminRateLimitScope::AuditRead,
            super::AdminRateLimitScope::Mutation,
            super::AdminRateLimitScope::RefreshIp,
            super::AdminRateLimitScope::SignInIp,
            super::AdminRateLimitScope::SignInIpLogin,
        ]
        .map(super::AdminRateLimitScope::as_str);
        let unique = scopes.into_iter().collect::<std::collections::HashSet<_>>();
        assert_eq!(unique.len(), 5usize);
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
                .all(|path| path.as_object().is_some_and(|operations| operations
                    .values()
                    .all(|operation| operation.pointer("/responses/429").is_some())))
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
