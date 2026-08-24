#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_auth_rbac_csrf_session_and_audit_flow() {
    let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL)
        .expect("ac0cb9e3 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    let pool = SqlxAdminApiTestPool::from(
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url.as_str())
            .await
            .expect(
                "a3e1f57c postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
            ),
    );
    let mut admin_db_test_lock =
        pool.0.begin().await.expect(
            "4dfb6865 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
        );
    let _locked = sqlx::query(constants_str::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
        .execute(&mut *admin_db_test_lock)
        .await
        .expect("693b147f postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool.0))
        .await
        .expect("0ea8d516 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool.0))
        .await
        .expect("676c00f1 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    server_admin::generated_tables::validate_catalog_schema(
        pg_crud_common::SqlxPgPoolRef::from(&pool.0),
        pg_crud_common::DbSchemaNameRef::from(constants_str::PUBLIC),
    )
    .await
    .expect("65ce07e9 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    let observed_permissions = sqlx::query_scalar::<_, String>(
        constants_str::SELECT_NAME_FROM_ADMIN_PERMISSIONS_ORDER_BY_NAME,
    )
    .fetch_all(&pool.0)
    .await
    .expect("db765f20 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    let expected_permissions = server_admin::AdminPermission::ALL
        .into_iter()
        .map(|permission| permission.as_str().as_ref().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(observed_permissions, expected_permissions);
    let _deleted_permission = sqlx::query(constants_str::DELETE_ADMIN_PERMISSION_BY_NAME)
        .bind(
            server_admin::AdminPermission::ALL
                .first()
                .expect(
                    "26d95ea4 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
                )
                .as_str()
                .as_ref(),
        )
        .execute(&pool.0)
        .await
        .expect("9d762f8c postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool.0))
        .await
        .expect("ea3f641d postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    let reconciled_permissions = sqlx::query_scalar::<_, String>(
        constants_str::SELECT_NAME_FROM_ADMIN_PERMISSIONS_ORDER_BY_NAME,
    )
    .fetch_all(&pool.0)
    .await
    .expect("458ab19e postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(reconciled_permissions, expected_permissions);
    let _truncate_result = sqlx::query(
        constants_str::TRUNCATE_ADMIN_RATE_LIMITS_ADMIN_AUDIT_LOG_ADMIN_LOGIN_ATTEMPTS_ADMIN_ACCESS,
    )
    .execute(&pool.0)
    .await
    .expect("97b5ad2f postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    let password = serde_json::from_str::<server_admin_contract::AdminNewPassword>(
        constants_str::CORRECT_PASSWORD,
    )
    .expect("703a8df2 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    let hasher =
        server_admin::AdminPasswordHasher::new(server_admin::AdminPasswordHashConcurrency::from(
            server_admin::StdAdminNonZeroUsize::from(std::num::NonZeroUsize::new(1).expect(
                "271f96d4 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
            )),
        ));
    let _admin_id = server_admin::bootstrap_admin(
        app_state::SqlxPgPoolRef::from(&pool.0),
        server_admin::AdminLogin::try_from(constants_str::ADMIN_ALT.to_owned()).expect(
            "98c7e04a postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
        ),
        server_admin::AdminDisplayName::try_from(constants_str::ADMIN.to_owned()).expect(
            "48efed01 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
        ),
        password,
        &hasher,
    )
    .await
    .expect("e2c94d67 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    let password_change_required = sqlx::query_scalar::<_, bool>(
        constants_str::SELECT_MUST_CHANGE_PASSWORD_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
    )
    .fetch_one(&pool.0)
    .await
    .expect("81f3c9d2 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert!(password_change_required);
    let original_password_hash = sqlx::query_scalar::<_, String>(
        constants_str::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
    )
    .fetch_one(&pool.0)
    .await
    .expect("1282b56e postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    let repeated_password = serde_json::from_str::<server_admin_contract::AdminNewPassword>(
        constants_str::DIFFERENT_PASSWORD,
    )
    .expect("e411f376 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert!(matches!(
        server_admin::bootstrap_admin(
            app_state::SqlxPgPoolRef::from(&pool.0),
            server_admin::AdminLogin::try_from("other_admin".to_owned()).expect(
                "8359ca1a postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold"
            ),
            server_admin::AdminDisplayName::try_from("Other Admin".to_owned()).expect(
                "d968dddb postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold"
            ),
            repeated_password,
            &hasher,
        )
        .await,
        Err(server_admin::AdminBootstrapError::AlreadyInitialized)
    ));
    let preserved_password_hash = sqlx::query_scalar::<_, String>(
        constants_str::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
    )
    .fetch_one(&pool.0)
    .await
    .expect("65ff827e postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(preserved_password_hash, original_password_hash);
    let administrator_count =
        sqlx::query_scalar::<_, i64>(constants_str::SELECT_COUNT_ASTERISK_FROM_ADMIN_USERS)
            .fetch_one(&pool.0)
            .await
            .expect(
                "ae89c3bd postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
            );
    assert_eq!(administrator_count, constants_i64::ONE);
    let admin_id =
        sqlx::query_scalar::<_, i64>(constants_str::SELECT_ID_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN)
            .fetch_one(&pool.0)
            .await
            .expect(
                "a61329bf postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
            );
    let dangling_role_links = sqlx::query_scalar::<_, i64>(
        constants_str::SELECT_COUNT_ASTERISK_FROM_ADMIN_USER_ROLES_LINK_LEFT_JOIN_ADMIN_USERS,
    )
    .fetch_one(&pool.0)
    .await
    .expect("08ef120f postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(dangling_role_links, constants_i64::ZERO);
    let dangling_permission_links = sqlx::query_scalar::<_, i64>(
        constants_str::SELECT_COUNT_ASTERISK_FROM_ADMIN_ROLE_PERMISSIONS_LINK_LEFT_JOIN_ADMIN_ROLES,
    )
    .fetch_one(&pool.0)
    .await
    .expect("aebf6dc8 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(dangling_permission_links, constants_i64::ZERO);
    let wrong_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(constants_str::LOGIN_ADMIN_PASSWORD_WRONG_PASSWORD),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("5472ea19 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(wrong_response.status(), http::StatusCode::UNAUTHORIZED);
    let sign_in_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(constants_str::LOGIN_ADMIN_PASSWORD_CORRECT_PASSWORD),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("c245193e postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(sign_in_response.status(), http::StatusCode::OK);
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
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("b67815ec postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(me_response.status(), http::StatusCode::OK);
    let changed_context_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer_at(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminMeRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(cookie.as_str())),
            None,
            StdAdminApiTestStrRef::from(constants_str::VALUE_127_0_0_2_43210),
        )
        .0,
    )
    .await
    .expect("f11e0324 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(first_refresh_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("9f0be285 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(refresh_response.status(), http::StatusCode::OK);
    let refreshed_access = cookie_value(
        HttpAdminApiTestResponseRef::from(&refresh_response),
        StdAdminApiTestStrRef::from(constants_str::ADMIN_ACCESS_TOKEN),
    );
    assert!(
        refresh_response
            .headers()
            .get_all(http::header::SET_COOKIE)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .any(|value| value.starts_with("admin_refresh_token="))
    );
    let rotated_refresh = cookie_value(
        HttpAdminApiTestResponseRef::from(&refresh_response),
        StdAdminApiTestStrRef::from(constants_str::ADMIN_REFRESH_TOKEN),
    );
    let refreshed_csrf = cookie_value(
        HttpAdminApiTestResponseRef::from(&refresh_response),
        StdAdminApiTestStrRef::from(constants_str::ADMIN_CSRF_TOKEN_ALT),
    );
    let active_cookie = format!(
        "admin_access_token={refreshed_access}; admin_refresh_token={rotated_refresh}; admin_csrf_token={refreshed_csrf}"
    );
    let reused_refresh_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminRefreshRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(first_refresh_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("b8c71e43 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(
        reused_refresh_response.status(),
        http::StatusCode::UNAUTHORIZED
    );
    let first_lockout_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSignInRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(constants_str::LOGIN_LOCKED_USER_PASSWORD_WRONG_PASSWORD),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("8f72b01e postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
            StdAdminApiTestStrRef::from(constants_str::LOGIN_LOCKED_USER_PASSWORD_WRONG_PASSWORD),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("2d94c01e postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
            StdAdminApiTestStrRef::from(constants_str::LOGIN_LOCKED_USER_PASSWORD_WRONG_PASSWORD),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("7324af80 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(
        limited_response.status(),
        http::StatusCode::TOO_MANY_REQUESTS
    );
    let password_change_gate_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(frontend_contract::typed_route_path::<server_admin_contract::AdminListUsersRoute>().as_ref()),
            StdAdminApiTestStrRef::from(constants_str::LOGIN_LIMITED_USER_DISPLAY_NAME_LIMITED_USER_PASSWORD_LIMITED_PASSWORD),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("d78b315c postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(
        password_change_gate_response.status(),
        http::StatusCode::FORBIDDEN
    );
    let change_password_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<
                    server_admin_contract::AdminChangeOwnPasswordRoute,
                >()
                .as_ref(),
            ),
            StdAdminApiTestStrRef::from(
                constants_str::CURRENT_PASSWORD_CORRECT_NEW_PASSWORD_CHANGED,
            ),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("820fbb75 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(
        change_password_response.status(),
        http::StatusCode::NO_CONTENT
    );
    let csrf_denied_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(frontend_contract::typed_route_path::<server_admin_contract::AdminListUsersRoute>().as_ref()),
            StdAdminApiTestStrRef::from(constants_str::LOGIN_LIMITED_USER_DISPLAY_NAME_LIMITED_USER_PASSWORD_LIMITED_PASSWORD),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("153b847c postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(csrf_denied_response.status(), http::StatusCode::FORBIDDEN);
    let create_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(frontend_contract::typed_route_path::<server_admin_contract::AdminListUsersRoute>().as_ref()),
            StdAdminApiTestStrRef::from(constants_str::LOGIN_LIMITED_USER_DISPLAY_NAME_LIMITED_USER_PASSWORD_LIMITED_PASSWORD),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("c86a4310 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
                constants_str::LOGIN_LIMITED_USER_PASSWORD_LIMITED_PASSWORD,
            ),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("a2d6139e postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(limited_sign_in_response.status(), http::StatusCode::OK);
    let limited_access = cookie_value(
        HttpAdminApiTestResponseRef::from(&limited_sign_in_response),
        StdAdminApiTestStrRef::from(constants_str::ADMIN_ACCESS_TOKEN),
    );
    let limited_refresh = cookie_value(
        HttpAdminApiTestResponseRef::from(&limited_sign_in_response),
        StdAdminApiTestStrRef::from(constants_str::ADMIN_REFRESH_TOKEN_ALT),
    );
    let limited_csrf = cookie_value(
        HttpAdminApiTestResponseRef::from(&limited_sign_in_response),
        StdAdminApiTestStrRef::from(constants_str::ADMIN_CSRF_TOKEN_ALT),
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
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(limited_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("617f08b9 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(forbidden_response.status(), http::StatusCode::FORBIDDEN);
    let revoke_all_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::DELETE),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminSessionsRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(limited_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(limited_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("0f51dc7a postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(revoke_all_response.status(), http::StatusCode::NO_CONTENT);
    let revoked_all_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminMeRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(limited_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("24ec178b postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(
        revoked_all_response.status(),
        http::StatusCode::UNAUTHORIZED
    );
    let limited_id = sqlx::query_scalar::<_, i64>(
        constants_str::SELECT_ID_FROM_ADMIN_USERS_WHERE_LOGIN_LIMITED_USER,
    )
    .fetch_one(&pool.0)
    .await
    .expect("10c8f7d2 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    let update_user_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::PATCH),
            StdAdminApiTestStrRef::from(format!("/users/{limited_id}").as_str()),
            StdAdminApiTestStrRef::from(constants_str::DISPLAY_NAME_UPDATED_USER),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("623cde18 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(update_user_response.status(), http::StatusCode::NO_CONTENT);
    let ban_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(format!("/users/{limited_id}/ban").as_str()),
            StdAdminApiTestStrRef::from(constants_str::IS_BANNED_TRUE),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("94a7e1cb postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(ban_response.status(), http::StatusCode::NO_CONTENT);
    let banned_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminMeRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(limited_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("fac2138b postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
                constants_str::LOGIN_LIMITED_USER_PASSWORD_LIMITED_PASSWORD,
            ),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("891d7ca2 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("475af63b postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(list_users_response.status(), http::StatusCode::OK);
    let list_roles_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminListRolesRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("c5f103da postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(list_roles_response.status(), http::StatusCode::OK);
    let create_role_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminListRolesRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(constants_str::NAME_TEMPORARY_ROLE),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("6d9384fe postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(create_role_response.status(), http::StatusCode::CREATED);
    let role_id = sqlx::query_scalar::<_, i64>(
        constants_str::SELECT_ID_FROM_ADMIN_ROLES_WHERE_NAME_TEMPORARY_ROLE,
    )
    .fetch_one(&pool.0)
    .await
    .expect("1e53a0c7 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    let assign_role_body =
        serde_json::to_string(&server_admin_contract::AdminSetUserRolesReq::new(
            empty_admin_role_ids(),
            one_admin_role_id(
                server_admin_contract::AdminRoleId::try_from(role_id).expect(
                    "a82fc2e5 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
                ),
            ),
        ))
        .expect("bf02e516 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
    .expect("f74095eb postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(assign_role_response.status(), http::StatusCode::NO_CONTENT);
    let stale_role_body = serde_json::to_string(&server_admin_contract::AdminSetUserRolesReq::new(
        empty_admin_role_ids(),
        empty_admin_role_ids(),
    ))
    .expect("1fd845d3 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
    .expect("170158fb postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(stale_role_response.status(), http::StatusCode::CONFLICT);
    let remove_role_body =
        serde_json::to_string(&server_admin_contract::AdminSetUserRolesReq::new(
            one_admin_role_id(
                server_admin_contract::AdminRoleId::try_from(role_id).expect(
                    "c8994c27 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
                ),
            ),
            empty_admin_role_ids(),
        ))
        .expect("23c416a1 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
    .expect("a895d91f postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(remove_role_response.status(), http::StatusCode::NO_CONTENT);
    let update_role_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::PATCH),
            StdAdminApiTestStrRef::from(format!("/roles/{role_id}").as_str()),
            StdAdminApiTestStrRef::from(constants_str::NAME_RENAMED_ROLE),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("4f08b7ec postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(update_role_response.status(), http::StatusCode::NO_CONTENT);
    let delete_role_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::DELETE),
            StdAdminApiTestStrRef::from(format!("/roles/{role_id}").as_str()),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("d7e1862c postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(delete_role_response.status(), http::StatusCode::NO_CONTENT);
    let delete_user_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::DELETE),
            StdAdminApiTestStrRef::from(format!("/users/{limited_id}").as_str()),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("c19be784 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(delete_user_response.status(), http::StatusCode::NO_CONTENT);
    let admin_role_id =
        sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL)
            .fetch_one(&pool.0)
            .await
            .expect(
                "20b5fb03 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
            );
    let remove_last_admin_role_body =
        serde_json::to_string(&server_admin_contract::AdminSetUserRolesReq::new(
            one_admin_role_id(
                server_admin_contract::AdminRoleId::try_from(admin_role_id).expect(
                    "84fe96c8 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
                ),
            ),
            empty_admin_role_ids(),
        ))
        .expect("1528b0d3 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
    .expect("fe0db65c postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(
        remove_last_admin_role_response.status(),
        http::StatusCode::CONFLICT
    );
    let last_admin_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::DELETE),
            StdAdminApiTestStrRef::from(format!("/users/{admin_id}").as_str()),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("e6175d82 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
                StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect("8103cd5f postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(audit_response.status(), http::StatusCode::OK);
    let audit_page =
        axum::body::to_bytes(audit_response.into_body(), constants_usize::VALUE_1_048_576)
            .await
            .map(|body| {
                serde_json::from_slice::<server_admin_contract::AdminAuditPage>(&body).expect(
                    "ed125d4a postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
                )
            })
            .expect(
                "50612a4d postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
            );
    assert!(audit_page.items().len() <= constants_usize::ONE);
    assert!(
        u64::from(audit_page.total())
            >= u64::try_from(audit_page.items().len()).expect(
                "03c133e9 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold"
            )
    );
    futures::StreamExt::fold(
        futures::stream::iter(constants_usize::ZERO..61usize),
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
                    StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                    Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
                    None,
                )
                .0,
            )
            .await
            .expect(
                "a6fa9aeb postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
            );
            assert_eq!(response.status(), http::StatusCode::OK);
        },
    )
    .await;

    let sessions_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from("/auth/sessions?limit=1&offset=0"),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("449bf918 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(sessions_response.status(), http::StatusCode::OK);
    let sessions_page = axum::body::to_bytes(
        sessions_response.into_body(),
        constants_usize::VALUE_1_048_576,
    )
    .await
    .map(|body| {
        serde_json::from_slice::<server_admin_contract::AdminSessionsPage>(&body)
            .expect("e544366c postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold")
    })
    .expect("141ddcdc postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert!(sessions_page.items().len() <= constants_usize::ONE);
    assert!(
        u64::from(sessions_page.total())
            >= u64::try_from(sessions_page.items().len()).expect(
                "701a7a79 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold"
            )
    );

    let data_table_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from("/tables/users?limit=1&offset=0"),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("ca94aec1 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(data_table_response.status(), http::StatusCode::OK);
    let data_table = axum::body::to_bytes(
        data_table_response.into_body(),
        constants_usize::VALUE_1_048_576,
    )
    .await
    .map(|body| {
        serde_json::from_slice::<server_admin_contract::AdminDataTableView>(&body)
            .expect("e16283f4 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold")
    })
    .expect("3f927581 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert!(data_table.items().len() <= constants_usize::ONE);
    assert!(
        u64::from(data_table.total())
            >= u64::try_from(data_table.items().len()).expect(
                "1440730f postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold"
            )
    );
    let filtered_data_table_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                format!(
                    "/tables/users?filter_field=login&filter_operation=eq&filter_value={}&limit=20&offset=0",
                    constants_str::ADMIN_ALT
                )
                .as_str(),
            ),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("766f5654 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(filtered_data_table_response.status(), http::StatusCode::OK);
    let filtered_data_table = axum::body::to_bytes(
        filtered_data_table_response.into_body(),
        constants_usize::VALUE_1_048_576,
    )
    .await
    .map(|body| {
        serde_json::from_slice::<server_admin_contract::AdminDataTableView>(&body)
            .expect("02d611ab postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold")
    })
    .expect("6dfe8f37 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(u64::from(filtered_data_table.total()), 1u64);
    assert_eq!(filtered_data_table.items().len(), constants_usize::ONE);
    assert!(
        filtered_data_table
            .items()
            .first()
            .expect("753fa97c postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold")
            .values()
            .iter()
            .any(|value| value.as_ref() == constants_str::ADMIN_ALT)
    );
    let empty_data_table_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from("/tables/users?filter_field=login&filter_operation=eq&filter_value=missing_filter_user&limit=20&offset=0"),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("1310e021 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(empty_data_table_response.status(), http::StatusCode::OK);
    let empty_data_table = axum::body::to_bytes(
        empty_data_table_response.into_body(),
        constants_usize::VALUE_1_048_576,
    )
    .await
    .map(|body| {
        serde_json::from_slice::<server_admin_contract::AdminDataTableView>(&body)
            .expect("aa8376d3 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold")
    })
    .expect("a98d6360 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(u64::from(empty_data_table.total()), constants_u64::ZERO);
    assert!(empty_data_table.items().is_empty());
    let unsupported_filter_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from("/tables/users?filter_field=login&filter_operation=between&filter_value=admin&filter_end=root&limit=20&offset=0"),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("dd6d2544 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("e9279b1f postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("ef71e50a postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(sign_out_response.status(), http::StatusCode::NO_CONTENT);
    let revoked_response = tower::ServiceExt::oneshot(
        router_with_pool(&pool).0,
        request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::GET),
            StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::<server_admin_contract::AdminMeRoute>()
                    .as_ref(),
            ),
            StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("54b9dc03 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
    assert_eq!(revoked_response.status(), http::StatusCode::UNAUTHORIZED);
    let audit_outcomes = sqlx::query_as::<_, (bool, i64)>(constants_str::SELECT_SUCCEEDED_COUNT_ASTERISK_FROM_ADMIN_AUDIT_LOG_GROUP_BY_SUCCEEDED_ORDER)
        .fetch_all(&pool.0)
        .await
        .expect("3de105a4 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
#[cfg(test)]
use super::{
    HttpAdminApiTestMethod, HttpAdminApiTestResponseRef, SqlxAdminApiTestPool,
    StdAdminApiTestStrRef, cookie_value, empty_admin_role_ids, one_admin_role_id,
    request_with_peer, request_with_peer_at, router_with_pool,
};
