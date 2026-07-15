#![allow(unused_crate_dependencies)]
// integration target inherits the library dependency graph while exercising the assembled public router
#![allow(clippy::tests_outside_test_module)] // every item in this integration target is compiled exclusively by the test harness
#[derive(Clone, Copy)]
struct StdAdminApiTestStrRef<'value_lt>(&'value_lt str);
struct AxumAdminApiTestRouter(axum::Router);
struct SqlxAdminApiTestPool(sqlx::PgPool);
struct HttpAdminApiTestMethod(http::Method);
struct HttpAdminApiTestRequest(http::Request<axum::body::Body>);
#[derive(Clone, Copy)]
struct HttpAdminApiTestResponseRef<'value_lt>(&'value_lt http::Response<axum::body::Body>);
#[derive(newtype::BoundedString)]
#[bounded_string(max = 16384)]
struct StdAdminApiTestCookie(String);
impl std::fmt::Display for StdAdminApiTestCookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
fn env<T>(value: StdAdminApiTestStrRef<'_>) -> T
where
    T: config_lib::TryFromStdEnvVarOk,
    T::Error: std::fmt::Debug,
{
    T::try_from_std_env_var_ok(
        config_lib::StdEnvVarOk::try_from(value.0.to_owned()).expect("92b71c4e"),
    )
    .expect("afe20c19")
}
fn router() -> AxumAdminApiTestRouter {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy(str_constants::expr::S_1615)
        .expect("27db915c");
    let state = server_admin::auth::AdminAuthSvcState::try_new(
        app_state::SqlxPgPool::from(pool),
        &env::<config_lib::AdminJwtSecret>(StdAdminApiTestStrRef(str_constants::expr::S_1430)),
        &env::<config_lib::AdminAccessTokenTtlSeconds>(StdAdminApiTestStrRef(
            str_constants::expr::S_0520,
        )),
        &env::<config_lib::AdminRefreshTokenTtlSeconds>(StdAdminApiTestStrRef(
            str_constants::expr::S_0279,
        )),
        &env::<config_lib::AdminSessionLimit>(StdAdminApiTestStrRef(str_constants::expr::S_0215)),
        &env::<config_lib::AdminSignInRateLimit>(StdAdminApiTestStrRef(
            str_constants::expr::S_0214,
        )),
        &env::<config_lib::AdminPasswordHashConcurrency>(StdAdminApiTestStrRef(
            str_constants::expr::S_0167,
        )),
        &env::<config_lib::AdminCookieSecure>(StdAdminApiTestStrRef(str_constants::expr::S_1311)),
        &env::<config_lib::AdminTokenIssuer>(StdAdminApiTestStrRef(str_constants::expr::S_1428)),
        &env::<config_lib::AdminTokenAudience>(StdAdminApiTestStrRef(str_constants::expr::S_1429)),
        &config_lib::CorsAllowOrigin(str_constants::expr::S_1391.to_owned()),
    )
    .expect("f7d8c961");
    AxumAdminApiTestRouter(axum::Router::from(server_admin::auth::routes(
        server_admin::auth::StdSharedAdminAuthSvcState::from(std::sync::Arc::new(state)),
    )))
}
fn router_with_pool(pool: &SqlxAdminApiTestPool) -> AxumAdminApiTestRouter {
    let state = server_admin::auth::AdminAuthSvcState::try_new(
        app_state::SqlxPgPool::from(pool.0.clone()),
        &env::<config_lib::AdminJwtSecret>(StdAdminApiTestStrRef(str_constants::expr::S_1430)),
        &env::<config_lib::AdminAccessTokenTtlSeconds>(StdAdminApiTestStrRef(
            str_constants::expr::S_0520,
        )),
        &env::<config_lib::AdminRefreshTokenTtlSeconds>(StdAdminApiTestStrRef(
            str_constants::expr::S_0279,
        )),
        &env::<config_lib::AdminSessionLimit>(StdAdminApiTestStrRef(str_constants::expr::S_0215)),
        &env::<config_lib::AdminSignInRateLimit>(StdAdminApiTestStrRef(
            str_constants::expr::S_0214,
        )),
        &env::<config_lib::AdminPasswordHashConcurrency>(StdAdminApiTestStrRef(
            str_constants::expr::S_0167,
        )),
        &env::<config_lib::AdminCookieSecure>(StdAdminApiTestStrRef(str_constants::expr::S_1311)),
        &env::<config_lib::AdminTokenIssuer>(StdAdminApiTestStrRef(str_constants::expr::S_1428)),
        &env::<config_lib::AdminTokenAudience>(StdAdminApiTestStrRef(str_constants::expr::S_1429)),
        &config_lib::CorsAllowOrigin(str_constants::expr::S_1391.to_owned()),
    )
    .expect("a59d73c1");
    AxumAdminApiTestRouter(axum::Router::from(server_admin::auth::routes(
        server_admin::auth::StdSharedAdminAuthSvcState::from(std::sync::Arc::new(state)),
    )))
}
fn request_with_peer(
    method: HttpAdminApiTestMethod,
    uri: StdAdminApiTestStrRef<'_>,
    body: StdAdminApiTestStrRef<'_>,
    cookie: Option<StdAdminApiTestStrRef<'_>>,
    csrf: Option<StdAdminApiTestStrRef<'_>>,
) -> HttpAdminApiTestRequest {
    request_with_peer_at(
        method,
        uri,
        body,
        cookie,
        csrf,
        StdAdminApiTestStrRef(str_constants::expr::S_0181),
    )
}
fn request_with_peer_at(
    method: HttpAdminApiTestMethod,
    uri: StdAdminApiTestStrRef<'_>,
    body: StdAdminApiTestStrRef<'_>,
    cookie: Option<StdAdminApiTestStrRef<'_>>,
    csrf: Option<StdAdminApiTestStrRef<'_>>,
    peer: StdAdminApiTestStrRef<'_>,
) -> HttpAdminApiTestRequest {
    let mut builder = http::Request::builder()
        .method(method.0)
        .uri(uri.0)
        .header(http::header::CONTENT_TYPE, str_constants::expr::S_0951)
        .header(http::header::ORIGIN, str_constants::expr::S_1391);
    if let Some(value) = cookie {
        builder = builder.header(http::header::COOKIE, value.0);
    }
    if let Some(value) = csrf {
        builder = builder.header(str_constants::expr::S_1922, value.0);
    }
    let mut request = builder
        .body(axum::body::Body::from(body.0.to_owned()))
        .expect("7d924f8a");
    let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
        peer.0.parse::<std::net::SocketAddr>().expect("d80fc31b"),
    ));
    HttpAdminApiTestRequest(request)
}
fn cookie_value(
    response: HttpAdminApiTestResponseRef<'_>,
    name: StdAdminApiTestStrRef<'_>,
) -> StdAdminApiTestCookie {
    response
        .0
        .headers()
        .get_all(http::header::SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find_map(|value| {
            value
                .split(';')
                .next()
                .and_then(|pair| pair.strip_prefix(name.0))
                .map(str::to_owned)
        })
        .map(|value| StdAdminApiTestCookie::try_from(value).expect("b9a203e6"))
        .expect("360de719")
}
#[tokio::test]
async fn protected_routes_reject_missing_authentication_without_database_io() {
    let users_response = tower::ServiceExt::oneshot(
        router().0,
        http::Request::builder()
            .uri(str_constants::expr::S_0096)
            .body(axum::body::Body::empty())
            .expect("b319e84d"),
    )
    .await
    .expect("0ac617de");
    assert_eq!(users_response.status(), http::StatusCode::UNAUTHORIZED);
    let response = tower::ServiceExt::oneshot(
        router().0,
        http::Request::builder()
            .uri(str_constants::expr::S_0131)
            .body(axum::body::Body::empty())
            .expect("895e12fc"),
    )
    .await
    .expect("1fe80ad3");
    assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
}
#[tokio::test]
async fn invalid_access_cookie_is_rejected_before_database_io() {
    let response = tower::ServiceExt::oneshot(
        router().0,
        http::Request::builder()
            .uri(str_constants::expr::S_0096)
            .header(http::header::COOKIE, str_constants::expr::S_0927)
            .body(axum::body::Body::empty())
            .expect("819acd53"),
    )
    .await
    .expect("c3af0891");
    assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
}
#[tokio::test]
async fn unknown_admin_api_route_is_not_captured_by_spa_fallback() {
    let response = tower::ServiceExt::oneshot(
        router().0,
        http::Request::builder()
            .uri(str_constants::expr::S_0115)
            .body(axum::body::Body::empty())
            .expect("1ca76f8d"),
    )
    .await
    .expect("ce417390");
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
}
#[tokio::test]
async fn wrong_admin_http_method_uses_problem_details_contract() {
    let response = tower::ServiceExt::oneshot(
        router().0,
        http::Request::builder()
            .method(http::Method::GET)
            .uri(str_constants::expr::S_0099)
            .body(axum::body::Body::empty())
            .expect("4eb1c098"),
    )
    .await
    .expect("6764152a");
    assert_eq!(response.status(), http::StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(
        response.headers().get(http::header::CONTENT_TYPE),
        Some(&http::HeaderValue::from_static("application/problem+json")),
    );
}
#[tokio::test]
async fn invalid_admin_json_uses_problem_details_and_body_limit_contract() {
    let malformed_response = tower::ServiceExt::oneshot(
        router().0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0099),
            StdAdminApiTestStrRef(str_constants::expr::S_1934),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("5fb0627d");
    assert_eq!(
        malformed_response.status(),
        http::StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(
        malformed_response.headers().get(http::header::CONTENT_TYPE),
        Some(&http::HeaderValue::from_static("application/problem+json")),
    );
    let oversized_password = str_constants::expr::S_1919.repeat(65_537usize);
    let oversized_body = format!(r#"{{"login":"admin","password":"{oversized_password}"}}"#);
    let oversized_response = tower::ServiceExt::oneshot(
        router().0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0099),
            StdAdminApiTestStrRef(oversized_body.as_str()),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("fcd3dd3f");
    assert_eq!(
        oversized_response.status(),
        http::StatusCode::PAYLOAD_TOO_LARGE
    );
    assert_eq!(
        oversized_response.headers().get(http::header::CONTENT_TYPE),
        Some(&http::HeaderValue::from_static("application/problem+json")),
    );
}
#[tokio::test]
async fn sign_in_requires_trusted_origin_without_database_io() {
    let make_request = |origin, referer| {
        let mut builder = http::Request::builder()
            .method(http::Method::POST)
            .uri(str_constants::expr::S_0099)
            .header(http::header::CONTENT_TYPE, str_constants::expr::S_0951);
        if let Some(value) = origin {
            builder = builder.header(http::header::ORIGIN, value);
        }
        if let Some(value) = referer {
            builder = builder.header(http::header::REFERER, value);
        }
        let mut request = builder
            .body(axum::body::Body::from(str_constants::expr::S_1935))
            .expect("168060a3");
        let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
            str_constants::expr::S_0181
                .parse::<std::net::SocketAddr>()
                .expect("c90cba14"),
        ));
        request
    };
    let missing_origin_response = tower::ServiceExt::oneshot(router().0, make_request(None, None))
        .await
        .expect("ed2f56fb");
    assert_eq!(
        missing_origin_response.status(),
        http::StatusCode::UNAUTHORIZED
    );
    let blocked_origin_response = tower::ServiceExt::oneshot(
        router().0,
        make_request(
            Some(str_constants::expr::S_1390),
            Some(str_constants::expr::S_1392),
        ),
    )
    .await
    .expect("df43c793");
    assert_eq!(
        blocked_origin_response.status(),
        http::StatusCode::UNAUTHORIZED
    );
}
#[tokio::test]
async fn postgresql_auth_rbac_csrf_session_and_audit_flow() {
    let Ok(database_url) = std::env::var(str_constants::expr::S_0649) else {
        return;
    };
    let pool = SqlxAdminApiTestPool(
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url.as_str())
            .await
            .expect("a3e1f57c"),
    );
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool.0))
        .await
        .expect("0ea8d516");
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool.0))
        .await
        .expect("676c00f1");
    let _truncate_result = sqlx::query(str_constants::expr::S_0797)
        .execute(&pool.0)
        .await
        .expect("97b5ad2f");
    let password = serde_json::from_str::<server_admin::AdminPassword>(str_constants::expr::S_0853)
        .expect("703a8df2");
    let hasher = server_admin::AdminPasswordHasher::new(
        server_admin::AdminPasswordHashConcurrency::from(server_admin::StdAdminNonZeroUsize::from(
            std::num::NonZeroUsize::new(1).expect("271f96d4"),
        )),
    );
    let _admin_id = server_admin::bootstrap_admin(
        app_state::SqlxPgPoolRef::from(&pool.0),
        server_admin::AdminLogin::try_from(str_constants::expr::S_1686.to_owned())
            .expect("98c7e04a"),
        server_admin::AdminDisplayName::try_from(str_constants::expr::S_0740.to_owned())
            .expect("48efed01"),
        password,
        &hasher,
    )
    .await
    .expect("e2c94d67");
    let original_password_hash = sqlx::query_scalar::<_, String>(str_constants::expr::S_0774)
        .fetch_one(&pool.0)
        .await
        .expect("1282b56e");
    let repeated_password =
        serde_json::from_str::<server_admin::AdminPassword>(str_constants::expr::S_0854)
            .expect("e411f376");
    assert!(matches!(
        server_admin::bootstrap_admin(
            app_state::SqlxPgPoolRef::from(&pool.0),
            server_admin::AdminLogin::try_from("other_admin".to_owned()).expect("8359ca1a"),
            server_admin::AdminDisplayName::try_from("Other Admin".to_owned()).expect("d968dddb"),
            repeated_password,
            &hasher,
        )
        .await,
        Err(server_admin::AdminBootstrapError::AlreadyInitialized)
    ));
    let preserved_password_hash = sqlx::query_scalar::<_, String>(str_constants::expr::S_0774)
        .fetch_one(&pool.0)
        .await
        .expect("65ff827e");
    assert_eq!(preserved_password_hash, original_password_hash);
    let administrator_count = sqlx::query_scalar::<_, i64>(str_constants::expr::S_0750)
        .fetch_one(&pool.0)
        .await
        .expect("ae89c3bd");
    assert_eq!(administrator_count, 1i64);
    let admin_id = sqlx::query_scalar::<_, i64>(str_constants::expr::S_0764)
        .fetch_one(&pool.0)
        .await
        .expect("a61329bf");
    let dangling_role_links = sqlx::query_scalar::<_, i64>(str_constants::expr::S_0749)
        .fetch_one(&pool.0)
        .await
        .expect("08ef120f");
    assert_eq!(dangling_role_links, 0i64);
    let dangling_permission_links = sqlx::query_scalar::<_, i64>(str_constants::expr::S_0747)
        .fetch_one(&pool.0)
        .await
        .expect("aebf6dc8");
    assert_eq!(dangling_permission_links, 0i64);
    let wrong_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0099),
            StdAdminApiTestStrRef(str_constants::expr::S_1941),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("5472ea19");
    assert_eq!(wrong_response.status(), http::StatusCode::UNAUTHORIZED);
    let sign_in_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0099),
            StdAdminApiTestStrRef(str_constants::expr::S_1940),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("c245193e");
    assert_eq!(sign_in_response.status(), http::StatusCode::OK);
    let access = cookie_value(
        HttpAdminApiTestResponseRef(&sign_in_response),
        StdAdminApiTestStrRef(str_constants::expr::S_0926),
    );
    let refresh = cookie_value(
        HttpAdminApiTestResponseRef(&sign_in_response),
        StdAdminApiTestStrRef(str_constants::expr::S_0935),
    );
    let csrf = cookie_value(
        HttpAdminApiTestResponseRef(&sign_in_response),
        StdAdminApiTestStrRef(str_constants::expr::S_0931),
    );
    let cookie = format!(
        "admin_access_token={access}; admin_refresh_token={refresh}; admin_csrf_token={csrf}"
    );
    let me_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::GET),
            StdAdminApiTestStrRef(str_constants::expr::S_0096),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("b67815ec");
    assert_eq!(me_response.status(), http::StatusCode::OK);
    let changed_context_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer_at(
            HttpAdminApiTestMethod(http::Method::GET),
            StdAdminApiTestStrRef(str_constants::expr::S_0096),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(cookie.as_str())),
            None,
            StdAdminApiTestStrRef(str_constants::expr::S_0183),
        )
        .0,
    )
    .await
    .expect("f11e0324");
    assert_eq!(
        changed_context_response.status(),
        http::StatusCode::UNAUTHORIZED
    );
    let first_refresh_cookie = format!("admin_refresh_token={refresh}");
    let refresh_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0097),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(first_refresh_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("9f0be285");
    assert_eq!(refresh_response.status(), http::StatusCode::OK);
    let refreshed_access = cookie_value(
        HttpAdminApiTestResponseRef(&refresh_response),
        StdAdminApiTestStrRef(str_constants::expr::S_0926),
    );
    assert!(
        !refresh_response
            .headers()
            .get_all(http::header::SET_COOKIE)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .any(|value| value.starts_with("admin_refresh_token="))
    );
    let refreshed_csrf = cookie_value(
        HttpAdminApiTestResponseRef(&refresh_response),
        StdAdminApiTestStrRef(str_constants::expr::S_0931),
    );
    let active_cookie = format!(
        "admin_access_token={refreshed_access}; admin_refresh_token={refresh}; admin_csrf_token={refreshed_csrf}"
    );
    let reused_refresh_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0097),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(first_refresh_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("b8c71e43");
    assert_eq!(reused_refresh_response.status(), http::StatusCode::OK);
    let first_lockout_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0099),
            StdAdminApiTestStrRef(str_constants::expr::S_1939),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("8f72b01e");
    assert_eq!(
        first_lockout_response.status(),
        http::StatusCode::UNAUTHORIZED
    );
    let second_lockout_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0099),
            StdAdminApiTestStrRef(str_constants::expr::S_1939),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("2d94c01e");
    assert_eq!(
        second_lockout_response.status(),
        http::StatusCode::UNAUTHORIZED
    );
    let limited_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0099),
            StdAdminApiTestStrRef(str_constants::expr::S_1939),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("7324af80");
    assert_eq!(
        limited_response.status(),
        http::StatusCode::TOO_MANY_REQUESTS
    );
    let csrf_denied_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0131),
            StdAdminApiTestStrRef(str_constants::expr::S_1937),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("153b847c");
    assert_eq!(csrf_denied_response.status(), http::StatusCode::FORBIDDEN);
    let create_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0131),
            StdAdminApiTestStrRef(str_constants::expr::S_1937),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("c86a4310");
    assert_eq!(create_response.status(), http::StatusCode::CREATED);
    let limited_sign_in_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0099),
            StdAdminApiTestStrRef(str_constants::expr::S_1938),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("a2d6139e");
    assert_eq!(limited_sign_in_response.status(), http::StatusCode::OK);
    let limited_access = cookie_value(
        HttpAdminApiTestResponseRef(&limited_sign_in_response),
        StdAdminApiTestStrRef(str_constants::expr::S_0926),
    );
    let limited_refresh = cookie_value(
        HttpAdminApiTestResponseRef(&limited_sign_in_response),
        StdAdminApiTestStrRef(str_constants::expr::S_0935),
    );
    let limited_csrf = cookie_value(
        HttpAdminApiTestResponseRef(&limited_sign_in_response),
        StdAdminApiTestStrRef(str_constants::expr::S_0931),
    );
    let limited_cookie = format!(
        "admin_access_token={limited_access}; admin_refresh_token={limited_refresh}; admin_csrf_token={limited_csrf}"
    );
    let forbidden_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::GET),
            StdAdminApiTestStrRef(str_constants::expr::S_0131),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(limited_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("617f08b9");
    assert_eq!(forbidden_response.status(), http::StatusCode::FORBIDDEN);
    let revoke_all_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::DELETE),
            StdAdminApiTestStrRef(str_constants::expr::S_0098),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(limited_cookie.as_str())),
            Some(StdAdminApiTestStrRef(limited_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("0f51dc7a");
    assert_eq!(revoke_all_response.status(), http::StatusCode::NO_CONTENT);
    let revoked_all_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::GET),
            StdAdminApiTestStrRef(str_constants::expr::S_0096),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(limited_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("24ec178b");
    assert_eq!(
        revoked_all_response.status(),
        http::StatusCode::UNAUTHORIZED
    );
    let limited_id = sqlx::query_scalar::<_, i64>(str_constants::expr::S_0763)
        .fetch_one(&pool.0)
        .await
        .expect("10c8f7d2");
    let update_user_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::PATCH),
            StdAdminApiTestStrRef(format!("/users/{limited_id}").as_str()),
            StdAdminApiTestStrRef(str_constants::expr::S_1931),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("623cde18");
    assert_eq!(update_user_response.status(), http::StatusCode::NO_CONTENT);
    let ban_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(format!("/users/{limited_id}/ban").as_str()),
            StdAdminApiTestStrRef(str_constants::expr::S_1933),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("94a7e1cb");
    assert_eq!(ban_response.status(), http::StatusCode::NO_CONTENT);
    let banned_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::GET),
            StdAdminApiTestStrRef(str_constants::expr::S_0096),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(limited_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("fac2138b");
    assert_eq!(banned_response.status(), http::StatusCode::UNAUTHORIZED);
    let banned_sign_in_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0099),
            StdAdminApiTestStrRef(str_constants::expr::S_1938),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("891d7ca2");
    assert_eq!(
        banned_sign_in_response.status(),
        http::StatusCode::UNAUTHORIZED
    );
    let list_users_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::GET),
            StdAdminApiTestStrRef(str_constants::expr::S_0131),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("475af63b");
    assert_eq!(list_users_response.status(), http::StatusCode::OK);
    let list_roles_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::GET),
            StdAdminApiTestStrRef(str_constants::expr::S_0118),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("c5f103da");
    assert_eq!(list_roles_response.status(), http::StatusCode::OK);
    let create_role_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0118),
            StdAdminApiTestStrRef(str_constants::expr::S_1944),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("6d9384fe");
    assert_eq!(create_role_response.status(), http::StatusCode::CREATED);
    let role_id = sqlx::query_scalar::<_, i64>(str_constants::expr::S_0762)
        .fetch_one(&pool.0)
        .await
        .expect("1e53a0c7");
    let update_role_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::PATCH),
            StdAdminApiTestStrRef(format!("/roles/{role_id}").as_str()),
            StdAdminApiTestStrRef(str_constants::expr::S_1943),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("4f08b7ec");
    assert_eq!(update_role_response.status(), http::StatusCode::NO_CONTENT);
    let delete_role_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::DELETE),
            StdAdminApiTestStrRef(format!("/roles/{role_id}").as_str()),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("d7e1862c");
    assert_eq!(delete_role_response.status(), http::StatusCode::NO_CONTENT);
    let delete_user_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::DELETE),
            StdAdminApiTestStrRef(format!("/users/{limited_id}").as_str()),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("c19be784");
    assert_eq!(delete_user_response.status(), http::StatusCode::NO_CONTENT);
    let last_admin_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::DELETE),
            StdAdminApiTestStrRef(format!("/users/{admin_id}").as_str()),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("e6175d82");
    assert_eq!(last_admin_response.status(), http::StatusCode::CONFLICT);
    let audit_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::GET),
            StdAdminApiTestStrRef(str_constants::expr::S_0095),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("8103cd5f");
    assert_eq!(audit_response.status(), http::StatusCode::OK);
    let sign_out_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef(str_constants::expr::S_0100),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("ef71e50a");
    assert_eq!(sign_out_response.status(), http::StatusCode::NO_CONTENT);
    let revoked_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::GET),
            StdAdminApiTestStrRef(str_constants::expr::S_0096),
            StdAdminApiTestStrRef(str_constants::expr::S_0021),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("54b9dc03");
    assert_eq!(revoked_response.status(), http::StatusCode::UNAUTHORIZED);
    let audit_outcomes = sqlx::query_as::<_, (bool, i64)>(str_constants::expr::S_0778)
        .fetch_all(&pool.0)
        .await
        .expect("3de105a4");
    assert!(
        audit_outcomes
            .iter()
            .any(|(succeeded, count)| !succeeded && *count > 0)
    );
    assert!(
        audit_outcomes
            .iter()
            .any(|(succeeded, count)| *succeeded && *count > 0)
    );
}
#[tokio::test]
async fn postgresql_generated_mutation_idempotency_contract() {
    let Ok(database_url) = std::env::var(str_constants::expr::S_0649) else {
        return;
    };
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4u32)
        .connect(database_url.as_str())
        .await
        .expect("cb6830bc");
    pg_table::ensure_pg_table_idempotency_schema(app_state::SqlxPgPoolRef::from(&pool))
        .await
        .expect("6c338824");
    let _truncate_result = sqlx::query(str_constants::expr::S_0798)
        .execute(&pool)
        .await
        .expect("d93beb69");
    let make_request = |actor: StdAdminApiTestStrRef<'_>,
                        route: StdAdminApiTestStrRef<'_>,
                        key: StdAdminApiTestStrRef<'_>,
                        body: pg_table::PgTableIdempotencyBodyRef<'_>| {
        pg_table::PgTableIdempotencyRequest::new(
            pg_table::PgTableIdempotencyScope::new(
                pg_table::PgTableIdempotencyActor::try_from(actor.0.to_owned()).expect("e6640036"),
                pg_table::PgTableIdempotencyMethod::try_from(
                    str_constants::expr::S_0722.to_owned(),
                )
                .expect("94bc0508"),
                pg_table::PgTableIdempotencyRoute::try_from(route.0.to_owned()).expect("4e8c040f"),
                pg_table::PgTableIdempotencyKey::try_from(key.0.to_owned()).expect("2028024d"),
            ),
            body,
        )
    };
    let first_request = make_request(
        StdAdminApiTestStrRef(str_constants::expr::S_0916),
        StdAdminApiTestStrRef(str_constants::expr::S_0107),
        StdAdminApiTestStrRef(str_constants::expr::S_1446),
        pg_table::PgTableIdempotencyBodyRef::from(br#"{"value":1}"#.as_slice()),
    );
    let first =
        pg_table::begin_pg_table_idempotency(app_state::SqlxPgPoolRef::from(&pool), &first_request)
            .await
            .expect("c8b3565c");
    assert_eq!(first, pg_table::PgTableIdempotencyBegin::Acquired);
    let pending =
        pg_table::begin_pg_table_idempotency(app_state::SqlxPgPoolRef::from(&pool), &first_request)
            .await
            .expect("c5c45332");
    assert_eq!(pending, pg_table::PgTableIdempotencyBegin::InProgress);
    let conflicting_request = make_request(
        StdAdminApiTestStrRef(str_constants::expr::S_0916),
        StdAdminApiTestStrRef(str_constants::expr::S_0107),
        StdAdminApiTestStrRef(str_constants::expr::S_1446),
        pg_table::PgTableIdempotencyBodyRef::from(br#"{"value":2}"#.as_slice()),
    );
    let conflict = pg_table::begin_pg_table_idempotency(
        app_state::SqlxPgPoolRef::from(&pool),
        &conflicting_request,
    )
    .await
    .expect("7f419767");
    assert_eq!(conflict, pg_table::PgTableIdempotencyBegin::Conflict);
    let response_body = br#"{"desirable":{"id":1}}"#;
    pg_table::complete_pg_table_idempotency(
        app_state::SqlxPgPoolRef::from(&pool),
        &first_request,
        pg_table::PgTableIdempotencyResponseStatus::from(201u16),
        pg_table::PgTableIdempotencyBodyRef::from(response_body.as_slice()),
    )
    .await
    .expect("9106c1e6");
    let replay =
        pg_table::begin_pg_table_idempotency(app_state::SqlxPgPoolRef::from(&pool), &first_request)
            .await
            .expect("0721b23f");
    let pg_table::PgTableIdempotencyBegin::Replay(replay_value) = replay else {
        panic!("9f97fb0d");
    };
    assert_eq!(
        replay_value.into_parts(),
        (
            pg_table::PgTableIdempotencyResponseStatus::from(201u16),
            pg_table::PgTableIdempotencyBody::from(response_body.to_vec()),
        )
    );
    let other_actor = make_request(
        StdAdminApiTestStrRef(str_constants::expr::S_0918),
        StdAdminApiTestStrRef(str_constants::expr::S_0107),
        StdAdminApiTestStrRef(str_constants::expr::S_1446),
        pg_table::PgTableIdempotencyBodyRef::from(br#"{"value":1}"#.as_slice()),
    );
    assert_eq!(
        pg_table::begin_pg_table_idempotency(app_state::SqlxPgPoolRef::from(&pool), &other_actor)
            .await
            .expect("e581d572"),
        pg_table::PgTableIdempotencyBegin::Acquired
    );
    pg_table::release_pg_table_idempotency(app_state::SqlxPgPoolRef::from(&pool), &other_actor)
        .await
        .expect("31e0437d");
    assert_eq!(
        pg_table::begin_pg_table_idempotency(app_state::SqlxPgPoolRef::from(&pool), &other_actor)
            .await
            .expect("fe57d4dc"),
        pg_table::PgTableIdempotencyBegin::Acquired
    );
    let concurrent = make_request(
        StdAdminApiTestStrRef(str_constants::expr::S_0919),
        StdAdminApiTestStrRef(str_constants::expr::S_0107),
        StdAdminApiTestStrRef(str_constants::expr::S_1448),
        pg_table::PgTableIdempotencyBodyRef::from(br#"{"value":3}"#.as_slice()),
    );
    let (left, right) = tokio::join!(
        pg_table::begin_pg_table_idempotency(app_state::SqlxPgPoolRef::from(&pool), &concurrent),
        pg_table::begin_pg_table_idempotency(app_state::SqlxPgPoolRef::from(&pool), &concurrent)
    );
    let outcomes = [left.expect("874153ec"), right.expect("64c4cc46")];
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| **outcome == pg_table::PgTableIdempotencyBegin::Acquired)
            .count(),
        1usize
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| **outcome == pg_table::PgTableIdempotencyBegin::InProgress)
            .count(),
        1usize
    );
    let _atomic_table = sqlx::query(str_constants::expr::S_0636)
        .execute(&pool)
        .await
        .expect("af066e8b");
    let _atomic_clear = sqlx::query(str_constants::expr::S_0799)
        .execute(&pool)
        .await
        .expect("3130e593");
    let atomic = make_request(
        StdAdminApiTestStrRef(str_constants::expr::S_0917),
        StdAdminApiTestStrRef(str_constants::expr::S_0108),
        StdAdminApiTestStrRef(str_constants::expr::S_1447),
        pg_table::PgTableIdempotencyBodyRef::from(br#"{"id":1}"#.as_slice()),
    );
    assert_eq!(
        pg_table::begin_pg_table_idempotency(app_state::SqlxPgPoolRef::from(&pool), &atomic)
            .await
            .expect("925ea283"),
        pg_table::PgTableIdempotencyBegin::Acquired
    );
    let mut rollback_tx = pool.begin().await.expect("fcba80e1");
    let _mutation = sqlx::query(str_constants::expr::S_0693)
        .execute(&mut *rollback_tx)
        .await
        .expect("67503e70");
    pg_table::complete_pg_table_idempotency_in_connection(
        pg_table::SqlxPgTablePgConnectionRef::from(&mut *rollback_tx),
        &atomic,
        pg_table::PgTableIdempotencyResponseStatus::from(201u16),
        pg_table::PgTableIdempotencyBodyRef::from(br#"{"id":1}"#.as_slice()),
    )
    .await
    .expect("8ad86515");
    rollback_tx.rollback().await.expect("11cfcb27");
    let mutation_count = sqlx::query_scalar::<_, i64>(str_constants::expr::S_0752)
        .fetch_one(&pool)
        .await
        .expect("84e57ab6");
    assert_eq!(mutation_count, 0i64);
    assert_eq!(
        pg_table::begin_pg_table_idempotency(app_state::SqlxPgPoolRef::from(&pool), &atomic)
            .await
            .expect("3903bf53"),
        pg_table::PgTableIdempotencyBegin::InProgress
    );
    pg_table::release_pg_table_idempotency(app_state::SqlxPgPoolRef::from(&pool), &atomic)
        .await
        .expect("67973e68");
    let _age_records = sqlx::query(str_constants::expr::S_0817)
        .execute(&pool)
        .await
        .expect("a46f7336");
    let before_cleanup = sqlx::query_scalar::<_, i64>(str_constants::expr::S_0751)
        .fetch_one(&pool)
        .await
        .expect("2c080f6d");
    let cleaned = pg_table::cleanup_pg_table_idempotency(
        app_state::SqlxPgPoolRef::from(&pool),
        pg_table::PgTableIdempotencyCleanupRetentionSeconds::from(1i64),
        pg_table::PgTableIdempotencyCleanupRetentionSeconds::from(1i64),
        pg_table::PgTableIdempotencyCleanupBatchSize::from(2i64),
    )
    .await
    .expect("b1ba49cc");
    assert_eq!(u64::from(cleaned), 2u64);
    let after_cleanup = sqlx::query_scalar::<_, i64>(str_constants::expr::S_0751)
        .fetch_one(&pool)
        .await
        .expect("6863201e");
    assert_eq!(
        before_cleanup.checked_sub(after_cleanup).expect("f93ed3cf"),
        2i64
    );
}
#[tokio::test]
async fn postgresql_optimistic_revision_allows_one_concurrent_writer() {
    let Ok(database_url) = std::env::var(str_constants::expr::S_0649) else {
        return;
    };
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4u32)
        .connect(database_url.as_str())
        .await
        .expect("2480f8c4");
    let _drop_before = sqlx::query(str_constants::expr::S_0659)
        .execute(&pool)
        .await
        .expect("e5e1f7cb");
    let _create = sqlx::query(str_constants::expr::S_0637)
        .execute(&pool)
        .await
        .expect("a75bc224");
    let _insert = sqlx::query(str_constants::expr::S_0694)
        .execute(&pool)
        .await
        .expect("da271038");
    let update = str_constants::expr::S_0818;
    let (left, right) = tokio::join!(
        sqlx::query_scalar::<_, i64>(update)
            .bind(1i64)
            .bind(
                pg_table::PgTableRevision::try_from(str_constants::expr::S_0136.to_owned())
                    .expect("979fa4b2")
            )
            .fetch_optional(&pool),
        sqlx::query_scalar::<_, i64>(update)
            .bind(2i64)
            .bind(
                pg_table::PgTableRevision::try_from(str_constants::expr::S_0136.to_owned())
                    .expect("589ea31d")
            )
            .fetch_optional(&pool),
    );
    let outcomes = [left.expect("a1a1382a"), right.expect("8406b933")];
    assert_eq!(
        outcomes.iter().filter(|value| value.is_some()).count(),
        1usize
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT revision FROM pg_table_optimistic_revision_test WHERE id=1",
        )
        .fetch_one(&pool)
        .await
        .expect("c0f01a04"),
        1i64
    );
    let stale = sqlx::query_scalar::<_, i64>(update)
        .bind(3i64)
        .bind(
            pg_table::PgTableRevision::try_from(str_constants::expr::S_0136.to_owned())
                .expect("a3a08aeb"),
        )
        .fetch_optional(&pool)
        .await
        .expect("964e3ef4");
    assert_eq!(stale, None);
    let _drop_after = sqlx::query(str_constants::expr::S_0660)
        .execute(&pool)
        .await
        .expect("a4d77f54");
}
#[tokio::test]
async fn postgresql_cleanup_is_batched_and_preserves_append_only_policy() {
    let Ok(database_url) = std::env::var(str_constants::expr::S_0649) else {
        return;
    };
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2u32)
        .connect(database_url.as_str())
        .await
        .expect("f6a51733");
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool))
        .await
        .expect("029cb682");
    pg_table::ensure_pg_table_idempotency_schema(app_state::SqlxPgPoolRef::from(&pool))
        .await
        .expect("eb08dffc");
    let _clear = sqlx::query(str_constants::expr::S_0796)
        .execute(&pool)
        .await
        .expect("e1b22572");
    let _attempts = sqlx::query(str_constants::expr::S_0684)
        .execute(&pool)
        .await
        .expect("480b06eb");
    let _limits = sqlx::query(str_constants::expr::S_0686)
        .execute(&pool)
        .await
        .expect("0375574d");
    let _audit = sqlx::query(str_constants::expr::S_0682)
        .execute(&pool)
        .await
        .expect("f50ef817");
    let retention = server_admin::AdminCleanupRetentionSeconds::try_from(1i64).expect("ab892fc5");
    let config = server_admin::AdminCleanupCfg::new(
        server_admin::AdminCleanupBatchSize::try_from(2i64).expect("1d97b31c"),
        retention,
        retention,
        retention,
        retention,
        retention,
    );
    let report = server_admin::cleanup_admin_tables(app_state::SqlxPgPoolRef::from(&pool), config)
        .await
        .expect("a422e8d4");
    assert_eq!(report.total_rows().to_string(), "6");
    let remaining = sqlx::query_as::<_, (i64, i64, i64)>(str_constants::expr::S_0743)
        .fetch_one(&pool)
        .await
        .expect("f37a3ab4");
    assert_eq!(remaining, (1i64, 1i64, 1i64));
    let ordinary_delete = sqlx::query(str_constants::expr::S_0650)
        .execute(&pool)
        .await;
    assert!(matches!(ordinary_delete, Err(_error)));
}
#[tokio::test]
async fn postgresql_migrations_cover_fresh_and_supported_baseline_upgrade() {
    let Ok(database_url) = std::env::var(str_constants::expr::S_0649) else {
        return;
    };
    let base_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1u32)
        .connect(database_url.as_str())
        .await
        .expect("0047f74e");
    let _drop_schemas = sqlx::raw_sql(str_constants::expr::S_0657)
        .execute(&base_pool)
        .await
        .expect("df91b04d");
    let _create_schemas = sqlx::raw_sql(str_constants::expr::S_0634)
        .execute(&base_pool)
        .await
        .expect("02bcd1c2");
    let connect = |schema: StdAdminApiTestStrRef<'static>| {
        let options = <sqlx::postgres::PgConnectOptions as std::str::FromStr>::from_str(
            database_url.as_str(),
        )
        .expect("aa7735db")
        .options([(str_constants::expr::S_1702, schema.0)]);
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(1u32)
            .connect_lazy_with(options)
    };
    let fresh_pool = connect(StdAdminApiTestStrRef(str_constants::expr::S_0932));
    let upgrade_pool = connect(StdAdminApiTestStrRef(str_constants::expr::S_0933));
    let full = sqlx::migrate!("./migrations");
    full.run(&fresh_pool).await.expect("4b6c3bd6");
    let baseline = sqlx::migrate::Migrator {
        migrations: std::borrow::Cow::Owned(full.migrations.iter().take(3usize).cloned().collect()),
        ignore_missing: false,
        locking: true,
        no_tx: false,
    };
    baseline.run(&upgrade_pool).await.expect("2e03eccc");
    let baseline_version = sqlx::query_scalar::<_, i64>(str_constants::expr::S_0758)
        .fetch_one(&upgrade_pool)
        .await
        .expect("17862da9");
    assert_eq!(baseline_version, 3i64);
    full.run(&upgrade_pool).await.expect("3664ecff");
    let versions = sqlx::query_as::<_, (i64, i64)>(str_constants::expr::S_0744)
        .fetch_one(&base_pool)
        .await
        .expect("5c10c931");
    assert_eq!(versions, (5i64, 5i64));
    fresh_pool.close().await;
    upgrade_pool.close().await;
    let _drop_after = sqlx::raw_sql(str_constants::expr::S_0658)
        .execute(&base_pool)
        .await
        .expect("88dd90b8");
}
