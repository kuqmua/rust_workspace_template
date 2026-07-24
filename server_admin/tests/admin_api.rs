#![allow(unused_crate_dependencies)]
// integration target inherits the library dependency graph while exercising the assembled public router
#![allow(clippy::tests_outside_test_module)] // every item in this integration target is compiled exclusively by the test harness
#[derive(Clone, Copy, newtype::FromInner)]
struct StdAdminApiTestStrRef<'value_lt>(&'value_lt str);
#[derive(newtype::FromInner)]
struct AxumAdminApiTestRouter(axum::Router);
#[derive(newtype::FromInner)]
struct SqlxAdminApiTestPool(sqlx::PgPool);
#[derive(newtype::FromInner)]
struct SqlxAdminHtmlTestTransaction(sqlx::Transaction<'static, sqlx::Postgres>);
#[derive(newtype::FromInner)]
struct HttpAdminApiTestMethod(http::Method);
#[derive(newtype::FromInner)]
struct HttpAdminApiTestRequest(http::Request<axum::body::Body>);
#[derive(newtype::DerefInner, newtype::FromInner)]
struct HttpAdminHtmlTestResponse(http::Response<axum::body::Body>);
#[derive(Clone, Copy, newtype::FromInner)]
struct HttpAdminApiTestResponseRef<'value_lt>(&'value_lt http::Response<axum::body::Body>);
#[derive(newtype::BoundedString)]
#[bounded_string(max = 16384)]
#[derive(newtype::Display)]
struct StdAdminApiTestCookie(String);
#[derive(newtype::BoundedString)]
#[bounded_string(max = 1_048_576)]
struct AdminHtmlTestBody(String);
#[derive(newtype::BoundedString)]
#[bounded_string(max = 65_536)]
struct AdminHtmlTestFormBody(String);
struct AdminHtmlTestFixture {
    cookie: StdAdminApiTestCookie,
    csrf: StdAdminApiTestCookie,
    lock: SqlxAdminHtmlTestTransaction,
    pool: SqlxAdminApiTestPool,
    router: AxumAdminApiTestRouter,
}
#[derive(Clone, Copy)]
struct AdminHtmlSettingsTestValues<'value_lt> {
    default_admin_route: StdAdminApiTestStrRef<'value_lt>,
    main_logo: StdAdminApiTestStrRef<'value_lt>,
    organization_contacts: StdAdminApiTestStrRef<'value_lt>,
    organization_name: StdAdminApiTestStrRef<'value_lt>,
    primary_color: StdAdminApiTestStrRef<'value_lt>,
    site_name: StdAdminApiTestStrRef<'value_lt>,
    support_url: StdAdminApiTestStrRef<'value_lt>,
    tab_title: StdAdminApiTestStrRef<'value_lt>,
}

impl AdminHtmlSettingsTestValues<'_> {
    fn form_body(self) -> AdminHtmlTestFormBody {
        AdminHtmlTestFormBody::try_from(format!(
            "default_admin_route={}&main_logo={}&organization_contacts={}&organization_name={}&primary_color={}&site_name={}&support_url={}&tab_title={}",
            self.default_admin_route.0,
            self.main_logo.0,
            self.organization_contacts.0,
            self.organization_name.0,
            self.primary_color.0,
            self.site_name.0,
            self.support_url.0,
            self.tab_title.0,
        ))
        .expect("c2af6158")
    }
}

fn one_admin_role_id(
    value: server_admin_contract::AdminRoleId,
) -> server_admin_contract::AdminRoleIds {
    server_admin_contract::AdminRoleIds::try_from(vec![value]).expect("69bc51bc")
}
fn empty_admin_role_ids() -> server_admin_contract::AdminRoleIds {
    server_admin_contract::AdminRoleIds::try_from(Vec::new()).expect("d5ccd621")
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
        .connect_lazy(str_constants::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION)
        .expect("27db915c");
    let state = server_admin::auth::AdminAuthSvcState::try_new(
        app_state::SqlxPgPool::from(pool),
        &env::<config_lib::AdminJwtSecret>(StdAdminApiTestStrRef::from(
            str_constants::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES,
        )),
        &env::<config_lib::AdminAccessTokenTtlSeconds>(StdAdminApiTestStrRef::from(
            str_constants::VALUE_900,
        )),
        &env::<config_lib::AdminRefreshTokenTtlSeconds>(StdAdminApiTestStrRef::from(
            str_constants::VALUE_3600,
        )),
        &env::<config_lib::AdminSessionLimit>(StdAdminApiTestStrRef::from(str_constants::VALUE_20)),
        &env::<config_lib::AdminSignInRateLimit>(StdAdminApiTestStrRef::from(
            str_constants::VALUE_2,
        )),
        &env::<config_lib::AdminPasswordHashConcurrency>(StdAdminApiTestStrRef::from(
            str_constants::VALUE_1,
        )),
        &env::<config_lib::AdminCookieSecure>(StdAdminApiTestStrRef::from(str_constants::FALSE)),
        &env::<config_lib::AdminTokenIssuer>(StdAdminApiTestStrRef::from(
            str_constants::INTEGRATION_TEST,
        )),
        &env::<config_lib::AdminTokenAudience>(StdAdminApiTestStrRef::from(
            str_constants::INTEGRATION_TEST_ADMIN,
        )),
        &config_lib::CorsAllowOrigin(str_constants::HTTP_LOCALHOST.to_owned()),
    )
    .expect("f7d8c961");
    AxumAdminApiTestRouter::from(axum::Router::from(server_admin::auth::routes(
        server_admin::auth::StdSharedAdminAuthSvcState::from(std::sync::Arc::new(state)),
    )))
}
fn router_with_pool(pool: &SqlxAdminApiTestPool) -> AxumAdminApiTestRouter {
    let state = server_admin::auth::AdminAuthSvcState::try_new(
        app_state::SqlxPgPool::from(pool.0.clone()),
        &env::<config_lib::AdminJwtSecret>(StdAdminApiTestStrRef::from(
            str_constants::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES,
        )),
        &env::<config_lib::AdminAccessTokenTtlSeconds>(StdAdminApiTestStrRef::from(
            str_constants::VALUE_900,
        )),
        &env::<config_lib::AdminRefreshTokenTtlSeconds>(StdAdminApiTestStrRef::from(
            str_constants::VALUE_3600,
        )),
        &env::<config_lib::AdminSessionLimit>(StdAdminApiTestStrRef::from(str_constants::VALUE_20)),
        &env::<config_lib::AdminSignInRateLimit>(StdAdminApiTestStrRef::from(
            str_constants::VALUE_2,
        )),
        &env::<config_lib::AdminPasswordHashConcurrency>(StdAdminApiTestStrRef::from(
            str_constants::VALUE_1,
        )),
        &env::<config_lib::AdminCookieSecure>(StdAdminApiTestStrRef::from(str_constants::FALSE)),
        &env::<config_lib::AdminTokenIssuer>(StdAdminApiTestStrRef::from(
            str_constants::INTEGRATION_TEST,
        )),
        &env::<config_lib::AdminTokenAudience>(StdAdminApiTestStrRef::from(
            str_constants::INTEGRATION_TEST_ADMIN,
        )),
        &config_lib::CorsAllowOrigin(str_constants::HTTP_LOCALHOST.to_owned()),
    )
    .expect("a59d73c1");
    AxumAdminApiTestRouter::from(axum::Router::from(server_admin::auth::routes(
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
        StdAdminApiTestStrRef::from(str_constants::VALUE_127_0_0_1_43210),
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
        .header(http::header::CONTENT_TYPE, str_constants::APPLICATION_JSON)
        .header(http::header::ORIGIN, str_constants::HTTP_LOCALHOST);
    if let Some(value) = cookie {
        builder = builder.header(http::header::COOKIE, value.0);
    }
    if let Some(value) = csrf {
        builder = builder.header(str_constants::X_CSRF_TOKEN_ALT, value.0);
    }
    let mut request = builder
        .body(axum::body::Body::from(body.0.to_owned()))
        .expect("7d924f8a");
    let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
        peer.0.parse::<std::net::SocketAddr>().expect("d80fc31b"),
    ));
    HttpAdminApiTestRequest::from(request)
}
fn html_request_with_peer(
    method: HttpAdminApiTestMethod,
    uri: StdAdminApiTestStrRef<'_>,
    body: StdAdminApiTestStrRef<'_>,
    cookie: Option<StdAdminApiTestStrRef<'_>>,
) -> HttpAdminApiTestRequest {
    let mut builder = http::Request::builder()
        .method(method.0)
        .uri(uri.0)
        .header(
            http::header::CONTENT_TYPE,
            str_constants::APPLICATION_X_WWW_FORM_URLENCODED,
        )
        .header(http::header::ORIGIN, str_constants::HTTP_LOCALHOST);
    if let Some(value) = cookie {
        builder = builder.header(http::header::COOKIE, value.0);
    }
    let mut request = builder
        .body(axum::body::Body::from(body.0.to_owned()))
        .expect("9f211b84");
    let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
        str_constants::VALUE_127_0_0_1_43210
            .parse::<std::net::SocketAddr>()
            .expect("bcd41a67"),
    ));
    HttpAdminApiTestRequest::from(request)
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
async fn admin_html_response(
    fixture: &AdminHtmlTestFixture,
    method: HttpAdminApiTestMethod,
    uri: StdAdminApiTestStrRef<'_>,
    body: StdAdminApiTestStrRef<'_>,
) -> HttpAdminHtmlTestResponse {
    tower::ServiceExt::oneshot(
        fixture.router.0.clone(),
        html_request_with_peer(
            method,
            uri,
            body,
            Some(StdAdminApiTestStrRef::from(fixture.cookie.0.as_str())),
        )
        .0,
    )
    .await
    .map(HttpAdminHtmlTestResponse::from)
    .expect("3cb98672")
}
async fn admin_html_body(response: HttpAdminHtmlTestResponse) -> AdminHtmlTestBody {
    axum::body::to_bytes(response.0.into_body(), 1_048_576usize)
        .await
        .map(|bytes| String::from_utf8(bytes.to_vec()).expect("86547438"))
        .map(|body| AdminHtmlTestBody::try_from(body).expect("ec7261cd"))
        .expect("8b54de37")
}
fn assert_admin_csr_shell(body: &AdminHtmlTestBody) {
    assert!(
        body.0.contains("id=\"admin-csr-root\""),
        "CSR root is missing"
    );
    assert!(
        body.0
            .contains("src=\"/admin/assets/csr_bootstrap.js?v=20260723-01\""),
        "CSR bootstrap script is missing"
    );
    assert!(!body.0.contains("<table"), "server rendered a data table");
    assert!(!body.0.contains("<form"), "server rendered a data form");
}
#[expect(
    clippy::missing_assert_message,
    reason = "the asserted status identifies the failed fixture stage"
)]
async fn admin_html_test_fixture() -> AdminHtmlTestFixture {
    let database_url = std::env::var(str_constants::ENV_NAMES_DATABASE_URL).expect("fbe54d19");
    let pool = SqlxAdminApiTestPool::from(
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5u32)
            .connect(database_url.as_str())
            .await
            .expect("ac089d31"),
    );
    let mut lock = pool.0.begin().await.expect("37480e56");
    let _locked = sqlx::query(str_constants::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
        .execute(&mut *lock)
        .await
        .expect("a6b7c8d9");
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool.0))
        .await
        .expect("45de3a61");
    let _truncated = sqlx::query(
        str_constants::TRUNCATE_ADMIN_RATE_LIMITS_ADMIN_AUDIT_LOG_ADMIN_LOGIN_ATTEMPTS_ADMIN_ACCESS,
    )
    .execute(&pool.0)
    .await
    .expect("cf37a9e2");
    let password =
        serde_json::from_str::<server_admin::AdminPassword>(str_constants::CORRECT_PASSWORD)
            .expect("d20a35e4");
    let hasher = server_admin::AdminPasswordHasher::new(
        server_admin::AdminPasswordHashConcurrency::from(server_admin::StdAdminNonZeroUsize::from(
            std::num::NonZeroUsize::new(1usize).expect("560498ab"),
        )),
    );
    let _created_admin_id = server_admin::bootstrap_admin(
        app_state::SqlxPgPoolRef::from(&pool.0),
        server_admin::AdminLogin::try_from(str_constants::ADMIN_ALT.to_owned()).expect("6a417bde"),
        server_admin::AdminDisplayName::try_from(str_constants::ADMIN.to_owned())
            .expect("703fc568"),
        password,
        &hasher,
    )
    .await
    .expect("1e29c87f");
    let state = server_admin::auth::AdminAuthSvcState::try_new(
        app_state::SqlxPgPool::from(pool.0.clone()),
        &env::<config_lib::AdminJwtSecret>(StdAdminApiTestStrRef::from(
            str_constants::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES,
        )),
        &env::<config_lib::AdminAccessTokenTtlSeconds>(StdAdminApiTestStrRef::from(
            str_constants::VALUE_900,
        )),
        &env::<config_lib::AdminRefreshTokenTtlSeconds>(StdAdminApiTestStrRef::from(
            str_constants::VALUE_3600,
        )),
        &env::<config_lib::AdminSessionLimit>(StdAdminApiTestStrRef::from(str_constants::VALUE_20)),
        &env::<config_lib::AdminSignInRateLimit>(StdAdminApiTestStrRef::from(
            str_constants::VALUE_20,
        )),
        &env::<config_lib::AdminPasswordHashConcurrency>(StdAdminApiTestStrRef::from(
            str_constants::VALUE_1,
        )),
        &env::<config_lib::AdminCookieSecure>(StdAdminApiTestStrRef::from(str_constants::FALSE)),
        &env::<config_lib::AdminTokenIssuer>(StdAdminApiTestStrRef::from(
            str_constants::INTEGRATION_TEST,
        )),
        &env::<config_lib::AdminTokenAudience>(StdAdminApiTestStrRef::from(
            str_constants::INTEGRATION_TEST_ADMIN,
        )),
        &config_lib::CorsAllowOrigin(str_constants::HTTP_LOCALHOST.to_owned()),
    )
    .expect("ec39b61d");
    let router = AxumAdminApiTestRouter::from(axum::Router::from(
        server_admin::auth::html_routes_with_swagger(
            server_admin::auth::StdSharedAdminAuthSvcState::from(std::sync::Arc::new(state)),
            server_admin::auth::AdminHtmlSwaggerEnabled::from(true),
        ),
    ));
    let correct_password =
        serde_json::from_str::<String>(str_constants::CORRECT_PASSWORD).expect("825e50c7");
    let sign_in_body = AdminHtmlTestFormBody::try_from(format!(
        "login={}&password={correct_password}",
        str_constants::ADMIN_ALT,
    ))
    .expect("9df2164c");
    let sign_in_response = tower::ServiceExt::oneshot(
        router.0.clone(),
        html_request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::SignIn.get()),
            StdAdminApiTestStrRef::from(sign_in_body.0.as_str()),
            None,
        )
        .0,
    )
    .await
    .expect("68a2cb40");
    assert_eq!(sign_in_response.status(), http::StatusCode::SEE_OTHER);
    let access = cookie_value(
        HttpAdminApiTestResponseRef::from(&sign_in_response),
        StdAdminApiTestStrRef::from(str_constants::ADMIN_ACCESS_TOKEN),
    );
    let refresh = cookie_value(
        HttpAdminApiTestResponseRef::from(&sign_in_response),
        StdAdminApiTestStrRef::from(str_constants::ADMIN_REFRESH_TOKEN_ALT),
    );
    let csrf = cookie_value(
        HttpAdminApiTestResponseRef::from(&sign_in_response),
        StdAdminApiTestStrRef::from(str_constants::ADMIN_CSRF_TOKEN_ALT),
    );
    AdminHtmlTestFixture {
        cookie: StdAdminApiTestCookie::try_from(format!(
            "{}{access}; {}{refresh}; {}{csrf}",
            str_constants::ADMIN_ACCESS_TOKEN,
            str_constants::ADMIN_REFRESH_TOKEN_ALT,
            str_constants::ADMIN_CSRF_TOKEN_ALT,
        ))
        .expect("a4df94d1"),
        csrf,
        lock: SqlxAdminHtmlTestTransaction::from(lock),
        pool,
        router,
    }
}
async fn postgres_accepts_admin_user_policy_values(
    pool: &SqlxAdminApiTestPool,
    display_name: StdAdminApiTestStrRef<'_>,
    login: StdAdminApiTestStrRef<'_>,
) -> server_admin_contract::AdminBool {
    let mut transaction = pool.0.begin().await.expect("e6f2cdf7");
    let accepted = sqlx::query(str_constants::INSERT_ADMIN_USER_POLICY_PROBE)
        .bind(login.0)
        .bind(display_name.0)
        .bind(str_constants::X)
        .execute(&mut *transaction)
        .await
        .is_ok();
    transaction.rollback().await.expect("fc4eec8f");
    server_admin_contract::AdminBool::from(accepted)
}
async fn postgres_accepts_admin_role_policy_value(
    pool: &SqlxAdminApiTestPool,
    name: StdAdminApiTestStrRef<'_>,
) -> server_admin_contract::AdminBool {
    let mut transaction = pool.0.begin().await.expect("77c2db82");
    let accepted = sqlx::query(str_constants::INSERT_ADMIN_ROLE_POLICY_PROBE)
        .bind(name.0)
        .execute(&mut *transaction)
        .await
        .is_ok();
    transaction.rollback().await.expect("aa9b0106");
    server_admin_contract::AdminBool::from(accepted)
}
#[test]
fn generated_admin_users_descriptor_keeps_sensitive_and_server_owned_fields_excluded() {
    let read_excluded = <server_admin::generated_tables::AdminUsers as pg_crud_common::DbTableSchema>::read_excluded_columns();
    assert!(
        read_excluded
            .iter()
            .any(|field| field.as_ref() == str_constants::PASSWORD_HASH)
    );
    let create_excluded = <server_admin::generated_tables::AdminUsers as pg_crud_common::DbTableSchema>::create_excluded_columns();
    assert!(
        create_excluded
            .iter()
            .any(|field| field.as_ref() == str_constants::PASSWORD_HASH)
    );
}
#[tokio::test]
async fn protected_routes_reject_missing_authentication_without_database_io() {
    let users_response = tower::ServiceExt::oneshot(
        router().0,
        http::Request::builder()
            .uri(
                frontend_contract::typed_route_path::<server_admin_contract::AdminMeRoute>()
                    .as_ref(),
            )
            .body(axum::body::Body::empty())
            .expect("b319e84d"),
    )
    .await
    .expect("0ac617de");
    assert_eq!(users_response.status(), http::StatusCode::UNAUTHORIZED);
    let response = tower::ServiceExt::oneshot(
        router().0,
        http::Request::builder()
            .uri(
                frontend_contract::typed_route_path::<server_admin_contract::AdminListUsersRoute>()
                    .as_ref(),
            )
            .body(axum::body::Body::empty())
            .expect("895e12fc"),
    )
    .await
    .expect("1fe80ad3");
    assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
}
#[tokio::test]
#[allow(
    clippy::needless_for_each,
    reason = "repository policy requires iterator methods instead of for loops"
)]
async fn runtime_auth_router_contains_every_open_api_operation() {
    let document = serde_json::to_value(utoipa::openapi::OpenApi::from(
        server_admin::auth::open_api(),
    ))
    .expect("71599514");
    let paths = document
        .get(str_constants::PATHS)
        .and_then(serde_json::Value::as_object)
        .expect("d908872f");
    let responses = futures::future::join_all(
        paths
            .iter()
            .flat_map(|(documented_path, path_item)| {
                path_item
                    .as_object()
                    .into_iter()
                    .flat_map(|operation_map| operation_map.keys())
                    .map(move |method| (documented_path, method))
            })
            .map(|(path, method)| (path.to_owned(), method.to_owned()))
            .map(|(documented_path, documented_method)| {
                let runtime_path = documented_path
                    .replace(
                        str_constants::ADMIN_SESSION_ID_PLACEHOLDER,
                        str_constants::VALUE_1,
                    )
                    .replace(
                        str_constants::ADMIN_USER_ID_PLACEHOLDER,
                        str_constants::VALUE_1,
                    )
                    .replace(
                        str_constants::ADMIN_ROLE_ID_PLACEHOLDER,
                        str_constants::VALUE_1,
                    );
                let method =
                    http::Method::from_bytes(documented_method.to_ascii_uppercase().as_bytes())
                        .expect("9d31a7e4");
                async move {
                    (
                        documented_method,
                        documented_path,
                        tower::ServiceExt::oneshot(
                            router().0,
                            http::Request::builder()
                                .method(method)
                                .uri(runtime_path)
                                .body(axum::body::Body::empty())
                                .expect("a3d6fb65"),
                        )
                        .await,
                    )
                }
            }),
    )
    .await;
    responses.into_iter().for_each(|(method, path, response)| {
        let status = response.expect("f7bd9f15").status();
        assert!(
            status != http::StatusCode::METHOD_NOT_ALLOWED && status != http::StatusCode::NOT_FOUND,
            "runtime router does not expose documented operation {method} {path}"
        );
    });
}
#[tokio::test]
async fn invalid_access_cookie_is_rejected_before_database_io() {
    let response = tower::ServiceExt::oneshot(
        router().0,
        http::Request::builder()
            .uri(
                frontend_contract::typed_route_path::<server_admin_contract::AdminMeRoute>()
                    .as_ref(),
            )
            .header(
                http::header::COOKIE,
                str_constants::ADMIN_ACCESS_TOKEN_INVALID_JWT_TOKEN,
            )
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
            .uri(str_constants::NOT_AN_API_ROUTE)
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
            .uri(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            )
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
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::LOGIN_ALT),
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
    let body_limit = <server_admin_contract::AdminAuthenticationRouteFamily as frontend_contract::RouteFamily>::body_limit()
        .expect("a60751db")
        .get();
    let oversized_password = str_constants::X.repeat(body_limit.saturating_add(1usize));
    let oversized_body = format!(r#"{{"login":"admin","password":"{oversized_password}"}}"#);
    let oversized_response = tower::ServiceExt::oneshot(
        router().0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(oversized_body.as_str()),
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
            .uri(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            )
            .header(http::header::CONTENT_TYPE, str_constants::APPLICATION_JSON);
        if let Some(value) = origin {
            builder = builder.header(http::header::ORIGIN, value);
        }
        if let Some(value) = referer {
            builder = builder.header(http::header::REFERER, value);
        }
        let mut request = builder
            .body(axum::body::Body::from(
                str_constants::LOGIN_ADMIN_PASSWORD_PASSWORD,
            ))
            .expect("168060a3");
        let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
            str_constants::VALUE_127_0_0_1_43210
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
            Some(str_constants::HTTP_BLOCKED_EXAMPLE),
            Some(str_constants::HTTP_LOCALHOST_ADMIN_SIGN_IN),
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
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_html_users_crud_covers_every_frontend_field_separately() {
    let fixture = admin_html_test_fixture().await;
    assert!(fixture.cookie.0.contains(fixture.csrf.0.as_str()));
    let login = "html_crud_user";
    let updated_login = "html_crud_user_updated";
    let display_name = "HTML CRUD User";
    let updated_display_name = "HTML CRUD User Updated";
    let password = "Html-crud-pass1";
    let updated_password = "Html-crud-pass2";
    let create_body = AdminHtmlTestFormBody::try_from(format!(
        "login={login}&display_name=HTML+CRUD+User&password={password}"
    ))
    .expect("801d9a43");
    let create_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserCreate.get()),
        StdAdminApiTestStrRef::from(create_body.0.as_str()),
    )
    .await;
    assert_eq!(create_response.status(), http::StatusCode::SEE_OTHER);
    let created = sqlx::query_as::<_, (i64, String, String, bool)>(
        "SELECT id, login, display_name, is_banned FROM users WHERE login = $1",
    )
    .bind(login)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("5de4fc12");
    assert_eq!(created.1, login);
    assert_eq!(created.2, display_name);
    assert!(!created.3);
    let users_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Users.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(users_response.status(), http::StatusCode::OK);
    let users_html = admin_html_body(users_response).await;
    assert_admin_csr_shell(&users_html);

    let login_update_body = AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&login={updated_login}&display_name=HTML+CRUD+User",
        created.0
    ))
    .expect("b0714f29");
    let login_update_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserUpdate.get()),
        StdAdminApiTestStrRef::from(login_update_body.0.as_str()),
    )
    .await;
    assert_eq!(login_update_response.status(), http::StatusCode::SEE_OTHER);
    let login_update = sqlx::query_as::<_, (String, String)>(
        "SELECT login, display_name FROM users WHERE id = $1",
    )
    .bind(created.0)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("68fae270");
    assert_eq!(
        login_update,
        (updated_login.to_owned(), display_name.to_owned())
    );

    let display_update_body = AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&login={updated_login}&display_name=HTML+CRUD+User+Updated",
        created.0
    ))
    .expect("9a6eb324");
    let display_update_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserUpdate.get()),
        StdAdminApiTestStrRef::from(display_update_body.0.as_str()),
    )
    .await;
    assert_eq!(
        display_update_response.status(),
        http::StatusCode::SEE_OTHER
    );
    let display_update = sqlx::query_as::<_, (String, String)>(
        "SELECT login, display_name FROM users WHERE id = $1",
    )
    .bind(created.0)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("10df386a");
    assert_eq!(
        display_update,
        (updated_login.to_owned(), updated_display_name.to_owned())
    );

    let password_update_body = AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&password={updated_password}",
        created.0
    ))
    .expect("cd82f375");
    let password_update_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserPassword.get()),
        StdAdminApiTestStrRef::from(password_update_body.0.as_str()),
    )
    .await;
    assert_eq!(
        password_update_response.status(),
        http::StatusCode::SEE_OTHER
    );
    let old_sign_in_body =
        AdminHtmlTestFormBody::try_from(format!("login={updated_login}&password={password}"))
            .expect("8c42d7e1");
    let old_sign_in_response = tower::ServiceExt::oneshot(
        fixture.router.0.clone(),
        html_request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::SignIn.get()),
            StdAdminApiTestStrRef::from(old_sign_in_body.0.as_str()),
            None,
        )
        .0,
    )
    .await
    .expect("26ab3584");
    assert_eq!(
        old_sign_in_response.status(),
        http::StatusCode::UNAUTHORIZED
    );
    let new_sign_in_body = AdminHtmlTestFormBody::try_from(format!(
        "login={updated_login}&password={updated_password}"
    ))
    .expect("ef05a691");
    let new_sign_in_response = tower::ServiceExt::oneshot(
        fixture.router.0.clone(),
        html_request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::SignIn.get()),
            StdAdminApiTestStrRef::from(new_sign_in_body.0.as_str()),
            None,
        )
        .0,
    )
    .await
    .expect("b9306c2e");
    assert_eq!(new_sign_in_response.status(), http::StatusCode::SEE_OTHER);

    let role_id = sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("f1674ab9");
    let roles_update_body = AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&expected_role_ids=&role_{role_id}={role_id}",
        created.0
    ))
    .expect("410e6a8c");
    let roles_update_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserRoles.get()),
        StdAdminApiTestStrRef::from(roles_update_body.0.as_str()),
    )
    .await;
    assert_eq!(roles_update_response.status(), http::StatusCode::SEE_OTHER);
    let assigned_roles =
        sqlx::query_scalar::<_, i64>("SELECT role_id FROM user_roles WHERE user_id = $1")
            .bind(created.0)
            .fetch_all(&fixture.pool.0)
            .await
            .expect("739cb4f5");
    assert_eq!(assigned_roles, vec![role_id]);

    let ban_body = AdminHtmlTestFormBody::try_from(format!("user_id={}&is_banned=true", created.0))
        .expect("a17fdc64");
    let ban_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserBan.get()),
        StdAdminApiTestStrRef::from(ban_body.0.as_str()),
    )
    .await;
    assert_eq!(ban_response.status(), http::StatusCode::SEE_OTHER);
    let final_users_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Users.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    let final_users_html = admin_html_body(final_users_response).await;
    assert_admin_csr_shell(&final_users_html);
    let unban_body =
        AdminHtmlTestFormBody::try_from(format!("user_id={}&is_banned=false", created.0))
            .expect("9d304db3");
    let unban_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserBan.get()),
        StdAdminApiTestStrRef::from(unban_body.0.as_str()),
    )
    .await;
    assert_eq!(unban_response.status(), http::StatusCode::SEE_OTHER);
    let is_banned = sqlx::query_scalar::<_, bool>("SELECT is_banned FROM users WHERE id = $1")
        .bind(created.0)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("55208887");
    assert!(!is_banned);
    let roles_clear_body = AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&expected_role_ids={role_id}",
        created.0
    ))
    .expect("04b638dc");
    let roles_clear_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserRoles.get()),
        StdAdminApiTestStrRef::from(roles_clear_body.0.as_str()),
    )
    .await;
    assert_eq!(roles_clear_response.status(), http::StatusCode::SEE_OTHER);

    let delete_body =
        AdminHtmlTestFormBody::try_from(format!("user_id={}&confirmation=true", created.0))
            .expect("d4fe3069");
    let delete_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserDelete.get()),
        StdAdminApiTestStrRef::from(delete_body.0.as_str()),
    )
    .await;
    assert_eq!(delete_response.status(), http::StatusCode::SEE_OTHER);
    let deleted_count = sqlx::query_scalar::<_, i64>("SELECT count(*) FROM users WHERE id = $1")
        .bind(created.0)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("72c950ea");
    assert_eq!(deleted_count, 0i64);
    let deleted_users_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Users.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    let deleted_users_html = admin_html_body(deleted_users_response).await;
    assert_admin_csr_shell(&deleted_users_html);
    fixture.lock.0.rollback().await.expect("93db561a");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_html_roles_crud_covers_every_frontend_field_separately() {
    let fixture = admin_html_test_fixture().await;
    let role_name = "html_crud_role";
    let updated_role_name = "html_crud_role_updated";
    let create_body =
        AdminHtmlTestFormBody::try_from(format!("name={role_name}")).expect("c593e840");
    let create_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RoleCreate.get()),
        StdAdminApiTestStrRef::from(create_body.0.as_str()),
    )
    .await;
    assert_eq!(create_response.status(), http::StatusCode::SEE_OTHER);
    let created = sqlx::query_as::<_, (i64, String, bool)>(
        "SELECT id, name, is_system FROM roles WHERE name = $1",
    )
    .bind(role_name)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("196fbd27");
    assert_eq!(created.1, role_name);
    assert!(!created.2);
    let roles_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Roles.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(roles_response.status(), http::StatusCode::OK);
    let roles_html = admin_html_body(roles_response).await;
    assert_admin_csr_shell(&roles_html);

    let update_body =
        AdminHtmlTestFormBody::try_from(format!("role_id={}&name={updated_role_name}", created.0))
            .expect("7ea84503");
    let update_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RoleUpdate.get()),
        StdAdminApiTestStrRef::from(update_body.0.as_str()),
    )
    .await;
    assert_eq!(update_response.status(), http::StatusCode::SEE_OTHER);
    let updated = sqlx::query_scalar::<_, String>("SELECT name FROM roles WHERE id = $1")
        .bind(created.0)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("43f81d69");
    assert_eq!(updated, updated_role_name);

    let permission =
        sqlx::query_as::<_, (i64, String)>("SELECT id, name FROM permissions ORDER BY id LIMIT 1")
            .fetch_one(&fixture.pool.0)
            .await
            .expect("ba920f54");
    let permissions_body = AdminHtmlTestFormBody::try_from(format!(
        "role_id={}&expected_permission_ids=&permission_{}={}",
        created.0, permission.0, permission.0
    ))
    .expect("0d476c31");
    let permissions_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RolePermissions.get()),
        StdAdminApiTestStrRef::from(permissions_body.0.as_str()),
    )
    .await;
    assert_eq!(permissions_response.status(), http::StatusCode::SEE_OTHER);
    let assigned_permissions = sqlx::query_scalar::<_, i64>(
        "SELECT permission_id FROM role_permissions WHERE role_id = $1",
    )
    .bind(created.0)
    .fetch_all(&fixture.pool.0)
    .await
    .expect("82b0d9f3");
    assert_eq!(assigned_permissions, vec![permission.0]);
    let final_roles_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Roles.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    let final_roles_html = admin_html_body(final_roles_response).await;
    assert_admin_csr_shell(&final_roles_html);

    let delete_body =
        AdminHtmlTestFormBody::try_from(format!("role_id={}&confirmation=true", created.0))
            .expect("e1547a60");
    let delete_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RoleDelete.get()),
        StdAdminApiTestStrRef::from(delete_body.0.as_str()),
    )
    .await;
    assert_eq!(delete_response.status(), http::StatusCode::SEE_OTHER);
    let deleted_count = sqlx::query_scalar::<_, i64>("SELECT count(*) FROM roles WHERE id = $1")
        .bind(created.0)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("2db479f8");
    assert_eq!(deleted_count, 0i64);
    let deleted_roles_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Roles.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    let deleted_roles_html = admin_html_body(deleted_roles_response).await;
    assert_admin_csr_shell(&deleted_roles_html);
    fixture.lock.0.rollback().await.expect("674dc2a9");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_html_settings_updates_and_reads_every_field_separately() {
    let fixture = admin_html_test_fixture().await;
    let site_name_a = StdAdminApiTestStrRef::from("HtmlSiteA");
    let site_name_b = StdAdminApiTestStrRef::from("HtmlSiteB");
    let route_a =
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Users.get());
    let route_b = StdAdminApiTestStrRef::from("/admin/roles");
    let tab_title_a = StdAdminApiTestStrRef::from("HtmlTabA");
    let tab_title_b = StdAdminApiTestStrRef::from("HtmlTabB");
    let main_logo_a = StdAdminApiTestStrRef::from("https://example.com/logo-a.png");
    let main_logo_b = StdAdminApiTestStrRef::from("https://example.com/logo-b.png");
    let primary_color_a = StdAdminApiTestStrRef::from("#112233");
    let primary_color_b = StdAdminApiTestStrRef::from("#445566");
    let organization_name_a = StdAdminApiTestStrRef::from("HtmlOrgA");
    let organization_name_b = StdAdminApiTestStrRef::from("HtmlOrgB");
    let organization_contacts_a = StdAdminApiTestStrRef::from("ops-a@example.com");
    let organization_contacts_b = StdAdminApiTestStrRef::from("ops-b@example.com");
    let support_url_a = StdAdminApiTestStrRef::from("https://example.com/support-a");
    let support_url_b = StdAdminApiTestStrRef::from("https://example.com/support-b");
    let states = [
        AdminHtmlSettingsTestValues {
            default_admin_route: route_a,
            main_logo: main_logo_a,
            organization_contacts: organization_contacts_a,
            organization_name: organization_name_a,
            primary_color: primary_color_a,
            site_name: site_name_a,
            support_url: support_url_a,
            tab_title: tab_title_a,
        },
        AdminHtmlSettingsTestValues {
            site_name: site_name_b,
            ..AdminHtmlSettingsTestValues {
                default_admin_route: route_a,
                main_logo: main_logo_a,
                organization_contacts: organization_contacts_a,
                organization_name: organization_name_a,
                primary_color: primary_color_a,
                site_name: site_name_a,
                support_url: support_url_a,
                tab_title: tab_title_a,
            }
        },
        AdminHtmlSettingsTestValues {
            default_admin_route: route_b,
            main_logo: main_logo_a,
            organization_contacts: organization_contacts_a,
            organization_name: organization_name_a,
            primary_color: primary_color_a,
            site_name: site_name_b,
            support_url: support_url_a,
            tab_title: tab_title_a,
        },
        AdminHtmlSettingsTestValues {
            tab_title: tab_title_b,
            ..AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: main_logo_a,
                organization_contacts: organization_contacts_a,
                organization_name: organization_name_a,
                primary_color: primary_color_a,
                site_name: site_name_b,
                support_url: support_url_a,
                tab_title: tab_title_a,
            }
        },
        AdminHtmlSettingsTestValues {
            main_logo: main_logo_b,
            default_admin_route: route_b,
            organization_contacts: organization_contacts_a,
            organization_name: organization_name_a,
            primary_color: primary_color_a,
            site_name: site_name_b,
            support_url: support_url_a,
            tab_title: tab_title_b,
        },
        AdminHtmlSettingsTestValues {
            primary_color: primary_color_b,
            default_admin_route: route_b,
            main_logo: main_logo_b,
            organization_contacts: organization_contacts_a,
            organization_name: organization_name_a,
            site_name: site_name_b,
            support_url: support_url_a,
            tab_title: tab_title_b,
        },
        AdminHtmlSettingsTestValues {
            organization_name: organization_name_b,
            default_admin_route: route_b,
            main_logo: main_logo_b,
            organization_contacts: organization_contacts_a,
            primary_color: primary_color_b,
            site_name: site_name_b,
            support_url: support_url_a,
            tab_title: tab_title_b,
        },
        AdminHtmlSettingsTestValues {
            organization_contacts: organization_contacts_b,
            default_admin_route: route_b,
            main_logo: main_logo_b,
            organization_name: organization_name_b,
            primary_color: primary_color_b,
            site_name: site_name_b,
            support_url: support_url_a,
            tab_title: tab_title_b,
        },
        AdminHtmlSettingsTestValues {
            support_url: support_url_b,
            default_admin_route: route_b,
            main_logo: main_logo_b,
            organization_contacts: organization_contacts_b,
            organization_name: organization_name_b,
            primary_color: primary_color_b,
            site_name: site_name_b,
            tab_title: tab_title_b,
        },
    ];
    let fixture_ref = &fixture;
    futures::StreamExt::fold(futures::stream::iter(states), (), async |(), values| {
        let form_body = values.form_body();
        let update_response = admin_html_response(
            fixture_ref,
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                server_admin_contract::AdminHtmlAction::SettingsUpdate.get(),
            ),
            StdAdminApiTestStrRef::from(form_body.0.as_str()),
        )
        .await;
        assert_eq!(update_response.status(), http::StatusCode::SEE_OTHER);
        let read_response = admin_html_response(
            fixture_ref,
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Settings.get()),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(read_response.status(), http::StatusCode::OK);
        let read_html = admin_html_body(read_response).await;
        assert_admin_csr_shell(&read_html);
    })
    .await;
    let stored = sqlx::query_as::<
        _,
        (
            String,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    >(
        "SELECT site_name, default_admin_route, tab_title, main_logo, primary_color, organization_name, organization_contacts, support_url FROM system_settings WHERE id = 1",
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("a8f201de");
    assert_eq!(stored.0, site_name_b.0);
    assert_eq!(stored.1, route_b.0);
    assert_eq!(stored.2.as_deref(), Some(tab_title_b.0));
    assert_eq!(stored.3.as_deref(), Some(main_logo_b.0));
    assert_eq!(stored.4.as_deref(), Some(primary_color_b.0));
    assert_eq!(stored.5.as_deref(), Some(organization_name_b.0));
    assert_eq!(stored.6.as_deref(), Some(organization_contacts_b.0));
    assert_eq!(stored.7.as_deref(), Some(support_url_b.0));
    let empty = StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX);
    let clear_states = [
        (
            AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: main_logo_b,
                organization_contacts: organization_contacts_b,
                organization_name: organization_name_b,
                primary_color: primary_color_b,
                site_name: site_name_b,
                support_url: support_url_b,
                tab_title: empty,
            },
            1usize,
        ),
        (
            AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: empty,
                organization_contacts: organization_contacts_b,
                organization_name: organization_name_b,
                primary_color: primary_color_b,
                site_name: site_name_b,
                support_url: support_url_b,
                tab_title: empty,
            },
            2usize,
        ),
        (
            AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: empty,
                organization_contacts: organization_contacts_b,
                organization_name: organization_name_b,
                primary_color: empty,
                site_name: site_name_b,
                support_url: support_url_b,
                tab_title: empty,
            },
            3usize,
        ),
        (
            AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: empty,
                organization_contacts: organization_contacts_b,
                organization_name: empty,
                primary_color: empty,
                site_name: site_name_b,
                support_url: support_url_b,
                tab_title: empty,
            },
            4usize,
        ),
        (
            AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: empty,
                organization_contacts: empty,
                organization_name: empty,
                primary_color: empty,
                site_name: site_name_b,
                support_url: support_url_b,
                tab_title: empty,
            },
            5usize,
        ),
        (
            AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: empty,
                organization_contacts: empty,
                organization_name: empty,
                primary_color: empty,
                site_name: site_name_b,
                support_url: empty,
                tab_title: empty,
            },
            6usize,
        ),
    ];
    futures::StreamExt::fold(
        futures::stream::iter(clear_states),
        (),
        async |(), (values, expected_cleared)| {
            let form_body = values.form_body();
            let clear_response = admin_html_response(
                fixture_ref,
                HttpAdminApiTestMethod::from(http::Method::POST),
                StdAdminApiTestStrRef::from(
                    server_admin_contract::AdminHtmlAction::SettingsUpdate.get(),
                ),
                StdAdminApiTestStrRef::from(form_body.0.as_str()),
            )
            .await;
            assert_eq!(clear_response.status(), http::StatusCode::SEE_OTHER);
            let optional_values = sqlx::query_as::<
                _,
                (
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>,
                ),
            >(
                "SELECT tab_title, main_logo, primary_color, organization_name, organization_contacts, support_url FROM system_settings WHERE id = 1",
            )
            .fetch_one(&fixture_ref.pool.0)
            .await
            .expect("d418f9c0");
            assert_eq!(
                [
                    optional_values.0,
                    optional_values.1,
                    optional_values.2,
                    optional_values.3,
                    optional_values.4,
                    optional_values.5,
                ]
                .iter()
                .filter(|value| value.is_none())
                .count(),
                expected_cleared,
            );
        },
    )
    .await;
    fixture.lock.0.rollback().await.expect("c7659b40");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_html_profile_reads_every_field_and_changes_own_password() {
    let fixture = admin_html_test_fixture().await;
    let profile_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Profile.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(profile_response.status(), http::StatusCode::OK);
    let profile_html = admin_html_body(profile_response).await;
    assert_admin_csr_shell(&profile_html);

    let original_password_hash = sqlx::query_scalar::<_, String>(
        str_constants::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("c09b5e4e");
    let (current_session_id, user_id) = sqlx::query_as::<_, (uuid::Uuid, i64)>(
        "SELECT id, user_id FROM access_sessions WHERE revoked_at IS NULL",
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("ae46b7c1");
    let other_session_id = uuid::Uuid::from_u128(2u128);
    let _inserted_other_session = sqlx::query(
        "INSERT INTO access_sessions (id, user_id, token_identifier_hash, csrf_token_hash, token_context_hash, expires_at) VALUES ($1, $2, 'other-token-hash', 'other-csrf-hash', repeat('a', 64), NOW() + INTERVAL '1 hour')",
    )
    .bind(other_session_id)
    .bind(user_id)
    .execute(&fixture.pool.0)
    .await
    .expect("3e216ecd");
    let _inserted_other_refresh_token = sqlx::query(
        "INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at) VALUES ($1, $2, 'other-refresh-hash', NOW() + INTERVAL '1 hour')",
    )
    .bind(uuid::Uuid::from_u128(3u128))
    .bind(user_id)
    .execute(&fixture.pool.0)
    .await
    .expect("d61fc342");
    let correct_password =
        serde_json::from_str::<String>(str_constants::CORRECT_PASSWORD).expect("c59b011a");
    let change_password_body = AdminHtmlTestFormBody::try_from(format!(
        "current_password={correct_password}&new_password=Html-profile-pass2",
    ))
    .expect("c93d69e3");
    let change_password_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::ProfilePassword.get()),
        StdAdminApiTestStrRef::from(change_password_body.0.as_str()),
    )
    .await;
    assert_eq!(
        change_password_response.status(),
        http::StatusCode::SEE_OTHER
    );
    let changed_password_hash = sqlx::query_scalar::<_, String>(
        str_constants::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("696330ca");
    assert_ne!(changed_password_hash, original_password_hash);
    let current_session_revoked = sqlx::query_scalar::<_, bool>(
        "SELECT revoked_at IS NOT NULL FROM access_sessions WHERE id = $1",
    )
    .bind(current_session_id)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("38923e84");
    assert!(!current_session_revoked);
    let other_session_revoked = sqlx::query_scalar::<_, bool>(
        "SELECT revoked_at IS NOT NULL FROM access_sessions WHERE id = $1",
    )
    .bind(other_session_id)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("f0168dc5");
    assert!(other_session_revoked);
    let active_refresh_token_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM refresh_tokens WHERE revoked_at IS NULL",
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("740d6dc9");
    assert_eq!(active_refresh_token_count, 0i64);
    let authenticated_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Profile.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(authenticated_response.status(), http::StatusCode::OK);
    fixture.lock.0.rollback().await.expect("737bbbe6");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_html_sessions_reads_every_field_and_revokes_session() {
    let fixture = admin_html_test_fixture().await;
    let admin_id =
        sqlx::query_scalar::<_, i64>(str_constants::SELECT_ID_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN)
            .fetch_one(&fixture.pool.0)
            .await
            .expect("7f0a7c64");
    let (session_id, _created_at, _expires_at) = sqlx::query_as::<_, (uuid::Uuid, String, String)>(
        str_constants::SERVER_ADMIN_LIST_ACTIVE_SESSIONS_SQL,
    )
    .bind(admin_id)
    .bind(100i64)
    .bind(0i64)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("32e44a86");
    let sessions_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Sessions.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(sessions_response.status(), http::StatusCode::OK);
    let sessions_html = admin_html_body(sessions_response).await;
    assert_admin_csr_shell(&sessions_html);

    let revoke_body =
        AdminHtmlTestFormBody::try_from(format!("session_id={session_id}&confirmation=true"))
            .expect("2f8bea59");
    let revoke_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::SessionRevoke.get()),
        StdAdminApiTestStrRef::from(revoke_body.0.as_str()),
    )
    .await;
    assert_eq!(revoke_response.status(), http::StatusCode::SEE_OTHER);
    let revoked = sqlx::query_scalar::<_, bool>(
        "SELECT revoked_at IS NOT NULL FROM access_sessions WHERE id = $1",
    )
    .bind(session_id)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("e443902e");
    assert!(revoked);
    let rejected_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Sessions.get()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(rejected_response.status(), http::StatusCode::SEE_OTHER);
    fixture.lock.0.rollback().await.expect("9f41b8bd");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_html_router_registers_every_owned_page_and_action() {
    let fixture = admin_html_test_fixture().await;
    let fixture_ref = &fixture;
    futures::StreamExt::fold(
        futures::StreamExt::filter(
            futures::stream::iter(server_admin_contract::AdminFrontendPath::all_pages()),
            |path| {
                std::future::ready(!matches!(
                    path,
                    server_admin_contract::AdminFrontendPath::Metrics
                        | server_admin_contract::AdminFrontendPath::Permissions
                        | server_admin_contract::AdminFrontendPath::Roles
                        | server_admin_contract::AdminFrontendPath::Tables
                        | server_admin_contract::AdminFrontendPath::Users
                ))
            },
        ),
        (),
        async |(), path| {
            let response = admin_html_response(
                fixture_ref,
                HttpAdminApiTestMethod::from(http::Method::GET),
                StdAdminApiTestStrRef::from(path.get()),
                StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            )
            .await;
            assert!(
                !matches!(
                    response.status(),
                    http::StatusCode::NOT_FOUND
                        | http::StatusCode::METHOD_NOT_ALLOWED
                        | http::StatusCode::INTERNAL_SERVER_ERROR
                ),
                "frontend page {} returned {}",
                path.get(),
                response.status()
            );
            if matches!(
                path,
                server_admin_contract::AdminFrontendPath::Profile
                    | server_admin_contract::AdminFrontendPath::Sessions
                    | server_admin_contract::AdminFrontendPath::Settings
            ) {
                let html = admin_html_body(response).await;
                assert_admin_csr_shell(&html);
            }
        },
    )
    .await;
    futures::StreamExt::fold(
        futures::stream::iter(server_admin_contract::AdminDataTable::ALL),
        (),
        async |(), table| {
            let uri = table.frontend_path();
            let response = admin_html_response(
                fixture_ref,
                HttpAdminApiTestMethod::from(http::Method::GET),
                StdAdminApiTestStrRef::from(uri.as_ref()),
                StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            )
            .await;
            assert_eq!(
                response.status(),
                http::StatusCode::OK,
                "table view {table} failed"
            );
            let html = admin_html_body(response).await;
            assert_admin_csr_shell(&html);
        },
    )
    .await;
    futures::StreamExt::fold(
        futures::stream::iter(server_admin_contract::AdminHtmlAction::ALL),
        (),
        async |(), action| {
            let response = tower::ServiceExt::oneshot(
                fixture_ref.router.0.clone(),
                html_request_with_peer(
                    HttpAdminApiTestMethod::from(http::Method::POST),
                    StdAdminApiTestStrRef::from(action.get()),
                    StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
                    None,
                )
                .0,
            )
            .await
            .expect("d9567273");
            assert!(
                !matches!(
                    response.status(),
                    http::StatusCode::NOT_FOUND
                        | http::StatusCode::METHOD_NOT_ALLOWED
                        | http::StatusCode::INTERNAL_SERVER_ERROR
                ),
                "HTML action {} returned {}",
                action.get(),
                response.status()
            );
        },
    )
    .await;
    fixture.lock.0.rollback().await.expect("c0c53cdc");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering() {
    let fixture = admin_html_test_fixture().await;
    let unauthenticated_response = tower::ServiceExt::oneshot(
        fixture.router.0.clone(),
        html_request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(server_admin_contract::AdminFrontendPath::Users.get()),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            None,
        )
        .0,
    )
    .await
    .expect("184ec7b2");
    assert_eq!(
        unauthenticated_response.status(),
        http::StatusCode::SEE_OTHER
    );
    assert_eq!(
        unauthenticated_response
            .headers()
            .get(http::header::LOCATION),
        Some(&http::HeaderValue::from_static(
            server_admin_contract::AdminFrontendPath::SignIn.get(),
        )),
    );

    let login = "html_form_contract_user";
    let valid_body = AdminHtmlTestFormBody::try_from(format!(
        "login={login}&display_name=HTML+Form+Contract+User&password=Html-form-pass1"
    ))
    .expect("94b36ec1");
    let missing_csrf_response = tower::ServiceExt::oneshot(
        fixture.router.0.clone(),
        html_request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserCreate.get()),
            StdAdminApiTestStrRef::from(valid_body.0.as_str()),
            None,
        )
        .0,
    )
    .await
    .expect("e6013d7a");
    assert_eq!(missing_csrf_response.status(), http::StatusCode::FORBIDDEN);
    let unknown_field_body =
        AdminHtmlTestFormBody::try_from(format!("{}&unknown_field=true", valid_body.0))
            .expect("af2948d3");
    let unknown_field_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserCreate.get()),
        StdAdminApiTestStrRef::from(unknown_field_body.0.as_str()),
    )
    .await;
    assert_eq!(
        unknown_field_response.status(),
        http::StatusCode::UNPROCESSABLE_ENTITY
    );
    let create_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserCreate.get()),
        StdAdminApiTestStrRef::from(valid_body.0.as_str()),
    )
    .await;
    assert_eq!(create_response.status(), http::StatusCode::SEE_OTHER);
    let duplicate_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserCreate.get()),
        StdAdminApiTestStrRef::from(valid_body.0.as_str()),
    )
    .await;
    assert_eq!(duplicate_response.status(), http::StatusCode::CONFLICT);
    let created_id = sqlx::query_scalar::<_, i64>("SELECT id FROM users WHERE login = $1")
        .bind(login)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("378a4e50");
    let filtered_path = AdminHtmlTestFormBody::try_from(format!(
        "{}?search={login}",
        server_admin_contract::AdminFrontendPath::Users.get()
    ))
    .expect("60bf2c91");
    let filtered_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::GET),
        StdAdminApiTestStrRef::from(filtered_path.0.as_str()),
        StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
    )
    .await;
    assert_eq!(filtered_response.status(), http::StatusCode::OK);
    let filtered_html = admin_html_body(filtered_response).await;
    assert_admin_csr_shell(&filtered_html);

    let role_id = sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("bc10a764");
    let stale_roles_body = AdminHtmlTestFormBody::try_from(format!(
        "user_id={created_id}&expected_role_ids={role_id}"
    ))
    .expect("1934ad6f");
    let stale_roles_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserRoles.get()),
        StdAdminApiTestStrRef::from(stale_roles_body.0.as_str()),
    )
    .await;
    assert_eq!(stale_roles_response.status(), http::StatusCode::CONFLICT);

    let role_name = "html_form_contract_role";
    let create_role_body =
        AdminHtmlTestFormBody::try_from(format!("name={role_name}")).expect("8cf4260d");
    let create_role_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RoleCreate.get()),
        StdAdminApiTestStrRef::from(create_role_body.0.as_str()),
    )
    .await;
    assert_eq!(create_role_response.status(), http::StatusCode::SEE_OTHER);
    let duplicate_role_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RoleCreate.get()),
        StdAdminApiTestStrRef::from(create_role_body.0.as_str()),
    )
    .await;
    assert_eq!(duplicate_role_response.status(), http::StatusCode::CONFLICT);
    let created_role_id = sqlx::query_scalar::<_, i64>("SELECT id FROM roles WHERE name = $1")
        .bind(role_name)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("2643be19");
    let permission_id =
        sqlx::query_scalar::<_, i64>("SELECT id FROM permissions ORDER BY id LIMIT 1")
            .fetch_one(&fixture.pool.0)
            .await
            .expect("d8134c5b");
    let stale_permissions_body = AdminHtmlTestFormBody::try_from(format!(
        "role_id={created_role_id}&expected_permission_ids={permission_id}"
    ))
    .expect("49fac702");
    let stale_permissions_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RolePermissions.get()),
        StdAdminApiTestStrRef::from(stale_permissions_body.0.as_str()),
    )
    .await;
    assert_eq!(
        stale_permissions_response.status(),
        http::StatusCode::CONFLICT
    );
    let delete_role_body =
        AdminHtmlTestFormBody::try_from(format!("role_id={created_role_id}&confirmation=true"))
            .expect("f1c637d8");
    let delete_role_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::RoleDelete.get()),
        StdAdminApiTestStrRef::from(delete_role_body.0.as_str()),
    )
    .await;
    assert_eq!(delete_role_response.status(), http::StatusCode::SEE_OTHER);

    let unknown_delete_body = AdminHtmlTestFormBody::try_from(String::from(
        "user_id=9223372036854775807&confirmation=true",
    ))
    .expect("d96b20e4");
    let unknown_delete_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserDelete.get()),
        StdAdminApiTestStrRef::from(unknown_delete_body.0.as_str()),
    )
    .await;
    assert_eq!(unknown_delete_response.status(), http::StatusCode::CONFLICT);

    let delete_body =
        AdminHtmlTestFormBody::try_from(format!("user_id={created_id}&confirmation=true"))
            .expect("4cf9072d");
    let delete_response = admin_html_response(
        &fixture,
        HttpAdminApiTestMethod::from(http::Method::POST),
        StdAdminApiTestStrRef::from(server_admin_contract::AdminHtmlAction::UserDelete.get()),
        StdAdminApiTestStrRef::from(delete_body.0.as_str()),
    )
    .await;
    assert_eq!(delete_response.status(), http::StatusCode::SEE_OTHER);
    fixture.lock.0.rollback().await.expect("7361eb5c");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn generated_admin_descriptors_match_applied_migrations() {
    let database_url = std::env::var(str_constants::ENV_NAMES_DATABASE_URL).expect("7e62af41");
    let pool = SqlxAdminApiTestPool::from(
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect(database_url.as_str())
            .await
            .expect("20250c41"),
    );
    let mut admin_db_test_lock = pool.0.begin().await.expect("50eb5d64");
    let _locked = sqlx::query(str_constants::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
        .execute(&mut *admin_db_test_lock)
        .await
        .expect("77883cf4");
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool.0))
        .await
        .expect("9eceddf1");
    server_admin::generated_tables::validate_catalog_schema(
        pg_crud_common::SqlxPgPoolRef::from(&pool.0),
        pg_crud_common::DbSchemaNameRef::from(str_constants::PUBLIC),
    )
    .await
    .expect("7a31cf02");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn admin_string_policies_match_postgresql_constraints() {
    let database_url = std::env::var(str_constants::ENV_NAMES_DATABASE_URL).expect("93fcb3de");
    let pool = SqlxAdminApiTestPool::from(
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect(database_url.as_str())
            .await
            .expect("d48c868d"),
    );
    let mut admin_db_test_lock = pool.0.begin().await.expect("99ced936");
    let _locked = sqlx::query(str_constants::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
        .execute(&mut *admin_db_test_lock)
        .await
        .expect("168b689c");
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool.0))
        .await
        .expect("a453b862");
    let valid_login =
        server_admin_contract::AdminLogin::try_from(str_constants::SSOT_LOGIN_VALID.to_owned())
            .is_ok();
    assert_eq!(
        server_admin_contract::AdminBool::from(valid_login),
        postgres_accepts_admin_user_policy_values(
            &pool,
            StdAdminApiTestStrRef(str_constants::SSOT_DISPLAY_NAME_VALID),
            StdAdminApiTestStrRef(str_constants::SSOT_LOGIN_VALID),
        )
        .await
    );
    let invalid_login = server_admin_contract::AdminLogin::try_from(
        str_constants::SSOT_LOGIN_INVALID_CASE.to_owned(),
    )
    .is_ok();
    assert_eq!(
        server_admin_contract::AdminBool::from(invalid_login),
        postgres_accepts_admin_user_policy_values(
            &pool,
            StdAdminApiTestStrRef(str_constants::SSOT_DISPLAY_NAME_VALID),
            StdAdminApiTestStrRef(str_constants::SSOT_LOGIN_INVALID_CASE),
        )
        .await
    );
    let invalid_display = server_admin_contract::AdminDisplayName::try_from(
        str_constants::SSOT_DISPLAY_NAME_PADDED.to_owned(),
    )
    .is_ok();
    assert_eq!(
        server_admin_contract::AdminBool::from(invalid_display),
        postgres_accepts_admin_user_policy_values(
            &pool,
            StdAdminApiTestStrRef(str_constants::SSOT_DISPLAY_NAME_PADDED),
            StdAdminApiTestStrRef(str_constants::SSOT_LOGIN_VALID),
        )
        .await
    );
    let valid_role =
        server_admin_contract::AdminRoleName::try_from(str_constants::SSOT_ROLE_VALID.to_owned())
            .is_ok();
    assert_eq!(
        server_admin_contract::AdminBool::from(valid_role),
        postgres_accepts_admin_role_policy_value(
            &pool,
            StdAdminApiTestStrRef(str_constants::SSOT_ROLE_VALID),
        )
        .await
    );
    let invalid_role = server_admin_contract::AdminRoleName::try_from(
        str_constants::SSOT_ROLE_INVALID_CASE.to_owned(),
    )
    .is_ok();
    assert_eq!(
        server_admin_contract::AdminBool::from(invalid_role),
        postgres_accepts_admin_role_policy_value(
            &pool,
            StdAdminApiTestStrRef(str_constants::SSOT_ROLE_INVALID_CASE),
        )
        .await
    );
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_auth_rbac_csrf_session_and_audit_flow() {
    let database_url = std::env::var(str_constants::ENV_NAMES_DATABASE_URL).expect("ac0cb9e3");
    let pool = SqlxAdminApiTestPool::from(
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url.as_str())
            .await
            .expect("a3e1f57c"),
    );
    let mut admin_db_test_lock = pool.0.begin().await.expect("4dfb6865");
    let _locked = sqlx::query(str_constants::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
        .execute(&mut *admin_db_test_lock)
        .await
        .expect("693b147f");
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool.0))
        .await
        .expect("0ea8d516");
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool.0))
        .await
        .expect("676c00f1");
    server_admin::generated_tables::validate_catalog_schema(
        pg_crud_common::SqlxPgPoolRef::from(&pool.0),
        pg_crud_common::DbSchemaNameRef::from(str_constants::PUBLIC),
    )
    .await
    .expect("65ce07e9");
    let observed_permissions = sqlx::query_scalar::<_, String>(
        str_constants::SELECT_NAME_FROM_ADMIN_PERMISSIONS_ORDER_BY_NAME,
    )
    .fetch_all(&pool.0)
    .await
    .expect("db765f20");
    let expected_permissions = server_admin::AdminPermission::ALL
        .into_iter()
        .map(|permission| permission.as_str().as_ref().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(observed_permissions, expected_permissions);
    let _deleted_permission = sqlx::query(str_constants::DELETE_ADMIN_PERMISSION_BY_NAME)
        .bind(
            server_admin::AdminPermission::ALL
                .first()
                .expect("26d95ea4")
                .as_str()
                .as_ref(),
        )
        .execute(&pool.0)
        .await
        .expect("9d762f8c");
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool.0))
        .await
        .expect("ea3f641d");
    let reconciled_permissions = sqlx::query_scalar::<_, String>(
        str_constants::SELECT_NAME_FROM_ADMIN_PERMISSIONS_ORDER_BY_NAME,
    )
    .fetch_all(&pool.0)
    .await
    .expect("458ab19e");
    assert_eq!(reconciled_permissions, expected_permissions);
    let _truncate_result = sqlx::query(
        str_constants::TRUNCATE_ADMIN_RATE_LIMITS_ADMIN_AUDIT_LOG_ADMIN_LOGIN_ATTEMPTS_ADMIN_ACCESS,
    )
    .execute(&pool.0)
    .await
    .expect("97b5ad2f");
    let password =
        serde_json::from_str::<server_admin::AdminPassword>(str_constants::CORRECT_PASSWORD)
            .expect("703a8df2");
    let hasher = server_admin::AdminPasswordHasher::new(
        server_admin::AdminPasswordHashConcurrency::from(server_admin::StdAdminNonZeroUsize::from(
            std::num::NonZeroUsize::new(1).expect("271f96d4"),
        )),
    );
    let _admin_id = server_admin::bootstrap_admin(
        app_state::SqlxPgPoolRef::from(&pool.0),
        server_admin::AdminLogin::try_from(str_constants::ADMIN_ALT.to_owned()).expect("98c7e04a"),
        server_admin::AdminDisplayName::try_from(str_constants::ADMIN.to_owned())
            .expect("48efed01"),
        password,
        &hasher,
    )
    .await
    .expect("e2c94d67");
    let original_password_hash = sqlx::query_scalar::<_, String>(
        str_constants::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
    )
    .fetch_one(&pool.0)
    .await
    .expect("1282b56e");
    let repeated_password =
        serde_json::from_str::<server_admin::AdminPassword>(str_constants::DIFFERENT_PASSWORD)
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
    let preserved_password_hash = sqlx::query_scalar::<_, String>(
        str_constants::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
    )
    .fetch_one(&pool.0)
    .await
    .expect("65ff827e");
    assert_eq!(preserved_password_hash, original_password_hash);
    let administrator_count =
        sqlx::query_scalar::<_, i64>(str_constants::SELECT_COUNT_ASTERISK_FROM_ADMIN_USERS)
            .fetch_one(&pool.0)
            .await
            .expect("ae89c3bd");
    assert_eq!(administrator_count, 1i64);
    let admin_id =
        sqlx::query_scalar::<_, i64>(str_constants::SELECT_ID_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN)
            .fetch_one(&pool.0)
            .await
            .expect("a61329bf");
    let dangling_role_links = sqlx::query_scalar::<_, i64>(
        str_constants::SELECT_COUNT_ASTERISK_FROM_ADMIN_USER_ROLES_LINK_LEFT_JOIN_ADMIN_USERS,
    )
    .fetch_one(&pool.0)
    .await
    .expect("08ef120f");
    assert_eq!(dangling_role_links, 0i64);
    let dangling_permission_links = sqlx::query_scalar::<_, i64>(
        str_constants::SELECT_COUNT_ASTERISK_FROM_ADMIN_ROLE_PERMISSIONS_LINK_LEFT_JOIN_ADMIN_ROLES,
    )
    .fetch_one(&pool.0)
    .await
    .expect("aebf6dc8");
    assert_eq!(dangling_permission_links, 0i64);
    let wrong_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::LOGIN_ADMIN_PASSWORD_WRONG_PASSWORD),
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
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::LOGIN_ADMIN_PASSWORD_CORRECT_PASSWORD),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("c245193e");
    assert_eq!(sign_in_response.status(), http::StatusCode::OK);
    let access = cookie_value(
        HttpAdminApiTestResponseRef::from(&sign_in_response),
        StdAdminApiTestStrRef::from(str_constants::ADMIN_ACCESS_TOKEN),
    );
    let refresh = cookie_value(
        HttpAdminApiTestResponseRef::from(&sign_in_response),
        StdAdminApiTestStrRef::from(str_constants::ADMIN_REFRESH_TOKEN_ALT),
    );
    let csrf = cookie_value(
        HttpAdminApiTestResponseRef::from(&sign_in_response),
        StdAdminApiTestStrRef::from(str_constants::ADMIN_CSRF_TOKEN_ALT),
    );
    let cookie = format!(
        "admin_access_token={access}; admin_refresh_token={refresh}; admin_csrf_token={csrf}"
    );
    let me_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminMeRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(cookie.as_str())),
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
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminMeRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(cookie.as_str())),
            None,
            StdAdminApiTestStrRef::from(str_constants::VALUE_127_0_0_2_43210),
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
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminRefreshRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(first_refresh_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("9f0be285");
    assert_eq!(refresh_response.status(), http::StatusCode::OK);
    let refreshed_access = cookie_value(
        HttpAdminApiTestResponseRef::from(&refresh_response),
        StdAdminApiTestStrRef::from(str_constants::ADMIN_ACCESS_TOKEN),
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
        HttpAdminApiTestResponseRef::from(&refresh_response),
        StdAdminApiTestStrRef::from(str_constants::ADMIN_CSRF_TOKEN_ALT),
    );
    let active_cookie = format!(
        "admin_access_token={refreshed_access}; admin_refresh_token={refresh}; admin_csrf_token={refreshed_csrf}"
    );
    let reused_refresh_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminRefreshRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(first_refresh_cookie.as_str())),
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
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::LOGIN_LOCKED_USER_PASSWORD_WRONG_PASSWORD),
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
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::LOGIN_LOCKED_USER_PASSWORD_WRONG_PASSWORD),
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
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::LOGIN_LOCKED_USER_PASSWORD_WRONG_PASSWORD),
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
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(frontend_contract::typed_route_path::<server_admin_contract::AdminListUsersRoute>().as_ref()),
            StdAdminApiTestStrRef::from(str_constants::LOGIN_LIMITED_USER_DISPLAY_NAME_LIMITED_USER_PASSWORD_LIMITED_PASSWORD),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
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
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(frontend_contract::typed_route_path::<server_admin_contract::AdminListUsersRoute>().as_ref()),
            StdAdminApiTestStrRef::from(str_constants::LOGIN_LIMITED_USER_DISPLAY_NAME_LIMITED_USER_PASSWORD_LIMITED_PASSWORD),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("c86a4310");
    assert_eq!(create_response.status(), http::StatusCode::CREATED);
    let limited_sign_in_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(
                str_constants::LOGIN_LIMITED_USER_PASSWORD_LIMITED_PASSWORD,
            ),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("a2d6139e");
    assert_eq!(limited_sign_in_response.status(), http::StatusCode::OK);
    let limited_access = cookie_value(
        HttpAdminApiTestResponseRef::from(&limited_sign_in_response),
        StdAdminApiTestStrRef::from(str_constants::ADMIN_ACCESS_TOKEN),
    );
    let limited_refresh = cookie_value(
        HttpAdminApiTestResponseRef::from(&limited_sign_in_response),
        StdAdminApiTestStrRef::from(str_constants::ADMIN_REFRESH_TOKEN_ALT),
    );
    let limited_csrf = cookie_value(
        HttpAdminApiTestResponseRef::from(&limited_sign_in_response),
        StdAdminApiTestStrRef::from(str_constants::ADMIN_CSRF_TOKEN_ALT),
    );
    let limited_cookie = format!(
        "admin_access_token={limited_access}; admin_refresh_token={limited_refresh}; admin_csrf_token={limited_csrf}"
    );
    let forbidden_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminListUsersRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(limited_cookie.as_str())),
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
            HttpAdminApiTestMethod::from(http::Method::DELETE),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSessionsRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(limited_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(limited_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("0f51dc7a");
    assert_eq!(revoke_all_response.status(), http::StatusCode::NO_CONTENT);
    let revoked_all_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminMeRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(limited_cookie.as_str())),
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
    let limited_id = sqlx::query_scalar::<_, i64>(
        str_constants::SELECT_ID_FROM_ADMIN_USERS_WHERE_LOGIN_LIMITED_USER,
    )
    .fetch_one(&pool.0)
    .await
    .expect("10c8f7d2");
    let update_user_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::PATCH),
            StdAdminApiTestStrRef::from(format!("/users/{limited_id}").as_str()),
            StdAdminApiTestStrRef::from(str_constants::DISPLAY_NAME_UPDATED_USER),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("623cde18");
    assert_eq!(update_user_response.status(), http::StatusCode::NO_CONTENT);
    let ban_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(format!("/users/{limited_id}/ban").as_str()),
            StdAdminApiTestStrRef::from(str_constants::IS_BANNED_TRUE),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("94a7e1cb");
    assert_eq!(ban_response.status(), http::StatusCode::NO_CONTENT);
    let banned_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminMeRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(limited_cookie.as_str())),
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
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(
                str_constants::LOGIN_LIMITED_USER_PASSWORD_LIMITED_PASSWORD,
            ),
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
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminListUsersRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
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
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminListRolesRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
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
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminListRolesRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::NAME_TEMPORARY_ROLE),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("6d9384fe");
    assert_eq!(create_role_response.status(), http::StatusCode::CREATED);
    let role_id = sqlx::query_scalar::<_, i64>(
        str_constants::SELECT_ID_FROM_ADMIN_ROLES_WHERE_NAME_TEMPORARY_ROLE,
    )
    .fetch_one(&pool.0)
    .await
    .expect("1e53a0c7");
    let assign_role_body =
        serde_json::to_string(&server_admin_contract::AdminSetUserRolesReq::new(
            empty_admin_role_ids(),
            one_admin_role_id(
                server_admin_contract::AdminRoleId::try_from(role_id).expect("a82fc2e5"),
            ),
        ))
        .expect("bf02e516");
    let assign_role_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::PUT),
            StdAdminApiTestStrRef::from(format!("/users/{limited_id}/roles").as_str()),
            StdAdminApiTestStrRef::from(assign_role_body.as_str()),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("f74095eb");
    assert_eq!(assign_role_response.status(), http::StatusCode::NO_CONTENT);
    let stale_role_body = serde_json::to_string(&server_admin_contract::AdminSetUserRolesReq::new(
        empty_admin_role_ids(),
        empty_admin_role_ids(),
    ))
    .expect("1fd845d3");
    let stale_role_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::PUT),
            StdAdminApiTestStrRef::from(format!("/users/{limited_id}/roles").as_str()),
            StdAdminApiTestStrRef::from(stale_role_body.as_str()),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("170158fb");
    assert_eq!(stale_role_response.status(), http::StatusCode::CONFLICT);
    let remove_role_body =
        serde_json::to_string(&server_admin_contract::AdminSetUserRolesReq::new(
            one_admin_role_id(
                server_admin_contract::AdminRoleId::try_from(role_id).expect("c8994c27"),
            ),
            empty_admin_role_ids(),
        ))
        .expect("23c416a1");
    let remove_role_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::PUT),
            StdAdminApiTestStrRef::from(format!("/users/{limited_id}/roles").as_str()),
            StdAdminApiTestStrRef::from(remove_role_body.as_str()),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("a895d91f");
    assert_eq!(remove_role_response.status(), http::StatusCode::NO_CONTENT);
    let update_role_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::PATCH),
            StdAdminApiTestStrRef::from(format!("/roles/{role_id}").as_str()),
            StdAdminApiTestStrRef::from(str_constants::NAME_RENAMED_ROLE),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("4f08b7ec");
    assert_eq!(update_role_response.status(), http::StatusCode::NO_CONTENT);
    let delete_role_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::DELETE),
            StdAdminApiTestStrRef::from(format!("/roles/{role_id}").as_str()),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("d7e1862c");
    assert_eq!(delete_role_response.status(), http::StatusCode::NO_CONTENT);
    let delete_user_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::DELETE),
            StdAdminApiTestStrRef::from(format!("/users/{limited_id}").as_str()),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("c19be784");
    assert_eq!(delete_user_response.status(), http::StatusCode::NO_CONTENT);
    let admin_role_id =
        sqlx::query_scalar::<_, i64>(str_constants::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL)
            .fetch_one(&pool.0)
            .await
            .expect("20b5fb03");
    let remove_last_admin_role_body =
        serde_json::to_string(&server_admin_contract::AdminSetUserRolesReq::new(
            one_admin_role_id(
                server_admin_contract::AdminRoleId::try_from(admin_role_id).expect("84fe96c8"),
            ),
            empty_admin_role_ids(),
        ))
        .expect("1528b0d3");
    let remove_last_admin_role_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::PUT),
            StdAdminApiTestStrRef::from(format!("/users/{admin_id}/roles").as_str()),
            StdAdminApiTestStrRef::from(remove_last_admin_role_body.as_str()),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("fe0db65c");
    assert_eq!(
        remove_last_admin_role_response.status(),
        http::StatusCode::CONFLICT
    );
    let last_admin_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::DELETE),
            StdAdminApiTestStrRef::from(format!("/users/{admin_id}").as_str()),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("e6175d82");
    assert_eq!(last_admin_response.status(), http::StatusCode::CONFLICT);
    let audit_response =
        tower::ServiceExt::oneshot(
            router_with_pool(&pool).0,
            request_with_peer(
                HttpAdminApiTestMethod::from(http::Method::GET),
                StdAdminApiTestStrRef::from(
                    format!(
                        "{}?limit=1&offset=1",
                        frontend_contract::typed_route_path::<
                            server_admin_contract::AdminAuditLogRoute,
                        >()
                    )
                    .as_str(),
                ),
                StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect("8103cd5f");
    assert_eq!(audit_response.status(), http::StatusCode::OK);
    let audit_page = axum::body::to_bytes(audit_response.into_body(), 1_048_576usize)
        .await
        .map(|body| {
            serde_json::from_slice::<server_admin_contract::AdminAuditPage>(&body)
                .expect("ed125d4a")
        })
        .expect("50612a4d");
    assert!(audit_page.items().len() <= 1usize);
    assert!(
        u64::from(audit_page.total()) >= u64::try_from(audit_page.items().len()).expect("03c133e9")
    );
    futures::StreamExt::fold(
        futures::stream::iter(0usize..61usize),
        (),
        async |(), _index| {
            let response = tower::ServiceExt::oneshot(
                router_with_pool(&pool).0,
                request_with_peer(
                    HttpAdminApiTestMethod::from(http::Method::GET),
                    StdAdminApiTestStrRef::from(
                        format!(
                            "{}?limit=1&offset=0",
                            frontend_contract::typed_route_path::<
                                server_admin_contract::AdminAuditLogRoute,
                            >()
                        )
                        .as_str(),
                    ),
                    StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
                    Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
                    None,
                )
                .0,
            )
            .await
            .expect("a6fa9aeb");
            assert_eq!(response.status(), http::StatusCode::OK);
        },
    )
    .await;

    let sessions_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from("/auth/sessions?limit=1&offset=0"),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("449bf918");
    assert_eq!(sessions_response.status(), http::StatusCode::OK);
    let sessions_page = axum::body::to_bytes(sessions_response.into_body(), 1_048_576usize)
        .await
        .map(|body| {
            serde_json::from_slice::<server_admin_contract::AdminSessionsPage>(&body)
                .expect("e544366c")
        })
        .expect("141ddcdc");
    assert!(sessions_page.items().len() <= 1usize);
    assert!(
        u64::from(sessions_page.total())
            >= u64::try_from(sessions_page.items().len()).expect("701a7a79")
    );

    let data_table_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from("/tables/users?limit=1&offset=0"),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("ca94aec1");
    assert_eq!(data_table_response.status(), http::StatusCode::OK);
    let data_table = axum::body::to_bytes(data_table_response.into_body(), 1_048_576usize)
        .await
        .map(|body| {
            serde_json::from_slice::<server_admin_contract::AdminDataTableView>(&body)
                .expect("e16283f4")
        })
        .expect("3f927581");
    assert!(data_table.items().len() <= 1usize);
    assert!(
        u64::from(data_table.total()) >= u64::try_from(data_table.items().len()).expect("1440730f")
    );
    let filtered_data_table_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                format!(
                    "/tables/users?filter_field=login&filter_operation=eq&filter_value={}&limit=20&offset=0",
                    str_constants::ADMIN_ALT
                )
                .as_str(),
            ),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("766f5654");
    assert_eq!(filtered_data_table_response.status(), http::StatusCode::OK);
    let filtered_data_table =
        axum::body::to_bytes(filtered_data_table_response.into_body(), 1_048_576usize)
            .await
            .map(|body| {
                serde_json::from_slice::<server_admin_contract::AdminDataTableView>(&body)
                    .expect("02d611ab")
            })
            .expect("6dfe8f37");
    assert_eq!(u64::from(filtered_data_table.total()), 1u64);
    assert_eq!(filtered_data_table.items().len(), 1usize);
    assert!(
        filtered_data_table
            .items()
            .first()
            .expect("753fa97c")
            .values()
            .iter()
            .any(|value| value.as_ref() == str_constants::ADMIN_ALT)
    );
    let empty_data_table_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from("/tables/users?filter_field=login&filter_operation=eq&filter_value=missing_filter_user&limit=20&offset=0"),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("1310e021");
    assert_eq!(empty_data_table_response.status(), http::StatusCode::OK);
    let empty_data_table =
        axum::body::to_bytes(empty_data_table_response.into_body(), 1_048_576usize)
            .await
            .map(|body| {
                serde_json::from_slice::<server_admin_contract::AdminDataTableView>(&body)
                    .expect("aa8376d3")
            })
            .expect("a98d6360");
    assert_eq!(u64::from(empty_data_table.total()), 0u64);
    assert!(empty_data_table.items().is_empty());
    let unsupported_filter_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from("/tables/users?filter_field=login&filter_operation=between&filter_value=admin&filter_end=root&limit=20&offset=0"),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("dd6d2544");
    assert_eq!(
        unsupported_filter_response.status(),
        http::StatusCode::UNPROCESSABLE_ENTITY
    );
    let incomplete_filter_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                "/tables/users?filter_field=login&filter_value=admin&limit=20&offset=0",
            ),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("e9279b1f");
    assert_eq!(
        incomplete_filter_response.status(),
        http::StatusCode::UNPROCESSABLE_ENTITY
    );
    let sign_out_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignOutRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("ef71e50a");
    assert_eq!(sign_out_response.status(), http::StatusCode::NO_CONTENT);
    let revoked_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminMeRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("54b9dc03");
    assert_eq!(revoked_response.status(), http::StatusCode::UNAUTHORIZED);
    let audit_outcomes = sqlx::query_as::<_, (bool, i64)>(str_constants::SELECT_SUCCEEDED_COUNT_ASTERISK_FROM_ADMIN_AUDIT_LOG_GROUP_BY_SUCCEEDED_ORDER)
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
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_data_table_api_reads_every_public_field_from_every_table() {
    let fixture = admin_html_test_fixture().await;
    let _cleanup_status = sqlx::query(
        "INSERT INTO cleanup_status (singleton, last_success_at, last_deleted_rows) VALUES (TRUE, NOW(), 0) ON CONFLICT (singleton) DO UPDATE SET last_success_at = EXCLUDED.last_success_at, last_deleted_rows = EXCLUDED.last_deleted_rows",
    )
    .execute(&fixture.pool.0)
    .await
    .expect("70dfa001");
    let _rate_limit = sqlx::query(
        "INSERT INTO rate_limits (scope, subject, request_count) VALUES ('api_field_test', 'api_field_test', 1) ON CONFLICT (scope, subject) DO UPDATE SET request_count = EXCLUDED.request_count",
    )
    .execute(&fixture.pool.0)
    .await
    .expect("f8f27048");
    let fixture_ref = &fixture;
    futures::StreamExt::fold(
        futures::stream::iter(server_admin_contract::AdminDataTable::PG_ORDER),
        (),
        async |(), table| {
            let uri = format!("/tables/{table}?limit=100&offset=0");
            let response = tower::ServiceExt::oneshot(
                router_with_pool(&fixture_ref.pool).0,
                request_with_peer(
                    HttpAdminApiTestMethod::from(http::Method::GET),
                    StdAdminApiTestStrRef::from(uri.as_str()),
                    StdAdminApiTestStrRef::from(str_constants::PG_CRUD_EMPTY_SQL_SUFFIX),
                    Some(StdAdminApiTestStrRef::from(fixture_ref.cookie.0.as_str())),
                    None,
                )
                .0,
            )
            .await
            .expect("4b58a9ba");
            assert_eq!(
                response.status(),
                http::StatusCode::OK,
                "table API {table} failed"
            );
            let body = axum::body::to_bytes(response.into_body(), 1_048_576usize)
                .await
                .expect("78547eed");
            let view =
                serde_json::from_slice::<server_admin_contract::AdminDataTableView>(body.as_ref())
                    .expect("6d2a32e6");
            assert_eq!(view.table(), table);
            let expected_columns = table.spec().columns().get().split(',').collect::<Vec<_>>();
            assert_eq!(view.columns().len(), expected_columns.len());
            expected_columns
                .iter()
                .enumerate()
                .for_each(|(field_index, expected_name)| {
                    assert_eq!(
                        view.columns()
                            .get(field_index)
                            .map(|column| column.name().as_ref().as_str()),
                        Some(*expected_name),
                        "{table}.{expected_name} is missing or out of order"
                    );
                    assert!(
                        view.items().iter().all(|row| row
                            .values()
                            .get(field_index)
                            .is_some_and(|value| !value.as_ref().is_empty())),
                        "{table}.{expected_name} has no readable value"
                    );
                });
            assert!(
                !view.items().is_empty(),
                "table API {table} returned no rows"
            );
        },
    )
    .await;
    fixture.lock.0.rollback().await.expect("83226fd7");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_generated_mutation_idempotency_contract() {
    let database_url = std::env::var(str_constants::ENV_NAMES_DATABASE_URL).expect("40c1e398");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4u32)
        .connect(database_url.as_str())
        .await
        .expect("cb6830bc");
    let mut idempotency_test_isolation = pool.begin().await.expect("ea1d891d");
    pg_crud_common::lock_pg_relation_resources(
        pg_crud_common::SqlxPgRelationLockConnectionRef::from(&mut *idempotency_test_isolation),
        &pg_crud_common::PgRelationLockNamespace::try_from(str_constants::ACTOR_ATOMIC.to_owned())
            .expect("136c5acc"),
        &pg_crud_common::PgRelationResourceIds::try_from(vec![
            pg_crud_common::PgRelationResourceId::from(1i64),
        ])
        .expect("8b0c7ae1"),
    )
    .await
    .expect("508db033");
    pg_table::ensure_pg_table_idempotency_schema(app_state::SqlxPgPoolRef::from(&pool))
        .await
        .expect("6c338824");
    let _truncate_result = sqlx::query(str_constants::TRUNCATE_PG_TABLE_IDEMPOTENCY)
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
                pg_table::PgTableIdempotencyMethod::try_from(str_constants::POST.to_owned())
                    .expect("94bc0508"),
                pg_table::PgTableIdempotencyRoute::try_from(route.0.to_owned()).expect("4e8c040f"),
                pg_table::PgTableIdempotencyKey::try_from(key.0.to_owned()).expect("2028024d"),
            ),
            body,
        )
    };
    let first_request = make_request(
        StdAdminApiTestStrRef::from(str_constants::ACTOR_A),
        StdAdminApiTestStrRef::from(str_constants::ITEMS_CM),
        StdAdminApiTestStrRef::from(str_constants::KEY_A),
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
        StdAdminApiTestStrRef::from(str_constants::ACTOR_A),
        StdAdminApiTestStrRef::from(str_constants::ITEMS_CM),
        StdAdminApiTestStrRef::from(str_constants::KEY_A),
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
        pg_table::PgTableIdempotencyResponseStatus::try_from(201u16).expect("4df2dd1f"),
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
            pg_table::PgTableIdempotencyResponseStatus::try_from(201u16).expect("f89d923d"),
            pg_table::PgTableIdempotencyBody::try_from(response_body.to_vec()).expect("4a01ed0e"),
        )
    );
    let other_actor = make_request(
        StdAdminApiTestStrRef::from(str_constants::ACTOR_B),
        StdAdminApiTestStrRef::from(str_constants::ITEMS_CM),
        StdAdminApiTestStrRef::from(str_constants::KEY_A),
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
        StdAdminApiTestStrRef::from(str_constants::ACTOR_CONCURRENT),
        StdAdminApiTestStrRef::from(str_constants::ITEMS_CM),
        StdAdminApiTestStrRef::from(str_constants::KEY_CONCURRENT),
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
    let _atomic_table = sqlx::query(
        str_constants::CREATE_TABLE_IF_NOT_EXISTS_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST_ID_BIGINT,
    )
    .execute(&pool)
    .await
    .expect("af066e8b");
    let _atomic_clear = sqlx::query(str_constants::TRUNCATE_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST)
        .execute(&pool)
        .await
        .expect("3130e593");
    let atomic = make_request(
        StdAdminApiTestStrRef::from(str_constants::ACTOR_ATOMIC),
        StdAdminApiTestStrRef::from(str_constants::ITEMS_CO),
        StdAdminApiTestStrRef::from(str_constants::KEY_ATOMIC),
        pg_table::PgTableIdempotencyBodyRef::from(br#"{"id":1}"#.as_slice()),
    );
    assert_eq!(
        pg_table::begin_pg_table_idempotency(app_state::SqlxPgPoolRef::from(&pool), &atomic)
            .await
            .expect("925ea283"),
        pg_table::PgTableIdempotencyBegin::Acquired
    );
    let mut rollback_tx = pool.begin().await.expect("fcba80e1");
    let _mutation =
        sqlx::query(str_constants::INSERT_INTO_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST_ID_VALUES_1)
            .execute(&mut *rollback_tx)
            .await
            .expect("67503e70");
    pg_table::complete_pg_table_idempotency_in_connection(
        pg_table::SqlxPgTablePgConnectionRef::from(&mut *rollback_tx),
        &atomic,
        pg_table::PgTableIdempotencyResponseStatus::try_from(201u16).expect("98bb1db9"),
        pg_table::PgTableIdempotencyBodyRef::from(br#"{"id":1}"#.as_slice()),
    )
    .await
    .expect("8ad86515");
    rollback_tx.rollback().await.expect("11cfcb27");
    let mutation_count = sqlx::query_scalar::<_, i64>(
        str_constants::SELECT_COUNT_ASTERISK_FROM_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST,
    )
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
    let _age_records = sqlx::query(
        str_constants::UPDATE_PG_TABLE_IDEMPOTENCY_SET_CREATED_AT_TIMESTAMPTZ_2000_01_01_00,
    )
    .execute(&pool)
    .await
    .expect("a46f7336");
    let before_cleanup = sqlx::query_scalar::<_, i64>(
        str_constants::SELECT_COUNT_ASTERISK_FROM_PG_TABLE_IDEMPOTENCY,
    )
    .fetch_one(&pool)
    .await
    .expect("2c080f6d");
    let cleaned = pg_table::cleanup_pg_table_idempotency(
        app_state::SqlxPgPoolRef::from(&pool),
        pg_table::PgTableIdempotencyCleanupRetentionSeconds::try_from(3_600i64).expect("52189299"),
        pg_table::PgTableIdempotencyCleanupRetentionSeconds::try_from(3_600i64).expect("fa6dc1d7"),
        pg_table::PgTableIdempotencyCleanupBatchSize::try_from(2i64).expect("1780d6b1"),
    )
    .await
    .expect("b1ba49cc");
    assert_eq!(u64::from(cleaned), 2u64);
    let after_cleanup = sqlx::query_scalar::<_, i64>(
        str_constants::SELECT_COUNT_ASTERISK_FROM_PG_TABLE_IDEMPOTENCY,
    )
    .fetch_one(&pool)
    .await
    .expect("6863201e");
    assert_eq!(
        before_cleanup.checked_sub(after_cleanup).expect("f93ed3cf"),
        2i64
    );
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_optimistic_revision_allows_one_concurrent_writer() {
    let database_url = std::env::var(str_constants::ENV_NAMES_DATABASE_URL).expect("63a09eec");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4u32)
        .connect(database_url.as_str())
        .await
        .expect("2480f8c4");
    let _drop_before =
        sqlx::query(str_constants::DROP_TABLE_IF_EXISTS_PG_TABLE_OPTIMISTIC_REVISION_TEST)
            .execute(&pool)
            .await
            .expect("e5e1f7cb");
    let _create = sqlx::query(str_constants::CREATE_TABLE_PG_TABLE_OPTIMISTIC_REVISION_TEST_ID_BIGINT_PRIMARY_KEY_REVISION)
        .execute(&pool)
        .await
        .expect("a75bc224");
    let _insert = sqlx::query(
        str_constants::INSERT_INTO_PG_TABLE_OPTIMISTIC_REVISION_TEST_ID_REVISION_VALUE_VALUES_1,
    )
    .execute(&pool)
    .await
    .expect("da271038");
    let update = str_constants::UPDATE_PG_TABLE_OPTIMISTIC_REVISION_TEST_SET_VALUE_DOLLAR_1_REVISION_REVISION;
    let (left, right) = tokio::join!(
        sqlx::query_scalar::<_, i64>(update)
            .bind(1i64)
            .bind(
                pg_table::PgTableRevision::try_from(str_constants::VALUE_0.to_owned())
                    .expect("979fa4b2")
            )
            .fetch_optional(&pool),
        sqlx::query_scalar::<_, i64>(update)
            .bind(2i64)
            .bind(
                pg_table::PgTableRevision::try_from(str_constants::VALUE_0.to_owned())
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
            pg_table::PgTableRevision::try_from(str_constants::VALUE_0.to_owned())
                .expect("a3a08aeb"),
        )
        .fetch_optional(&pool)
        .await
        .expect("964e3ef4");
    assert_eq!(stale, None);
    let _drop_after = sqlx::query(str_constants::DROP_TABLE_PG_TABLE_OPTIMISTIC_REVISION_TEST)
        .execute(&pool)
        .await
        .expect("a4d77f54");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_cleanup_is_batched_and_preserves_append_only_policy() {
    let database_url = std::env::var(str_constants::ENV_NAMES_DATABASE_URL).expect("7316cf4d");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3u32)
        .connect(database_url.as_str())
        .await
        .expect("f6a51733");
    let mut admin_db_test_lock = pool.begin().await.expect("847caf57");
    let _locked = sqlx::query(str_constants::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
        .execute(&mut *admin_db_test_lock)
        .await
        .expect("8c298fef");
    let mut idempotency_test_isolation = pool.begin().await.expect("f56c4c85");
    pg_crud_common::lock_pg_relation_resources(
        pg_crud_common::SqlxPgRelationLockConnectionRef::from(&mut *idempotency_test_isolation),
        &pg_crud_common::PgRelationLockNamespace::try_from(str_constants::ACTOR_ATOMIC.to_owned())
            .expect("861fe23d"),
        &pg_crud_common::PgRelationResourceIds::try_from(vec![
            pg_crud_common::PgRelationResourceId::from(1i64),
        ])
        .expect("a18f804c"),
    )
    .await
    .expect("fab61374");
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool))
        .await
        .expect("029cb682");
    pg_table::ensure_pg_table_idempotency_schema(app_state::SqlxPgPoolRef::from(&pool))
        .await
        .expect("eb08dffc");
    let _clear = sqlx::query(str_constants::TRUNCATE_ADMIN_ACCESS_SESSIONS_ADMIN_REFRESH_TOKENS_ADMIN_LOGIN_ATTEMPTS_ADMIN_RATE)
        .execute(&pool)
        .await
        .expect("e1b22572");
    let _attempts = sqlx::query(str_constants::INSERT_INTO_ADMIN_LOGIN_ATTEMPTS_LOGIN_SUCCEEDED_ATTEMPTED_AT_SELECT_OLD_VALUE)
        .execute(&pool)
        .await
        .expect("480b06eb");
    let _limits = sqlx::query(str_constants::INSERT_INTO_ADMIN_RATE_LIMITS_SCOPE_SUBJECT_WINDOW_STARTED_AT_REQUEST_COUNT_ALT)
        .execute(&pool)
        .await
        .expect("0375574d");
    let _audit = sqlx::query(
        str_constants::INSERT_INTO_ADMIN_AUDIT_LOG_ACTION_RESOURCE_SUCCEEDED_CREATED_AT_SELECT_TEST,
    )
    .execute(&pool)
    .await
    .expect("f50ef817");
    let retention =
        server_admin::AdminCleanupRetentionSeconds::try_from(3_600i64).expect("ab892fc5");
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
    let remaining = sqlx::query_as::<_, (i64, i64, i64)>(str_constants::SELECT_SELECT_COUNT_ASTERISK_FROM_ADMIN_LOGIN_ATTEMPTS_SELECT_COUNT_ASTERISK_FROM)
        .fetch_one(&pool)
        .await
        .expect("f37a3ab4");
    assert_eq!(remaining, (1i64, 1i64, 1i64));
    let ordinary_delete = sqlx::query(str_constants::DELETE_FROM_ADMIN_AUDIT_LOG)
        .execute(&pool)
        .await;
    assert!(matches!(ordinary_delete, Err(_error)));
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_migration_creates_complete_schema() {
    let database_url = std::env::var(str_constants::ENV_NAMES_DATABASE_URL).expect("b65d1786");
    let base_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1u32)
        .connect(database_url.as_str())
        .await
        .expect("0047f74e");
    let _drop_schema =
        sqlx::raw_sql(str_constants::DROP_SCHEMA_IF_EXISTS_ADMIN_MIGRATION_FRESH_TEST_CASCADE)
            .execute(&base_pool)
            .await
            .expect("df91b04d");
    let _create_schema = sqlx::raw_sql(str_constants::CREATE_SCHEMA_ADMIN_MIGRATION_FRESH_TEST)
        .execute(&base_pool)
        .await
        .expect("02bcd1c2");
    let connect = |schema: StdAdminApiTestStrRef<'static>| {
        let options = <sqlx::postgres::PgConnectOptions as std::str::FromStr>::from_str(
            database_url.as_str(),
        )
        .expect("aa7735db")
        .options([(str_constants::SEARCH_PATH, schema.0)]);
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(1u32)
            .connect_lazy_with(options)
    };
    let fresh_pool = connect(StdAdminApiTestStrRef::from(
        str_constants::ADMIN_MIGRATION_FRESH_TEST,
    ));
    let full = sqlx::migrate!("./migrations");
    full.run(&fresh_pool).await.expect("4b6c3bd6");
    server_admin::generated_tables::validate_catalog_schema(
        pg_crud_common::SqlxPgPoolRef::from(&fresh_pool),
        pg_crud_common::DbSchemaNameRef::from(str_constants::ADMIN_MIGRATION_FRESH_TEST),
    )
    .await
    .expect("fac299aa");
    let version = sqlx::query_scalar::<_, i64>(
        str_constants::SELECT_MAX_VERSION_FROM_ADMIN_MIGRATION_FRESH_TEST_SQLX_MIGRATIONS_WHERE,
    )
    .fetch_one(&base_pool)
    .await
    .expect("5c10c931");
    assert_eq!(version, 12i64);
    let expected_tables = server_admin_contract::AdminDataTable::PG_ORDER
        .map(|table| table.to_string())
        .to_vec();
    let fresh_tables = sqlx::query_scalar::<_, String>(
        str_constants::SELECT_TABLE_NAME_FROM_INFORMATION_SCHEMA_TABLES_WHERE_TABLE_SCHEMA,
    )
    .bind(str_constants::ADMIN_MIGRATION_FRESH_TEST)
    .fetch_all(&base_pool)
    .await
    .expect("ab254ff4");
    assert_eq!(fresh_tables, expected_tables);
    fresh_pool.close().await;
    let _drop_after = sqlx::raw_sql(str_constants::DROP_SCHEMA_ADMIN_MIGRATION_FRESH_TEST_CASCADE)
        .execute(&base_pool)
        .await
        .expect("88dd90b8");
}
