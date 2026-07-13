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
        .connect_lazy("postgres://admin:integration-only@127.0.0.1/admin_integration")
        .expect("27db915c");
    let state = server_admin::auth::AdminAuthSvcState::try_new(
        app_state::SqlxPgPool::from(pool),
        &env::<config_lib::AdminJwtSecret>(StdAdminApiTestStrRef(
            "integration-test-jwt-secret-at-least-32-bytes",
        )),
        &env::<config_lib::AdminAccessTokenTtlSeconds>(StdAdminApiTestStrRef("900")),
        &env::<config_lib::AdminRefreshTokenTtlSeconds>(StdAdminApiTestStrRef("3600")),
        &env::<config_lib::AdminSessionLimit>(StdAdminApiTestStrRef("20")),
        &env::<config_lib::AdminSignInRateLimit>(StdAdminApiTestStrRef("2")),
        &env::<config_lib::AdminPasswordHashConcurrency>(StdAdminApiTestStrRef("1")),
        &env::<config_lib::AdminCookieSecure>(StdAdminApiTestStrRef("false")),
        &env::<config_lib::AdminTokenIssuer>(StdAdminApiTestStrRef("integration-test")),
        &env::<config_lib::AdminTokenAudience>(StdAdminApiTestStrRef("integration-test-admin")),
        &config_lib::CorsAllowOrigin("http://localhost".to_owned()),
    )
    .expect("f7d8c961");
    AxumAdminApiTestRouter(axum::Router::from(server_admin::auth::routes(
        server_admin::auth::StdSharedAdminAuthSvcState::from(std::sync::Arc::new(state)),
    )))
}
fn router_with_pool(pool: &SqlxAdminApiTestPool) -> AxumAdminApiTestRouter {
    let state = server_admin::auth::AdminAuthSvcState::try_new(
        app_state::SqlxPgPool::from(pool.0.clone()),
        &env::<config_lib::AdminJwtSecret>(StdAdminApiTestStrRef(
            "integration-test-jwt-secret-at-least-32-bytes",
        )),
        &env::<config_lib::AdminAccessTokenTtlSeconds>(StdAdminApiTestStrRef("900")),
        &env::<config_lib::AdminRefreshTokenTtlSeconds>(StdAdminApiTestStrRef("3600")),
        &env::<config_lib::AdminSessionLimit>(StdAdminApiTestStrRef("20")),
        &env::<config_lib::AdminSignInRateLimit>(StdAdminApiTestStrRef("2")),
        &env::<config_lib::AdminPasswordHashConcurrency>(StdAdminApiTestStrRef("1")),
        &env::<config_lib::AdminCookieSecure>(StdAdminApiTestStrRef("false")),
        &env::<config_lib::AdminTokenIssuer>(StdAdminApiTestStrRef("integration-test")),
        &env::<config_lib::AdminTokenAudience>(StdAdminApiTestStrRef("integration-test-admin")),
        &config_lib::CorsAllowOrigin("http://localhost".to_owned()),
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
    let mut builder = http::Request::builder()
        .method(method.0)
        .uri(uri.0)
        .header(http::header::CONTENT_TYPE, "application/json")
        .header(http::header::ORIGIN, "http://localhost");
    if let Some(value) = cookie {
        builder = builder.header(http::header::COOKIE, value.0);
    }
    if let Some(value) = csrf {
        builder = builder.header("x-csrf-token", value.0);
    }
    let mut request = builder
        .body(axum::body::Body::from(body.0.to_owned()))
        .expect("7d924f8a");
    let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
        "127.0.0.1:43210"
            .parse::<std::net::SocketAddr>()
            .expect("d80fc31b"),
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
            .uri("/auth/me")
            .body(axum::body::Body::empty())
            .expect("b319e84d"),
    )
    .await
    .expect("0ac617de");
    assert_eq!(users_response.status(), http::StatusCode::UNAUTHORIZED);
    let response = tower::ServiceExt::oneshot(
        router().0,
        http::Request::builder()
            .uri("/users")
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
            .uri("/auth/me")
            .header(http::header::COOKIE, "admin_access_token=invalid.jwt.token")
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
            .uri("/not-an-api-route")
            .body(axum::body::Body::empty())
            .expect("1ca76f8d"),
    )
    .await
    .expect("ce417390");
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
}
#[tokio::test]
async fn postgresql_auth_rbac_csrf_session_and_audit_flow() {
    #[cfg(feature = "test-utils")]
    let database_url = std::env::var("DATABASE_URL").expect("63f028ae");
    #[cfg(not(feature = "test-utils"))]
    let Ok(database_url) = std::env::var("DATABASE_URL") else {
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
    let _truncate_result = sqlx::query("TRUNCATE admin_rate_limits, admin_audit_log, admin_login_attempts, admin_access_sessions, admin_refresh_tokens, admin_user_roles, admin_users RESTART IDENTITY CASCADE")
        .execute(&pool.0)
        .await
        .expect("97b5ad2f");
    let password = serde_json::from_str::<server_admin::AdminPassword>("\"correct-password\"")
        .expect("703a8df2");
    let hasher = server_admin::AdminPasswordHasher::new(
        server_admin::AdminPasswordHashConcurrency::from(server_admin::StdAdminNonZeroUsize::from(
            std::num::NonZeroUsize::new(1).expect("271f96d4"),
        )),
    );
    let _admin_id = server_admin::bootstrap_admin(
        app_state::SqlxPgPoolRef::from(&pool.0),
        server_admin::AdminLogin::try_from("root_admin".to_owned()).expect("98c7e04a"),
        server_admin::AdminDisplayName::try_from("Root Admin".to_owned()).expect("48efed01"),
        password,
        &hasher,
    )
    .await
    .expect("e2c94d67");
    let admin_id =
        sqlx::query_scalar::<_, i64>("SELECT id FROM admin_users WHERE login = 'root_admin'")
            .fetch_one(&pool.0)
            .await
            .expect("a61329bf");
    let wrong_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef("/auth/sign-in"),
            StdAdminApiTestStrRef("{\"login\":\"root_admin\",\"password\":\"wrong-password\"}"),
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
            StdAdminApiTestStrRef("/auth/sign-in"),
            StdAdminApiTestStrRef("{\"login\":\"root_admin\",\"password\":\"correct-password\"}"),
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
        StdAdminApiTestStrRef("admin_access_token="),
    );
    let refresh = cookie_value(
        HttpAdminApiTestResponseRef(&sign_in_response),
        StdAdminApiTestStrRef("admin_refresh_token="),
    );
    let csrf = cookie_value(
        HttpAdminApiTestResponseRef(&sign_in_response),
        StdAdminApiTestStrRef("admin_csrf_token="),
    );
    let cookie = format!(
        "admin_access_token={access}; admin_refresh_token={refresh}; admin_csrf_token={csrf}"
    );
    let me_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::GET),
            StdAdminApiTestStrRef("/auth/me"),
            StdAdminApiTestStrRef(""),
            Some(StdAdminApiTestStrRef(cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("b67815ec");
    assert_eq!(me_response.status(), http::StatusCode::OK);
    let first_refresh_cookie = format!("admin_refresh_token={refresh}");
    let refresh_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef("/auth/refresh"),
            StdAdminApiTestStrRef(""),
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
        StdAdminApiTestStrRef("admin_access_token="),
    );
    let refreshed_refresh = cookie_value(
        HttpAdminApiTestResponseRef(&refresh_response),
        StdAdminApiTestStrRef("admin_refresh_token="),
    );
    let refreshed_csrf = cookie_value(
        HttpAdminApiTestResponseRef(&refresh_response),
        StdAdminApiTestStrRef("admin_csrf_token="),
    );
    let active_cookie = format!(
        "admin_access_token={refreshed_access}; admin_refresh_token={refreshed_refresh}; admin_csrf_token={refreshed_csrf}"
    );
    let reused_refresh_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef("/auth/refresh"),
            StdAdminApiTestStrRef(""),
            Some(StdAdminApiTestStrRef(first_refresh_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("b8c71e43");
    assert_eq!(
        reused_refresh_response.status(),
        http::StatusCode::UNAUTHORIZED
    );
    let first_lockout_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef("/auth/sign-in"),
            StdAdminApiTestStrRef("{\"login\":\"locked_user\",\"password\":\"wrong-password\"}"),
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
            StdAdminApiTestStrRef("/auth/sign-in"),
            StdAdminApiTestStrRef("{\"login\":\"locked_user\",\"password\":\"wrong-password\"}"),
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
            StdAdminApiTestStrRef("/auth/sign-in"),
            StdAdminApiTestStrRef("{\"login\":\"locked_user\",\"password\":\"wrong-password\"}"),
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
        request_with_peer(HttpAdminApiTestMethod(http::Method::POST), StdAdminApiTestStrRef("/users"), StdAdminApiTestStrRef("{\"login\":\"limited_user\",\"display_name\":\"Limited User\",\"password\":\"limited-password\"}"), Some(StdAdminApiTestStrRef(active_cookie.as_str())), None).0,
    )
    .await
    .expect("153b847c");
    assert_eq!(csrf_denied_response.status(), http::StatusCode::FORBIDDEN);
    let create_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(HttpAdminApiTestMethod(http::Method::POST), StdAdminApiTestStrRef("/users"), StdAdminApiTestStrRef("{\"login\":\"limited_user\",\"display_name\":\"Limited User\",\"password\":\"limited-password\"}"), Some(StdAdminApiTestStrRef(active_cookie.as_str())), Some(StdAdminApiTestStrRef(refreshed_csrf.0.as_str()))).0,
    )
    .await
    .expect("c86a4310");
    assert_eq!(create_response.status(), http::StatusCode::CREATED);
    let limited_sign_in_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::POST),
            StdAdminApiTestStrRef("/auth/sign-in"),
            StdAdminApiTestStrRef("{\"login\":\"limited_user\",\"password\":\"limited-password\"}"),
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
        StdAdminApiTestStrRef("admin_access_token="),
    );
    let limited_refresh = cookie_value(
        HttpAdminApiTestResponseRef(&limited_sign_in_response),
        StdAdminApiTestStrRef("admin_refresh_token="),
    );
    let limited_csrf = cookie_value(
        HttpAdminApiTestResponseRef(&limited_sign_in_response),
        StdAdminApiTestStrRef("admin_csrf_token="),
    );
    let limited_cookie = format!(
        "admin_access_token={limited_access}; admin_refresh_token={limited_refresh}; admin_csrf_token={limited_csrf}"
    );
    let forbidden_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::GET),
            StdAdminApiTestStrRef("/users"),
            StdAdminApiTestStrRef(""),
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
            StdAdminApiTestStrRef("/auth/sessions"),
            StdAdminApiTestStrRef(""),
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
            StdAdminApiTestStrRef("/auth/me"),
            StdAdminApiTestStrRef(""),
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
    let limited_id =
        sqlx::query_scalar::<_, i64>("SELECT id FROM admin_users WHERE login = 'limited_user'")
            .fetch_one(&pool.0)
            .await
            .expect("10c8f7d2");
    let update_user_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::PATCH),
            StdAdminApiTestStrRef(format!("/users/{limited_id}").as_str()),
            StdAdminApiTestStrRef("{\"display_name\":\"Updated User\"}"),
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
            StdAdminApiTestStrRef("{\"is_banned\":true}"),
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
            StdAdminApiTestStrRef("/auth/me"),
            StdAdminApiTestStrRef(""),
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
            StdAdminApiTestStrRef("/auth/sign-in"),
            StdAdminApiTestStrRef("{\"login\":\"limited_user\",\"password\":\"limited-password\"}"),
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
            StdAdminApiTestStrRef("/users"),
            StdAdminApiTestStrRef(""),
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
            StdAdminApiTestStrRef("/roles"),
            StdAdminApiTestStrRef(""),
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
            StdAdminApiTestStrRef("/roles"),
            StdAdminApiTestStrRef("{\"name\":\"temporary_role\"}"),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("6d9384fe");
    assert_eq!(create_role_response.status(), http::StatusCode::CREATED);
    let role_id =
        sqlx::query_scalar::<_, i64>("SELECT id FROM admin_roles WHERE name = 'temporary_role'")
            .fetch_one(&pool.0)
            .await
            .expect("1e53a0c7");
    let update_role_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod(http::Method::PATCH),
            StdAdminApiTestStrRef(format!("/roles/{role_id}").as_str()),
            StdAdminApiTestStrRef("{\"name\":\"renamed_role\"}"),
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
            StdAdminApiTestStrRef(""),
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
            StdAdminApiTestStrRef(""),
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
            StdAdminApiTestStrRef(""),
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
            StdAdminApiTestStrRef("/audit-log"),
            StdAdminApiTestStrRef(""),
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
            StdAdminApiTestStrRef("/auth/sign-out"),
            StdAdminApiTestStrRef(""),
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
            StdAdminApiTestStrRef("/auth/me"),
            StdAdminApiTestStrRef(""),
            Some(StdAdminApiTestStrRef(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("54b9dc03");
    assert_eq!(revoked_response.status(), http::StatusCode::UNAUTHORIZED);
    let audit_outcomes = sqlx::query_as::<_, (bool, i64)>(
        "SELECT succeeded, COUNT(*) FROM admin_audit_log GROUP BY succeeded ORDER BY succeeded",
    )
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
