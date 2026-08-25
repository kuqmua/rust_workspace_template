#![allow(unused_crate_dependencies)]
// integration target inherits the library dependency graph while exercising the assembled public router
#![allow(clippy::tests_outside_test_module)] // every item in this integration target is compiled exclusively by the test harness
#[path = "admin_api/data_tables.rs"]
mod data_tables;
#[path = "admin_api/flow.rs"]
mod flow;
#[path = "admin_api/html.rs"]
mod html;
#[path = "admin_api/maintenance.rs"]
mod maintenance;
#[path = "admin_api/policy.rs"]
mod policy;
#[path = "admin_api/routing.rs"]
mod routing;
#[path = "admin_api/schema.rs"]
mod schema;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct StdAdminApiTestStrRef<'value_lt>(&'value_lt str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct AxumAdminApiTestRouter(axum::Router);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct SqlxAdminApiTestPool(sqlx::PgPool);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct SqlxAdminHtmlTestTransaction(sqlx::Transaction<'static, sqlx::Postgres>);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct HttpAdminApiTestMethod(http::Method);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct HttpAdminApiTestRequest(http::Request<axum::body::Body>);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::DerefInner, newtype::FromInner)]
struct HttpAdminHtmlTestResponse(http::Response<axum::body::Body>);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct HttpAdminApiTestResponseRef<'value_lt>(&'value_lt http::Response<axum::body::Body>);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::BoundedString)]
#[bounded_string(max = 16384)]
#[derive(newtype::Display)]
struct StdAdminApiTestCookie(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::BoundedString)]
#[bounded_string(max = 1_048_576)]
struct AdminHtmlTestBody(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::BoundedString)]
#[bounded_string(max = 65_536)]
struct AdminHtmlTestFormBody(String);
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
struct AdminHtmlTestFixture {
    cookie: StdAdminApiTestCookie,
    csrf: StdAdminApiTestCookie,
    lock: SqlxAdminHtmlTestTransaction,
    pool: SqlxAdminApiTestPool,
    router: AxumAdminApiTestRouter,
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
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
        .expect("c2af6158 form_body invariant must hold")
    }
}

fn one_admin_role_id(
    value: server_admin_contract::domain_types::AdminRoleId,
) -> server_admin_contract::domain_types::AdminRoleIds {
    server_admin_contract::domain_types::AdminRoleIds::try_from(vec![value])
        .expect("69bc51bc one_admin_role_id invariant must hold")
}
fn empty_admin_role_ids() -> server_admin_contract::domain_types::AdminRoleIds {
    server_admin_contract::domain_types::AdminRoleIds::try_from(Vec::new())
        .expect("d5ccd621 empty_admin_role_ids invariant must hold")
}
fn env<T>(value: StdAdminApiTestStrRef<'_>) -> T
where
    T: config_lib::domain_types::TryFromStdEnvVarOk,
    T::Error: std::fmt::Debug,
{
    T::try_from_std_env_var_ok(
        config_lib::domain_types::StdEnvVarOk::try_from(value.0.to_owned())
            .expect("92b71c4e env invariant must hold"),
    )
    .expect("afe20c19 env invariant must hold")
}
fn router() -> AxumAdminApiTestRouter {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy(constants_str::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION)
        .expect("27db915c router invariant must hold");
    let state = server_admin::domain_types::auth::AdminAuthSvcState::try_new(
        app_state::domain_types::SqlxPgPool::from(pool),
        &env::<config_lib::domain_types::AdminJwtSecret>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES,
        )),
        &env::<config_lib::domain_types::AdminAccessTokenTtlSeconds>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_900,
        )),
        &env::<config_lib::domain_types::AdminRefreshTokenTtlSeconds>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_3600,
        )),
        &env::<config_lib::domain_types::AdminSessionLimit>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_20,
        )),
        &env::<config_lib::domain_types::AdminSignInRateLimit>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_2,
        )),
        &env::<config_lib::domain_types::AdminLoginFailureLimit>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_10,
        )),
        &env::<config_lib::domain_types::AdminPasswordHashConcurrency>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_1),
        ),
        &env::<config_lib::domain_types::AdminCookieSecure>(StdAdminApiTestStrRef::from(
            constants_str::FALSE,
        )),
        &env::<config_lib::domain_types::AdminTokenIssuer>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST,
        )),
        &env::<config_lib::domain_types::AdminTokenAudience>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST_ADMIN,
        )),
        &config_lib::domain_types::CorsAllowOrigin(constants_str::HTTP_LOCALHOST.to_owned()),
    )
    .expect("f7d8c961 router invariant must hold");
    AxumAdminApiTestRouter::from(axum::Router::from(
        server_admin::domain_types::auth::routes(
            server_admin::domain_types::auth::SharedAdminAuthSvcStateArc::from(
                std::sync::Arc::new(state),
            ),
        ),
    ))
}
fn router_with_pool(pool: &SqlxAdminApiTestPool) -> AxumAdminApiTestRouter {
    let state = server_admin::domain_types::auth::AdminAuthSvcState::try_new(
        app_state::domain_types::SqlxPgPool::from(pool.0.clone()),
        &env::<config_lib::domain_types::AdminJwtSecret>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES,
        )),
        &env::<config_lib::domain_types::AdminAccessTokenTtlSeconds>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_900,
        )),
        &env::<config_lib::domain_types::AdminRefreshTokenTtlSeconds>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_3600,
        )),
        &env::<config_lib::domain_types::AdminSessionLimit>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_20,
        )),
        &env::<config_lib::domain_types::AdminSignInRateLimit>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_2,
        )),
        &env::<config_lib::domain_types::AdminLoginFailureLimit>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_10,
        )),
        &env::<config_lib::domain_types::AdminPasswordHashConcurrency>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_1),
        ),
        &env::<config_lib::domain_types::AdminCookieSecure>(StdAdminApiTestStrRef::from(
            constants_str::FALSE,
        )),
        &env::<config_lib::domain_types::AdminTokenIssuer>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST,
        )),
        &env::<config_lib::domain_types::AdminTokenAudience>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST_ADMIN,
        )),
        &config_lib::domain_types::CorsAllowOrigin(constants_str::HTTP_LOCALHOST.to_owned()),
    )
    .expect("a59d73c1 router_with_pool invariant must hold");
    AxumAdminApiTestRouter::from(axum::Router::from(
        server_admin::domain_types::auth::routes(
            server_admin::domain_types::auth::SharedAdminAuthSvcStateArc::from(
                std::sync::Arc::new(state),
            ),
        ),
    ))
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
        StdAdminApiTestStrRef::from(constants_str::VALUE_127_0_0_1_43210),
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
        .header(http::header::CONTENT_TYPE, constants_str::APPLICATION_JSON)
        .header(http::header::ORIGIN, constants_str::HTTP_LOCALHOST);
    if let Some(value) = cookie {
        builder = builder.header(http::header::COOKIE, value.0);
    }
    if let Some(value) = csrf {
        builder = builder.header(constants_str::X_CSRF_TOKEN_ALT, value.0);
    }
    let mut request = builder
        .body(axum::body::Body::from(body.0.to_owned()))
        .expect("7d924f8a request_with_peer_at invariant must hold");
    let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
        peer.0
            .parse::<std::net::SocketAddr>()
            .expect("d80fc31b request_with_peer_at invariant must hold"),
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
            constants_str::APPLICATION_X_WWW_FORM_URLENCODED,
        )
        .header(http::header::ORIGIN, constants_str::HTTP_LOCALHOST);
    if let Some(value) = cookie {
        builder = builder.header(http::header::COOKIE, value.0);
    }
    let mut request = builder
        .body(axum::body::Body::from(body.0.to_owned()))
        .expect("9f211b84 html_request_with_peer invariant must hold");
    let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
        constants_str::VALUE_127_0_0_1_43210
            .parse::<std::net::SocketAddr>()
            .expect("bcd41a67 html_request_with_peer invariant must hold"),
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
        .map(|value| {
            StdAdminApiTestCookie::try_from(value)
                .expect("b9a203e6 cookie_value invariant must hold")
        })
        .expect("360de719 cookie_value invariant must hold")
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
    .expect("3cb98672 admin_html_response invariant must hold")
}
async fn admin_html_body(response: HttpAdminHtmlTestResponse) -> AdminHtmlTestBody {
    axum::body::to_bytes(response.0.into_body(), constants_usize::VALUE_1_048_576)
        .await
        .map(|bytes| {
            String::from_utf8(bytes.to_vec()).expect("86547438 admin_html_body invariant must hold")
        })
        .map(|body| {
            AdminHtmlTestBody::try_from(body).expect("ec7261cd admin_html_body invariant must hold")
        })
        .expect("8b54de37 admin_html_body invariant must hold")
}
fn assert_admin_csr_shell(body: &AdminHtmlTestBody) {
    assert!(
        body.0.contains("id=\"admin-csr-root\""),
        "CSR root is missing"
    );
    assert!(
        body.0
            .contains("src=\"/admin/assets/csr_bootstrap.js?v=20260801-37\""),
        "CSR bootstrap script is missing"
    );
    assert!(!body.0.contains("<table"), "server rendered a data table");
    assert!(!body.0.contains("<form"), "server rendered a data form");
}
#[expect(
    clippy::missing_assert_message,
    reason = "the asserted status identifies the failed fixture stage"
)]
async fn admin_html_test_fixture_with_password_change(
    password_change_required: server_admin_contract::domain_types::AdminBool,
) -> AdminHtmlTestFixture {
    let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL)
        .expect("fbe54d19 admin_html_test_fixture_with_password_change invariant must hold");
    let pool = SqlxAdminApiTestPool::from(
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5u32)
            .connect(database_url.as_str())
            .await
            .expect("ac089d31 admin_html_test_fixture_with_password_change invariant must hold"),
    );
    let mut lock = pool
        .0
        .begin()
        .await
        .expect("37480e56 admin_html_test_fixture_with_password_change invariant must hold");
    let _locked = sqlx::query(constants_str::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
        .execute(&mut *lock)
        .await
        .expect("a6b7c8d9 admin_html_test_fixture_with_password_change invariant must hold");
    server_admin::domain_types::prep_pg(app_state::domain_types::SqlxPgPoolRef::from(&pool.0))
        .await
        .expect("45de3a61 admin_html_test_fixture_with_password_change invariant must hold");
    let _truncated = sqlx::query(
        constants_str::TRUNCATE_ADMIN_RATE_LIMITS_ADMIN_AUDIT_LOG_ADMIN_LOGIN_ATTEMPTS_ADMIN_ACCESS,
    )
    .execute(&pool.0)
    .await
    .expect("cf37a9e2 admin_html_test_fixture_with_password_change invariant must hold");
    let _deleted_non_system_roles = sqlx::query(constants_str::VALUE_4BCE193A)
        .execute(&pool.0)
        .await
        .expect("b267a647 admin_html_test_fixture_with_password_change invariant must hold");
    let password = serde_json::from_str::<server_admin_contract::domain_types::AdminNewPassword>(
        constants_str::CORRECT_PASSWORD,
    )
    .expect("d20a35e4 admin_html_test_fixture_with_password_change invariant must hold");
    let hasher = server_admin::domain_types::AdminPasswordHasher::new(
        server_admin::domain_types::AdminPasswordHashConcurrency::from(
            server_admin::domain_types::AdminNonZeroUsize::from(
                std::num::NonZeroUsize::new(constants_usize::ONE).expect(
                    "560498ab admin_html_test_fixture_with_password_change invariant must hold",
                ),
            ),
        ),
    );
    let _created_admin_id = server_admin::domain_types::bootstrap_admin(
        app_state::domain_types::SqlxPgPoolRef::from(&pool.0),
        server_admin::domain_types::AdminLogin::try_from(constants_str::ADMIN_ALT.to_owned())
            .expect("6a417bde admin_html_test_fixture_with_password_change invariant must hold"),
        server_admin::domain_types::AdminDisplayName::try_from(constants_str::ADMIN.to_owned())
            .expect("703fc568 admin_html_test_fixture_with_password_change invariant must hold"),
        password,
        &hasher,
    )
    .await
    .expect("1e29c87f admin_html_test_fixture_with_password_change invariant must hold");
    if !bool::from(password_change_required) {
        let _updated =
            sqlx::query(constants_str::UPDATE_ADMIN_USERS_SET_MUST_CHANGE_PASSWORD_FALSE)
                .execute(&pool.0)
                .await
                .expect(
                    "a37042f1 admin_html_test_fixture_with_password_change invariant must hold",
                );
    }
    let state = server_admin::domain_types::auth::AdminAuthSvcState::try_new(
        app_state::domain_types::SqlxPgPool::from(pool.0.clone()),
        &env::<config_lib::domain_types::AdminJwtSecret>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES,
        )),
        &env::<config_lib::domain_types::AdminAccessTokenTtlSeconds>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_900,
        )),
        &env::<config_lib::domain_types::AdminRefreshTokenTtlSeconds>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_3600,
        )),
        &env::<config_lib::domain_types::AdminSessionLimit>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_20,
        )),
        &env::<config_lib::domain_types::AdminSignInRateLimit>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_20,
        )),
        &env::<config_lib::domain_types::AdminLoginFailureLimit>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_10,
        )),
        &env::<config_lib::domain_types::AdminPasswordHashConcurrency>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_1),
        ),
        &env::<config_lib::domain_types::AdminCookieSecure>(StdAdminApiTestStrRef::from(
            constants_str::FALSE,
        )),
        &env::<config_lib::domain_types::AdminTokenIssuer>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST,
        )),
        &env::<config_lib::domain_types::AdminTokenAudience>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST_ADMIN,
        )),
        &config_lib::domain_types::CorsAllowOrigin(constants_str::HTTP_LOCALHOST.to_owned()),
    )
    .expect("ec39b61d admin_html_test_fixture_with_password_change invariant must hold");
    let router = AxumAdminApiTestRouter::from(axum::Router::from(
        server_admin::domain_types::auth::html_routes_with_swagger(
            server_admin::domain_types::auth::SharedAdminAuthSvcStateArc::from(
                std::sync::Arc::new(state),
            ),
            server_admin::domain_types::auth::AdminHtmlSwaggerEnabled::from(true),
        ),
    ));
    let correct_password = serde_json::from_str::<String>(constants_str::CORRECT_PASSWORD)
        .expect("825e50c7 admin_html_test_fixture_with_password_change invariant must hold");
    let sign_in_body = AdminHtmlTestFormBody::try_from(format!(
        "login={}&password={correct_password}",
        constants_str::ADMIN_ALT,
    ))
    .expect("9df2164c admin_html_test_fixture_with_password_change invariant must hold");
    let sign_in_response = tower::ServiceExt::oneshot(
        router.0.clone(),
        html_request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                server_admin_contract::domain_types::AdminHtmlAction::SignIn.get(),
            ),
            StdAdminApiTestStrRef::from(sign_in_body.0.as_str()),
            None,
        )
        .0,
    )
    .await
    .expect("68a2cb40 admin_html_test_fixture_with_password_change invariant must hold");
    assert_eq!(sign_in_response.status(), http::StatusCode::SEE_OTHER);
    let access = cookie_value(
        HttpAdminApiTestResponseRef::from(&sign_in_response),
        StdAdminApiTestStrRef::from(constants_str::ADMIN_ACCESS_TOKEN),
    );
    let refresh = cookie_value(
        HttpAdminApiTestResponseRef::from(&sign_in_response),
        StdAdminApiTestStrRef::from(constants_str::ADMIN_REFRESH_TOKEN_ALT),
    );
    let csrf = cookie_value(
        HttpAdminApiTestResponseRef::from(&sign_in_response),
        StdAdminApiTestStrRef::from(constants_str::ADMIN_CSRF_TOKEN_ALT),
    );
    AdminHtmlTestFixture {
        cookie: StdAdminApiTestCookie::try_from(format!(
            "{}{access}; {}{refresh}; {}{csrf}",
            constants_str::ADMIN_ACCESS_TOKEN,
            constants_str::ADMIN_REFRESH_TOKEN_ALT,
            constants_str::ADMIN_CSRF_TOKEN_ALT,
        ))
        .expect("a4df94d1 admin_html_test_fixture_with_password_change invariant must hold"),
        csrf,
        lock: SqlxAdminHtmlTestTransaction::from(lock),
        pool,
        router,
    }
}
async fn admin_html_test_fixture() -> AdminHtmlTestFixture {
    admin_html_test_fixture_with_password_change(
        server_admin_contract::domain_types::AdminBool::from(false),
    )
    .await
}
async fn postgres_accepts_admin_user_policy_values(
    pool: &SqlxAdminApiTestPool,
    display_name: StdAdminApiTestStrRef<'_>,
    login: StdAdminApiTestStrRef<'_>,
) -> server_admin_contract::domain_types::AdminBool {
    let mut transaction = pool
        .0
        .begin()
        .await
        .expect("e6f2cdf7 postgres_accepts_admin_user_policy_values invariant must hold");
    let accepted = sqlx::query(constants_str::INSERT_ADMIN_USER_POLICY_PROBE)
        .bind(login.0)
        .bind(display_name.0)
        .bind(constants_str::X)
        .execute(&mut *transaction)
        .await
        .is_ok();
    transaction
        .rollback()
        .await
        .expect("fc4eec8f postgres_accepts_admin_user_policy_values invariant must hold");
    server_admin_contract::domain_types::AdminBool::from(accepted)
}
async fn postgres_accepts_admin_role_policy_value(
    pool: &SqlxAdminApiTestPool,
    name: StdAdminApiTestStrRef<'_>,
) -> server_admin_contract::domain_types::AdminBool {
    let mut transaction = pool
        .0
        .begin()
        .await
        .expect("77c2db82 postgres_accepts_admin_role_policy_value invariant must hold");
    let accepted = sqlx::query(constants_str::INSERT_ADMIN_ROLE_POLICY_PROBE)
        .bind(name.0)
        .execute(&mut *transaction)
        .await
        .is_ok();
    transaction
        .rollback()
        .await
        .expect("aa9b0106 postgres_accepts_admin_role_policy_value invariant must hold");
    server_admin_contract::domain_types::AdminBool::from(accepted)
}
