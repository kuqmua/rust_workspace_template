// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(unused_crate_dependencies)]
// integration target inherits the library dependency graph while exercising the assembled public router
// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::tests_outside_test_module)] // every item in this integration target is compiled exclusively by the test harness
mod data_tables {
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn postgresql_data_table_api_reads_every_public_field_from_every_table() {
        let fixture = crate::admin_html_test_fixture().await;
        let _cleanup_status = sqlx::query(
        constants_str::test_fixtures::VALUE_6E1CBD4B,
    )
    .execute(&fixture.pool.0)
    .await
    .expect("70dfa001 postgresql_data_table_api_reads_every_public_field_from_every_table invariant must hold");
        let _rate_limit = sqlx::query(
        constants_str::test_fixtures::VALUE_91A1975C,
    )
    .execute(&fixture.pool.0)
    .await
    .expect("f8f27048 postgresql_data_table_api_reads_every_public_field_from_every_table invariant must hold");
        let fixture_ref = &fixture;
        futures::StreamExt::fold(
        futures::stream::iter(server_admin_contract::admin_data_table::AdminDataTable::PG_ORDER),
        (),
        async |(), table| {
            let uri = format!("/tables/{table}?limit=100&offset=0");
            let response = tower::ServiceExt::oneshot(
                crate::router_with_pool(&fixture_ref.pool).0,
                crate::request_with_peer(
                    super::HttpAdminApiTestMethod::from(http::Method::GET),
                    super::StdAdminApiTestStrRef::from(uri.as_str()),
                    super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
                    Some(super::StdAdminApiTestStrRef::from(fixture_ref.cookie.0.as_str())),
                    None,
                )
                .0,
            )
            .await
            .expect("4b58a9ba postgresql_data_table_api_reads_every_public_field_from_every_table invariant must hold");
            assert_eq!(
                response.status(),
                http::StatusCode::OK,
                "table API {table} failed"
            );
            let body = axum::body::to_bytes(response.into_body(), constants_usize::VALUE_1_048_576)
                .await
                .expect("78547eed postgresql_data_table_api_reads_every_public_field_from_every_table invariant must hold");
            let view =
                serde_json::from_slice::<server_admin_contract::admin_data_table_view::AdminDataTableView>(body.as_ref())
                    .expect("6d2a32e6 postgresql_data_table_api_reads_every_public_field_from_every_table invariant must hold");
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
        fixture.lock.0.rollback().await.expect("83226fd7 postgresql_data_table_api_reads_every_public_field_from_every_table invariant must hold");
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn postgresql_generated_mutation_idempotency_contract() {
        let database_url = std::env::var(constants_str::catalog::ENV_NAMES_DATABASE_URL).expect(
            "40c1e398 postgresql_generated_mutation_idempotency_contract invariant must hold",
        );
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4u32)
            .connect(database_url.as_str())
            .await
            .expect(
                "cb6830bc postgresql_generated_mutation_idempotency_contract invariant must hold",
            );
        let mut idempotency_test_isolation = pool.begin().await.expect(
            "ea1d891d postgresql_generated_mutation_idempotency_contract invariant must hold",
        );
        pg_crud_common::lock_pg_relation_resources::lock_pg_relation_resources(
            pg_crud_common::sqlx_pg_relation_lock_connection_ref::SqlxPgRelationLockConnectionRef::from(
                &mut *idempotency_test_isolation,
            ),
            &pg_crud_common::pg_relation_lock_namespace::PgRelationLockNamespace::try_from(
                constants_str::catalog::ACTOR_ATOMIC.to_owned(),
            )
            .expect(
                "136c5acc postgresql_generated_mutation_idempotency_contract invariant must hold",
            ),
            &pg_crud_common::pg_relation_resource_ids::PgRelationResourceIds::try_from(vec![
                pg_crud_common::pg_relation_resource_id::PgRelationResourceId::from(constants_i64::ONE),
            ])
            .expect(
                "8b0c7ae1 postgresql_generated_mutation_idempotency_contract invariant must hold",
            ),
        )
        .await
        .expect("508db033 postgresql_generated_mutation_idempotency_contract invariant must hold");
        pg_table::ensure_pg_table_idempotency_schema::ensure_pg_table_idempotency_schema(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
        )
        .await
        .expect("6c338824 postgresql_generated_mutation_idempotency_contract invariant must hold");
        let _truncate_result = sqlx::query(constants_str::catalog::TRUNCATE_PG_TABLE_IDEMPOTENCY)
            .execute(&pool)
            .await
            .expect(
                "d93beb69 postgresql_generated_mutation_idempotency_contract invariant must hold",
            );
        let make_request =
            |actor: super::StdAdminApiTestStrRef<'_>,
             route: super::StdAdminApiTestStrRef<'_>,
             key: super::StdAdminApiTestStrRef<'_>,
             body: pg_table::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef<'_>| {
                pg_table::pg_table_idempotency_request::PgTableIdempotencyRequest::new(
            pg_table::pg_table_idempotency_scope::PgTableIdempotencyScope::new(
                pg_table::pg_table_idempotency_actor::PgTableIdempotencyActor::try_from(actor.0.to_owned()).expect("e6640036 postgresql_generated_mutation_idempotency_contract invariant must hold"),
                pg_table::pg_table_idempotency_method::PgTableIdempotencyMethod::try_from(constants_str::catalog::POST.to_owned())
                    .expect("94bc0508 postgresql_generated_mutation_idempotency_contract invariant must hold"),
                pg_table::pg_table_idempotency_route::PgTableIdempotencyRoute::try_from(route.0.to_owned()).expect("4e8c040f postgresql_generated_mutation_idempotency_contract invariant must hold"),
                pg_table::pg_table_idempotency_key::PgTableIdempotencyKey::try_from(key.0.to_owned()).expect("2028024d postgresql_generated_mutation_idempotency_contract invariant must hold"),
            ),
            body,
        )
            };
        let first_request = make_request(
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ACTOR_A),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ITEMS_CM),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::KEY_A),
            pg_table::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(
                br#"{"value":1}"#.as_slice(),
            ),
        );
        let first = pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            &first_request,
        )
        .await
        .expect("c8b3565c postgresql_generated_mutation_idempotency_contract invariant must hold");
        assert_eq!(
            first,
            pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::Acquired
        );
        let pending = pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            &first_request,
        )
        .await
        .expect("c5c45332 postgresql_generated_mutation_idempotency_contract invariant must hold");
        assert_eq!(
            pending,
            pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::InProgress
        );
        let conflicting_request = make_request(
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ACTOR_A),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ITEMS_CM),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::KEY_A),
            pg_table::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(
                br#"{"value":2}"#.as_slice(),
            ),
        );
        let conflict = pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            &conflicting_request,
        )
        .await
        .expect("7f419767 postgresql_generated_mutation_idempotency_contract invariant must hold");
        assert_eq!(
            conflict,
            pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::Conflict
        );
        let response_body = br#"{"desirable":{"id":1}}"#;
        pg_table::complete_pg_table_idempotency::complete_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            &first_request,
            pg_table::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus::try_from(201u16).expect(
                "4df2dd1f postgresql_generated_mutation_idempotency_contract invariant must hold",
            ),
            pg_table::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(response_body.as_slice()),
        )
        .await
        .expect("9106c1e6 postgresql_generated_mutation_idempotency_contract invariant must hold");
        let replay = pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            &first_request,
        )
        .await
        .expect("0721b23f postgresql_generated_mutation_idempotency_contract invariant must hold");
        let pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::Replay(replay_value) =
            replay
        else {
            panic!("9f97fb0d");
        };
        assert_eq!(
        replay_value.into_parts(),
        (
            pg_table::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus::try_from(201u16).expect(
                "f89d923d postgresql_generated_mutation_idempotency_contract invariant must hold"
            ),
            pg_table::pg_table_idempotency_body::PgTableIdempotencyBody::try_from(response_body.to_vec()).expect(
                "4a01ed0e postgresql_generated_mutation_idempotency_contract invariant must hold"
            ),
        )
    );
        let other_actor = make_request(
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ACTOR_B),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ITEMS_CM),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::KEY_A),
            pg_table::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(
                br#"{"value":1}"#.as_slice(),
            ),
        );
        assert_eq!(
            pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
                app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
                &other_actor
            )
            .await
            .expect(
                "e581d572 postgresql_generated_mutation_idempotency_contract invariant must hold"
            ),
            pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::Acquired
        );
        pg_table::release_pg_table_idempotency::release_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            &other_actor,
        )
        .await
        .expect("31e0437d postgresql_generated_mutation_idempotency_contract invariant must hold");
        assert_eq!(
            pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
                app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
                &other_actor
            )
            .await
            .expect(
                "fe57d4dc postgresql_generated_mutation_idempotency_contract invariant must hold"
            ),
            pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::Acquired
        );
        let concurrent = make_request(
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ACTOR_CONCURRENT),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ITEMS_CM),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::KEY_CONCURRENT),
            pg_table::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(
                br#"{"value":3}"#.as_slice(),
            ),
        );
        let (left, right) = tokio::join!(
            pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
                app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
                &concurrent
            ),
            pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
                app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
                &concurrent
            )
        );
        let outcomes = [
            left.expect(
                "874153ec postgresql_generated_mutation_idempotency_contract invariant must hold",
            ),
            right.expect(
                "64c4cc46 postgresql_generated_mutation_idempotency_contract invariant must hold",
            ),
        ];
        assert_eq!(
            outcomes
                .iter()
                .filter(|outcome| **outcome
                    == pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::Acquired)
                .count(),
            constants_usize::ONE
        );
        assert_eq!(
            outcomes
                .iter()
                .filter(|outcome| **outcome
                    == pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::InProgress)
                .count(),
            constants_usize::ONE
        );
        let _atomic_table = sqlx::query(
            constants_str::catalog::CREATE_TABLE_IF_NOT_EXISTS_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST_ID_BIGINT,
        )
        .execute(&pool)
        .await
        .expect("af066e8b postgresql_generated_mutation_idempotency_contract invariant must hold");
        let _atomic_clear = sqlx::query(
            constants_str::catalog::TRUNCATE_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST,
        )
        .execute(&pool)
        .await
        .expect("3130e593 postgresql_generated_mutation_idempotency_contract invariant must hold");
        let atomic = make_request(
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ACTOR_ATOMIC),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ITEMS_CO),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::KEY_ATOMIC),
            pg_table::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(
                br#"{"id":1}"#.as_slice(),
            ),
        );
        assert_eq!(
            pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
                app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
                &atomic
            )
            .await
            .expect(
                "925ea283 postgresql_generated_mutation_idempotency_contract invariant must hold"
            ),
            pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::Acquired
        );
        let mut rollback_tx = pool.begin().await.expect(
            "fcba80e1 postgresql_generated_mutation_idempotency_contract invariant must hold",
        );
        let _mutation = sqlx::query(
            constants_str::catalog::INSERT_INTO_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST_ID_VALUES_1,
        )
        .execute(&mut *rollback_tx)
        .await
        .expect("67503e70 postgresql_generated_mutation_idempotency_contract invariant must hold");
        pg_table::complete_pg_table_idempotency_in_connection::complete_pg_table_idempotency_in_connection(
            pg_table::sqlx_pg_table_pg_connection_ref::SqlxPgTablePgConnectionRef::from(&mut *rollback_tx),
            &atomic,
            pg_table::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus::try_from(201u16).expect(
                "98bb1db9 postgresql_generated_mutation_idempotency_contract invariant must hold",
            ),
            pg_table::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(br#"{"id":1}"#.as_slice()),
        )
        .await
        .expect("8ad86515 postgresql_generated_mutation_idempotency_contract invariant must hold");
        rollback_tx.rollback().await.expect(
            "11cfcb27 postgresql_generated_mutation_idempotency_contract invariant must hold",
        );
        let mutation_count = sqlx::query_scalar::<_, i64>(
            constants_str::catalog::SELECT_COUNT_ASTERISK_FROM_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST,
        )
        .fetch_one(&pool)
        .await
        .expect("84e57ab6 postgresql_generated_mutation_idempotency_contract invariant must hold");
        assert_eq!(mutation_count, constants_i64::ZERO);
        assert_eq!(
            pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
                app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
                &atomic
            )
            .await
            .expect(
                "3903bf53 postgresql_generated_mutation_idempotency_contract invariant must hold"
            ),
            pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::InProgress
        );
        pg_table::release_pg_table_idempotency::release_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            &atomic,
        )
        .await
        .expect("67973e68 postgresql_generated_mutation_idempotency_contract invariant must hold");
        let _age_records = sqlx::query(
            constants_str::catalog::UPDATE_PG_TABLE_IDEMPOTENCY_SET_CREATED_AT_TIMESTAMPTZ_2000_01_01_00,
        )
        .execute(&pool)
        .await
        .expect("a46f7336 postgresql_generated_mutation_idempotency_contract invariant must hold");
        let before_cleanup = sqlx::query_scalar::<_, i64>(
            constants_str::catalog::SELECT_COUNT_ASTERISK_FROM_PG_TABLE_IDEMPOTENCY,
        )
        .fetch_one(&pool)
        .await
        .expect("2c080f6d postgresql_generated_mutation_idempotency_contract invariant must hold");
        let cleaned = pg_table::cleanup_pg_table_idempotency::cleanup_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            pg_table::pg_table_idempotency_cleanup_retention_seconds::PgTableIdempotencyCleanupRetentionSeconds::try_from(3_600i64).expect(
                "52189299 postgresql_generated_mutation_idempotency_contract invariant must hold",
            ),
            pg_table::pg_table_idempotency_cleanup_retention_seconds::PgTableIdempotencyCleanupRetentionSeconds::try_from(3_600i64).expect(
                "fa6dc1d7 postgresql_generated_mutation_idempotency_contract invariant must hold",
            ),
            pg_table::pg_table_idempotency_cleanup_batch_size::PgTableIdempotencyCleanupBatchSize::try_from(2i64).expect(
                "1780d6b1 postgresql_generated_mutation_idempotency_contract invariant must hold",
            ),
        )
        .await
        .expect("b1ba49cc postgresql_generated_mutation_idempotency_contract invariant must hold");
        assert_eq!(u64::from(cleaned), 2u64);
        let after_cleanup = sqlx::query_scalar::<_, i64>(
            constants_str::catalog::SELECT_COUNT_ASTERISK_FROM_PG_TABLE_IDEMPOTENCY,
        )
        .fetch_one(&pool)
        .await
        .expect("6863201e postgresql_generated_mutation_idempotency_contract invariant must hold");
        assert_eq!(
            before_cleanup.checked_sub(after_cleanup).expect(
                "f93ed3cf postgresql_generated_mutation_idempotency_contract invariant must hold"
            ),
            2i64
        );
    }
}
mod flow {
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn flow() {
        let database_url = std::env::var(constants_str::catalog::ENV_NAMES_DATABASE_URL).expect(
            "ac0cb9e3 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
        );
        let pool = super::SqlxAdminApiTestPool::from(
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(database_url.as_str())
                .await
                .expect(
                    "a3e1f57c postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
                ),
        );
        let mut admin_db_test_lock = pool.0.begin().await.expect(
            "4dfb6865 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
        );
        let _locked = sqlx::query(
            constants_str::integration_fixtures::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS,
        )
        .execute(&mut *admin_db_test_lock)
        .await
        .expect("693b147f postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        server_admin::prepare_postgresql::prepare_postgresql(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
        )
        .await
        .expect("0ea8d516 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        server_admin::prepare_postgresql::prepare_postgresql(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
        )
        .await
        .expect("676c00f1 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        server_admin::validate_catalog_schema::validate_catalog_schema(
            pg_crud_common::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef::from(&pool.0),
            pg_crud_common::db_schema_name_ref::DbSchemaNameRef::from(
                constants_str::catalog::PUBLIC,
            ),
        )
        .await
        .expect("65ce07e9 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let observed_permissions = sqlx::query_scalar::<_, String>(
            constants_str::test_fixtures::SELECT_NAME_FROM_ADMIN_PERMISSIONS_ORDER_BY_NAME,
        )
        .fetch_all(&pool.0)
        .await
        .expect("db765f20 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let expected_permissions = server_admin_contract::admin_permission::AdminPermission::ALL
            .into_iter()
            .map(|permission| permission.as_str().as_ref().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(observed_permissions, expected_permissions);
        let _deleted_permission = sqlx::query(
            constants_str::test_fixtures::DELETE_ADMIN_PERMISSION_BY_NAME,
        )
        .bind(
            server_admin_contract::admin_permission::AdminPermission::ALL
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
        server_admin::prepare_postgresql::prepare_postgresql(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
        )
        .await
        .expect("ea3f641d postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let reconciled_permissions = sqlx::query_scalar::<_, String>(
            constants_str::test_fixtures::SELECT_NAME_FROM_ADMIN_PERMISSIONS_ORDER_BY_NAME,
        )
        .fetch_all(&pool.0)
        .await
        .expect("458ab19e postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(reconciled_permissions, expected_permissions);
        let _truncate_result = sqlx::query(
        constants_str::catalog::TRUNCATE_ADMIN_RATE_LIMITS_ADMIN_AUDIT_LOG_ADMIN_LOGIN_ATTEMPTS_ADMIN_ACCESS,
    )
    .execute(&pool.0)
    .await
    .expect("97b5ad2f postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let password = serde_json::from_str::<
            server_admin_contract::admin_new_password::AdminNewPassword,
        >(constants_str::catalog::CORRECT_PASSWORD)
        .expect("703a8df2 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let hasher = server_admin::admin_password_hasher::AdminPasswordHasher::new(
            server_admin::runtime_admin_password_hash_concurrency::RuntimeAdminPasswordHashConcurrency::from(
                std::num::NonZeroUsize::new(1).expect(
                    "271f96d4 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
                ),
            ),
        );
        let _admin_id = server_admin::create_initial_administrator::create_initial_administrator(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
            server_admin_contract::admin_login::AdminLogin::try_from(
                constants_str::catalog::ADMIN_ALT.to_owned(),
            )
            .expect(
                "98c7e04a postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
            ),
            server_admin_contract::admin_display_name::AdminDisplayName::try_from(
                constants_str::catalog::ADMIN.to_owned(),
            )
            .expect(
                "48efed01 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
            ),
            password,
            &hasher,
        )
        .await
        .expect("e2c94d67 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let password_change_required = sqlx::query_scalar::<_, bool>(
            constants_str::integration_fixtures::SELECT_MUST_CHANGE_PASSWORD_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
        )
        .fetch_one(&pool.0)
        .await
        .expect("81f3c9d2 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert!(password_change_required);
        let original_password_hash = sqlx::query_scalar::<_, String>(
            constants_str::catalog::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
        )
        .fetch_one(&pool.0)
        .await
        .expect("1282b56e postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let repeated_password = serde_json::from_str::<
            server_admin_contract::admin_new_password::AdminNewPassword,
        >(constants_str::catalog::DIFFERENT_PASSWORD)
        .expect("e411f376 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert!(matches!(
        server_admin::create_initial_administrator::create_initial_administrator(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
            server_admin_contract::admin_login::AdminLogin::try_from("other_admin".to_owned()).expect(
                "8359ca1a postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold"
            ),
            server_admin_contract::admin_display_name::AdminDisplayName::try_from("Other Admin".to_owned())
                .expect(
                    "d968dddb postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold"
                ),
            repeated_password,
            &hasher,
        )
        .await,
        Err(server_admin::initial_administrator_creation_error::InitialAdministratorCreationError::AlreadyInitialized)
    ));
        let preserved_password_hash = sqlx::query_scalar::<_, String>(
            constants_str::catalog::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
        )
        .fetch_one(&pool.0)
        .await
        .expect("65ff827e postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(preserved_password_hash, original_password_hash);
        let administrator_count = sqlx::query_scalar::<_, i64>(
            constants_str::catalog::SELECT_COUNT_ASTERISK_FROM_ADMIN_USERS,
        )
        .fetch_one(&pool.0)
        .await
        .expect("ae89c3bd postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(administrator_count, constants_i64::ONE);
        let admin_id = sqlx::query_scalar::<_, i64>(
            constants_str::catalog::SELECT_ID_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
        )
        .fetch_one(&pool.0)
        .await
        .expect("a61329bf postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let dangling_role_links = sqlx::query_scalar::<_, i64>(
            constants_str::catalog::SELECT_COUNT_ASTERISK_FROM_ADMIN_USER_ROLES_LINK_LEFT_JOIN_ADMIN_USERS,
        )
        .fetch_one(&pool.0)
        .await
        .expect("08ef120f postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(dangling_role_links, constants_i64::ZERO);
        let dangling_permission_links = sqlx::query_scalar::<_, i64>(
        constants_str::catalog::SELECT_COUNT_ASTERISK_FROM_ADMIN_ROLE_PERMISSIONS_LINK_LEFT_JOIN_ADMIN_ROLES,
    )
    .fetch_one(&pool.0)
    .await
    .expect("aebf6dc8 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(dangling_permission_links, constants_i64::ZERO);
        let wrong_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_sign_in_route::AdminSignInRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::integration_fixtures::LOGIN_ADMIN_PASSWORD_WRONG_PASSWORD,
                ),
                None,
                None,
            )
            .0,
        )
        .await
        .expect("5472ea19 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(wrong_response.status(), http::StatusCode::UNAUTHORIZED);
        let sign_in_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_sign_in_route::AdminSignInRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::integration_fixtures::LOGIN_ADMIN_PASSWORD_CORRECT_PASSWORD,
                ),
                None,
                None,
            )
            .0,
        )
        .await
        .expect("c245193e postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(sign_in_response.status(), http::StatusCode::OK);
        let access = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&sign_in_response),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ADMIN_ACCESS_TOKEN),
        );
        let refresh = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&sign_in_response),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ADMIN_REFRESH_TOKEN_ALT),
        );
        let csrf = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&sign_in_response),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ADMIN_CSRF_TOKEN_ALT),
        );
        let cookie = format!(
            "admin_access_token={access}; admin_refresh_token={refresh}; admin_csrf_token={csrf}"
        );
        let me_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_me_route::AdminMeRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect("b67815ec postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(me_response.status(), http::StatusCode::OK);
        let changed_context_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer_at(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_me_route::AdminMeRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(cookie.as_str())),
                None,
                super::StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_127_0_0_2_43210),
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
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_refresh_route::AdminRefreshRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(
                    first_refresh_cookie.as_str(),
                )),
                None,
            )
            .0,
        )
        .await
        .expect("9f0be285 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(refresh_response.status(), http::StatusCode::OK);
        let refreshed_access = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&refresh_response),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ADMIN_ACCESS_TOKEN),
        );
        assert!(
            refresh_response
                .headers()
                .get_all(http::header::SET_COOKIE)
                .iter()
                .filter_map(|value| value.to_str().ok())
                .any(|value| value.starts_with("admin_refresh_token="))
        );
        let rotated_refresh = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&refresh_response),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ADMIN_REFRESH_TOKEN),
        );
        let refreshed_csrf = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&refresh_response),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ADMIN_CSRF_TOKEN_ALT),
        );
        let active_cookie = format!(
            "admin_access_token={refreshed_access}; admin_refresh_token={rotated_refresh}; admin_csrf_token={refreshed_csrf}"
        );
        let reused_refresh_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_refresh_route::AdminRefreshRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(
                    first_refresh_cookie.as_str(),
                )),
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
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_sign_in_route::AdminSignInRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::integration_fixtures::LOGIN_LOCKED_USER_PASSWORD_WRONG_PASSWORD,
                ),
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
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_sign_in_route::AdminSignInRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::integration_fixtures::LOGIN_LOCKED_USER_PASSWORD_WRONG_PASSWORD,
                ),
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
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_sign_in_route::AdminSignInRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::integration_fixtures::LOGIN_LOCKED_USER_PASSWORD_WRONG_PASSWORD,
                ),
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
        crate::router_with_pool(&pool).0,
        crate::request_with_peer(
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(frontend_contract::typed_route_path::typed_route_path::<server_admin_contract::admin_list_users_route::AdminListUsersRoute>().as_ref()),
            super::StdAdminApiTestStrRef::from(constants_str::integration_fixtures::LOGIN_LIMITED_USER_DISPLAY_NAME_LIMITED_USER_PASSWORD_LIMITED_PASSWORD),
            Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(super::StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
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
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_change_own_password_route::AdminChangeOwnPasswordRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::integration_fixtures::CURRENT_PASSWORD_CORRECT_NEW_PASSWORD_CHANGED,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
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
        crate::router_with_pool(&pool).0,
        crate::request_with_peer(
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(frontend_contract::typed_route_path::typed_route_path::<server_admin_contract::admin_list_users_route::AdminListUsersRoute>().as_ref()),
            super::StdAdminApiTestStrRef::from(constants_str::integration_fixtures::LOGIN_LIMITED_USER_DISPLAY_NAME_LIMITED_USER_PASSWORD_LIMITED_PASSWORD),
            Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect("153b847c postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(csrf_denied_response.status(), http::StatusCode::FORBIDDEN);
        let create_response = tower::ServiceExt::oneshot(
        crate::router_with_pool(&pool).0,
        crate::request_with_peer(
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(frontend_contract::typed_route_path::typed_route_path::<server_admin_contract::admin_list_users_route::AdminListUsersRoute>().as_ref()),
            super::StdAdminApiTestStrRef::from(constants_str::integration_fixtures::LOGIN_LIMITED_USER_DISPLAY_NAME_LIMITED_USER_PASSWORD_LIMITED_PASSWORD),
            Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(super::StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect("c86a4310 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(create_response.status(), http::StatusCode::CREATED);
        let limited_sign_in_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_sign_in_route::AdminSignInRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::integration_fixtures::LOGIN_LIMITED_USER_PASSWORD_LIMITED_PASSWORD,
                ),
                None,
                None,
            )
            .0,
        )
        .await
        .expect("a2d6139e postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(limited_sign_in_response.status(), http::StatusCode::OK);
        let limited_access = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&limited_sign_in_response),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ADMIN_ACCESS_TOKEN),
        );
        let limited_refresh = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&limited_sign_in_response),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ADMIN_REFRESH_TOKEN_ALT),
        );
        let limited_csrf = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&limited_sign_in_response),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::ADMIN_CSRF_TOKEN_ALT),
        );
        let limited_cookie = format!(
            "admin_access_token={limited_access}; admin_refresh_token={limited_refresh}; admin_csrf_token={limited_csrf}"
        );
        let forbidden_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_list_users_route::AdminListUsersRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(limited_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect("617f08b9 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(forbidden_response.status(), http::StatusCode::FORBIDDEN);
        let revoke_all_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::DELETE),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_sessions_route::AdminSessionsRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(limited_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(limited_csrf.0.as_str())),
            )
            .0,
        )
        .await
        .expect("0f51dc7a postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(revoke_all_response.status(), http::StatusCode::NO_CONTENT);
        let revoked_all_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_me_route::AdminMeRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(limited_cookie.as_str())),
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
            constants_str::catalog::SELECT_ID_FROM_ADMIN_USERS_WHERE_LOGIN_LIMITED_USER,
        )
        .fetch_one(&pool.0)
        .await
        .expect("10c8f7d2 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let update_user_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::PATCH),
                super::StdAdminApiTestStrRef::from(format!("/users/{limited_id}").as_str()),
                super::StdAdminApiTestStrRef::from(
                    constants_str::integration_fixtures::DISPLAY_NAME_UPDATED_USER,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect("623cde18 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(update_user_response.status(), http::StatusCode::NO_CONTENT);
        let ban_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(format!("/users/{limited_id}/ban").as_str()),
                super::StdAdminApiTestStrRef::from(
                    constants_str::integration_fixtures::IS_BANNED_TRUE,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect("94a7e1cb postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(ban_response.status(), http::StatusCode::NO_CONTENT);
        let banned_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_me_route::AdminMeRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(limited_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect("fac2138b postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(banned_response.status(), http::StatusCode::UNAUTHORIZED);
        let banned_sign_in_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_sign_in_route::AdminSignInRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::integration_fixtures::LOGIN_LIMITED_USER_PASSWORD_LIMITED_PASSWORD,
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
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_list_users_route::AdminListUsersRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect("475af63b postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(list_users_response.status(), http::StatusCode::OK);
        let list_roles_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_list_roles_route::AdminListRolesRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect("c5f103da postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(list_roles_response.status(), http::StatusCode::OK);
        let create_role_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_list_roles_route::AdminListRolesRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::integration_fixtures::NAME_TEMPORARY_ROLE,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect("6d9384fe postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(create_role_response.status(), http::StatusCode::CREATED);
        let role_id = sqlx::query_scalar::<_, i64>(
            constants_str::catalog::SELECT_ID_FROM_ADMIN_ROLES_WHERE_NAME_TEMPORARY_ROLE,
        )
        .fetch_one(&pool.0)
        .await
        .expect("1e53a0c7 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let assign_role_body = serde_json::to_string(
        &server_admin_contract::admin_set_user_roles_req::AdminSetUserRolesReq::new(
            crate::empty_admin_role_ids(),
            crate::one_admin_role_id(
                server_admin_contract::admin_role_id::AdminRoleId::try_from(role_id).expect(
                    "a82fc2e5 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
                ),
            ),
        ),
    )
    .expect("bf02e516 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let assign_role_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::PUT),
                super::StdAdminApiTestStrRef::from(format!("/users/{limited_id}/roles").as_str()),
                super::StdAdminApiTestStrRef::from(assign_role_body.as_str()),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect("f74095eb postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(assign_role_response.status(), http::StatusCode::NO_CONTENT);
        let stale_role_body = serde_json::to_string(
            &server_admin_contract::admin_set_user_roles_req::AdminSetUserRolesReq::new(
                crate::empty_admin_role_ids(),
                crate::empty_admin_role_ids(),
            ),
        )
        .expect("1fd845d3 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let stale_role_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::PUT),
                super::StdAdminApiTestStrRef::from(format!("/users/{limited_id}/roles").as_str()),
                super::StdAdminApiTestStrRef::from(stale_role_body.as_str()),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect("170158fb postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(stale_role_response.status(), http::StatusCode::CONFLICT);
        let remove_role_body = serde_json::to_string(
        &server_admin_contract::admin_set_user_roles_req::AdminSetUserRolesReq::new(
            crate::one_admin_role_id(
                server_admin_contract::admin_role_id::AdminRoleId::try_from(role_id).expect(
                    "c8994c27 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
                ),
            ),
            crate::empty_admin_role_ids(),
        ),
    )
    .expect("23c416a1 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let remove_role_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::PUT),
                super::StdAdminApiTestStrRef::from(format!("/users/{limited_id}/roles").as_str()),
                super::StdAdminApiTestStrRef::from(remove_role_body.as_str()),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect("a895d91f postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(remove_role_response.status(), http::StatusCode::NO_CONTENT);
        let update_role_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::PATCH),
                super::StdAdminApiTestStrRef::from(format!("/roles/{role_id}").as_str()),
                super::StdAdminApiTestStrRef::from(
                    constants_str::integration_fixtures::NAME_RENAMED_ROLE,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect("4f08b7ec postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(update_role_response.status(), http::StatusCode::NO_CONTENT);
        let delete_role_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::DELETE),
                super::StdAdminApiTestStrRef::from(format!("/roles/{role_id}").as_str()),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect("d7e1862c postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(delete_role_response.status(), http::StatusCode::NO_CONTENT);
        let delete_user_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::DELETE),
                super::StdAdminApiTestStrRef::from(format!("/users/{limited_id}").as_str()),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect("c19be784 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(delete_user_response.status(), http::StatusCode::NO_CONTENT);
        let admin_role_id = sqlx::query_scalar::<_, i64>(
            constants_str::integration_fixtures::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL,
        )
        .fetch_one(&pool.0)
        .await
        .expect("20b5fb03 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let remove_last_admin_role_body = serde_json::to_string(
        &server_admin_contract::admin_set_user_roles_req::AdminSetUserRolesReq::new(
            crate::one_admin_role_id(
                server_admin_contract::admin_role_id::AdminRoleId::try_from(admin_role_id).expect(
                    "84fe96c8 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
                ),
            ),
            crate::empty_admin_role_ids(),
        ),
    )
    .expect("1528b0d3 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        let remove_last_admin_role_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::PUT),
                super::StdAdminApiTestStrRef::from(format!("/users/{admin_id}/roles").as_str()),
                super::StdAdminApiTestStrRef::from(remove_last_admin_role_body.as_str()),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
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
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::DELETE),
                super::StdAdminApiTestStrRef::from(format!("/users/{admin_id}").as_str()),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect("e6175d82 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(last_admin_response.status(), http::StatusCode::CONFLICT);
        let audit_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(
                    format!(
                        "{}?limit=1&offset=1",
                        frontend_contract::typed_route_path::typed_route_path::<
                            server_admin_contract::admin_audit_log_route::AdminAuditLogRoute,
                        >()
                    )
                    .as_str(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect("8103cd5f postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(audit_response.status(), http::StatusCode::OK);
        let audit_page = axum::body::to_bytes(
            audit_response.into_body(),
            constants_usize::VALUE_1_048_576,
        )
        .await
        .map(|body| {
            serde_json::from_slice::<server_admin_contract::admin_audit_page::AdminAuditPage>(&body)
                .expect(
                    "ed125d4a postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
                )
        })
        .expect("50612a4d postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
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
                    crate::router_with_pool(&pool).0,
                    crate::request_with_peer(
                        super::HttpAdminApiTestMethod::from(http::Method::GET),
                        super::StdAdminApiTestStrRef::from(
                            format!(
                                "{}?limit=1&offset=0",
                                frontend_contract::typed_route_path::typed_route_path::<
                                    server_admin_contract::admin_audit_log_route::AdminAuditLogRoute,
                                >()
                            )
                            .as_str(),
                        ),
                        super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
                        Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
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
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_9B6938A5),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
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
            serde_json::from_slice::<server_admin_contract::admin_sessions_page::AdminSessionsPage>(
                &body,
            )
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
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_8F292E26),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
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
            serde_json::from_slice::<
                    server_admin_contract::admin_data_table_view::AdminDataTableView,
                >(&body)
                .expect(
                    "e16283f4 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
                )
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
        crate::router_with_pool(&pool).0,
        crate::request_with_peer(
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                format!(
                    "/tables/users?filter_field=login&filter_operation=eq&filter_value={}&limit=20&offset=0",
                    constants_str::catalog::ADMIN_ALT
                )
                .as_str(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
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
            serde_json::from_slice::<
                    server_admin_contract::admin_data_table_view::AdminDataTableView,
                >(&body)
                .expect(
                    "02d611ab postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
                )
        })
        .expect("6dfe8f37 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(u64::from(filtered_data_table.total()), 1u64);
        assert_eq!(filtered_data_table.items().len(), constants_usize::ONE);
        assert!(
            filtered_data_table
                .items()
                .first()
                .expect(
                    "753fa97c postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold"
                )
                .values()
                .iter()
                .any(|value| value.as_ref() == constants_str::catalog::ADMIN_ALT)
        );
        let empty_data_table_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_2C93E406),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
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
            serde_json::from_slice::<
                    server_admin_contract::admin_data_table_view::AdminDataTableView,
                >(&body)
                .expect(
                    "aa8376d3 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold",
                )
        })
        .expect("a98d6360 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(u64::from(empty_data_table.total()), constants_u64::ZERO);
        assert!(empty_data_table.items().is_empty());
        let unsupported_filter_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_946CA218),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
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
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_5E6D79D4),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
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
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_sign_out_route::AdminSignOutRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect("ef71e50a postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(sign_out_response.status(), http::StatusCode::NO_CONTENT);
        let revoked_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_me_route::AdminMeRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect("54b9dc03 postgresql_auth_rbac_csrf_session_and_audit_flow invariant must hold");
        assert_eq!(revoked_response.status(), http::StatusCode::UNAUTHORIZED);
        let audit_outcomes = sqlx::query_as::<_, (bool, i64)>(constants_str::catalog::SELECT_SUCCEEDED_COUNT_ASTERISK_FROM_ADMIN_AUDIT_LOG_GROUP_BY_SUCCEEDED_ORDER)
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
}
mod html {
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn postgresql_html_users_crud_covers_every_frontend_field_separately() {
        let fixture = crate::admin_html_test_fixture().await;
        assert!(fixture.cookie.0.contains(fixture.csrf.0.as_str()));
        let login = constants_str::test_fixtures::VALUE_2562E0C2;
        let updated_login = constants_str::test_fixtures::VALUE_A582339C;
        let display_name = constants_str::test_fixtures::VALUE_79B22AC4;
        let updated_display_name = constants_str::test_fixtures::VALUE_8AE21450;
        let password = constants_str::test_fixtures::VALUE_4EDBB68D;
        let updated_password = constants_str::test_fixtures::VALUE_B6F4A0C4;
        let create_body = super::AdminHtmlTestFormBody::try_from(format!(
        "login={login}&display_name=HTML+CRUD+User&password={password}"
    ))
    .expect("801d9a43 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        let create_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserCreate.get(),
            ),
            super::StdAdminApiTestStrRef::from(create_body.0.as_str()),
        )
        .await;
        assert_eq!(create_response.status(), http::StatusCode::SEE_OTHER);
        let created = sqlx::query_as::<_, (i64, String, String, bool)>(
        constants_str::test_fixtures::VALUE_1B03D1AA,
    )
    .bind(login)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("5de4fc12 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        assert_eq!(created.1, login);
        assert_eq!(created.2, display_name);
        assert!(!created.3);
        let users_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(users_response.status(), http::StatusCode::OK);
        let users_html = crate::admin_html_body(users_response).await;
        crate::assert_admin_csr_shell(&users_html);

        let login_update_body = super::AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&login={updated_login}&display_name=HTML+CRUD+User",
        created.0
    ))
    .expect("b0714f29 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        let login_update_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserUpdate.get(),
            ),
            super::StdAdminApiTestStrRef::from(login_update_body.0.as_str()),
        )
        .await;
        assert_eq!(login_update_response.status(), http::StatusCode::SEE_OTHER);
        let login_update = sqlx::query_as::<_, (String, String)>(
        constants_str::test_fixtures::VALUE_56386809,
    )
    .bind(created.0)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("68fae270 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        assert_eq!(
            login_update,
            (updated_login.to_owned(), display_name.to_owned())
        );

        let display_update_body = super::AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&login={updated_login}&display_name=HTML+CRUD+User+Updated",
        created.0
    ))
    .expect("9a6eb324 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        let display_update_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserUpdate.get(),
            ),
            super::StdAdminApiTestStrRef::from(display_update_body.0.as_str()),
        )
        .await;
        assert_eq!(
            display_update_response.status(),
            http::StatusCode::SEE_OTHER
        );
        let display_update = sqlx::query_as::<_, (String, String)>(
        constants_str::test_fixtures::VALUE_56386809,
    )
    .bind(created.0)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("10df386a postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        assert_eq!(
            display_update,
            (updated_login.to_owned(), updated_display_name.to_owned())
        );

        let password_update_body = super::AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&password={updated_password}",
        created.0
    ))
    .expect("cd82f375 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        let password_update_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserPassword.get(),
            ),
            super::StdAdminApiTestStrRef::from(password_update_body.0.as_str()),
        )
        .await;
        assert_eq!(
            password_update_response.status(),
            http::StatusCode::SEE_OTHER
        );
        let old_sign_in_body =
        super::AdminHtmlTestFormBody::try_from(format!("login={updated_login}&password={password}"))
            .expect("8c42d7e1 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        let old_sign_in_response = tower::ServiceExt::oneshot(
        fixture.router.0.clone(),
        crate::html_request_with_peer(
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(server_admin_contract::admin_html_action::AdminHtmlAction::SignIn.get()),
            super::StdAdminApiTestStrRef::from(old_sign_in_body.0.as_str()),
            None,
        )
        .0,
    )
    .await
    .expect("26ab3584 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        assert_eq!(
            old_sign_in_response.status(),
            http::StatusCode::UNAUTHORIZED
        );
        let new_sign_in_body = super::AdminHtmlTestFormBody::try_from(format!(
        "login={updated_login}&password={updated_password}"
    ))
    .expect("ef05a691 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        let new_sign_in_response = tower::ServiceExt::oneshot(
        fixture.router.0.clone(),
        crate::html_request_with_peer(
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(server_admin_contract::admin_html_action::AdminHtmlAction::SignIn.get()),
            super::StdAdminApiTestStrRef::from(new_sign_in_body.0.as_str()),
            None,
        )
        .0,
    )
    .await
    .expect("b9306c2e postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        assert_eq!(new_sign_in_response.status(), http::StatusCode::SEE_OTHER);

        let role_id = sqlx::query_scalar::<_, i64>(constants_str::integration_fixtures::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("f1674ab9 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        let roles_update_body = super::AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&expected_role_ids=&role_{role_id}={role_id}",
        created.0
    ))
    .expect("410e6a8c postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        let roles_update_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserRoles.get(),
            ),
            super::StdAdminApiTestStrRef::from(roles_update_body.0.as_str()),
        )
        .await;
        assert_eq!(roles_update_response.status(), http::StatusCode::SEE_OTHER);
        let assigned_roles =
        sqlx::query_scalar::<_, i64>(constants_str::test_fixtures::VALUE_4616DD96)
            .bind(created.0)
            .fetch_all(&fixture.pool.0)
            .await
            .expect("739cb4f5 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        assert_eq!(assigned_roles, vec![role_id]);

        let ban_body = super::AdminHtmlTestFormBody::try_from(format!("user_id={}&is_banned=true", created.0))
        .expect("a17fdc64 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        let ban_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserBan.get(),
            ),
            super::StdAdminApiTestStrRef::from(ban_body.0.as_str()),
        )
        .await;
        assert_eq!(ban_response.status(), http::StatusCode::SEE_OTHER);
        let final_users_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        let final_users_html = crate::admin_html_body(final_users_response).await;
        crate::assert_admin_csr_shell(&final_users_html);
        let unban_body =
        super::AdminHtmlTestFormBody::try_from(format!("user_id={}&is_banned=false", created.0))
            .expect("9d304db3 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        let unban_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserBan.get(),
            ),
            super::StdAdminApiTestStrRef::from(unban_body.0.as_str()),
        )
        .await;
        assert_eq!(unban_response.status(), http::StatusCode::SEE_OTHER);
        let is_banned = sqlx::query_scalar::<_, bool>(constants_str::test_fixtures::VALUE_A65908E0)
        .bind(created.0)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("55208887 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        assert!(!is_banned);
        let roles_clear_body = super::AdminHtmlTestFormBody::try_from(format!(
        "user_id={}&expected_role_ids={role_id}",
        created.0
    ))
    .expect("04b638dc postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        let roles_clear_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserRoles.get(),
            ),
            super::StdAdminApiTestStrRef::from(roles_clear_body.0.as_str()),
        )
        .await;
        assert_eq!(roles_clear_response.status(), http::StatusCode::SEE_OTHER);

        let delete_body =
        super::AdminHtmlTestFormBody::try_from(format!("user_id={}&confirmation=true", created.0))
            .expect("d4fe3069 postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        let delete_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserDelete.get(),
            ),
            super::StdAdminApiTestStrRef::from(delete_body.0.as_str()),
        )
        .await;
        assert_eq!(delete_response.status(), http::StatusCode::SEE_OTHER);
        let deleted_count = sqlx::query_scalar::<_, i64>(constants_str::test_fixtures::VALUE_ED81ED3A)
        .bind(created.0)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("72c950ea postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
        assert_eq!(deleted_count, constants_i64::ZERO);
        let deleted_users_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        let deleted_users_html = crate::admin_html_body(deleted_users_response).await;
        crate::assert_admin_csr_shell(&deleted_users_html);
        fixture.lock.0.rollback().await.expect("93db561a postgresql_html_users_crud_covers_every_frontend_field_separately invariant must hold");
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn postgresql_html_roles_crud_covers_every_frontend_field_separately() {
        let fixture = crate::admin_html_test_fixture().await;
        let role_name = constants_str::test_fixtures::VALUE_B20522BC;
        let updated_role_name = constants_str::test_fixtures::VALUE_C940BA4C;
        let create_body =
        super::AdminHtmlTestFormBody::try_from(format!("name={role_name}")).expect("c593e840 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
        let create_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::RoleCreate.get(),
            ),
            super::StdAdminApiTestStrRef::from(create_body.0.as_str()),
        )
        .await;
        assert_eq!(create_response.status(), http::StatusCode::SEE_OTHER);
        let created = sqlx::query_as::<_, (i64, String, bool)>(
        constants_str::test_fixtures::VALUE_96DFAB96,
    )
    .bind(role_name)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("196fbd27 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
        assert_eq!(created.1, role_name);
        assert!(!created.2);
        let roles_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(roles_response.status(), http::StatusCode::OK);
        let roles_html = crate::admin_html_body(roles_response).await;
        crate::assert_admin_csr_shell(&roles_html);

        let update_body =
        super::AdminHtmlTestFormBody::try_from(format!("role_id={}&name={updated_role_name}", created.0))
            .expect("7ea84503 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
        let update_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::RoleUpdate.get(),
            ),
            super::StdAdminApiTestStrRef::from(update_body.0.as_str()),
        )
        .await;
        assert_eq!(update_response.status(), http::StatusCode::SEE_OTHER);
        let updated = sqlx::query_scalar::<_, String>(constants_str::test_fixtures::VALUE_59A3D59A)
        .bind(created.0)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("43f81d69 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
        assert_eq!(updated, updated_role_name);

        let permission =
        sqlx::query_as::<_, (i64, String)>(constants_str::test_fixtures::VALUE_F3C2734E)
            .fetch_one(&fixture.pool.0)
            .await
            .expect("ba920f54 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
        let permissions_body = super::AdminHtmlTestFormBody::try_from(format!(
        "role_id={}&expected_permission_ids=&permission_{}={}",
        created.0, permission.0, permission.0
    ))
    .expect("0d476c31 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
        let permissions_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::RolePermissions.get(),
            ),
            super::StdAdminApiTestStrRef::from(permissions_body.0.as_str()),
        )
        .await;
        assert_eq!(permissions_response.status(), http::StatusCode::SEE_OTHER);
        let assigned_permissions = sqlx::query_scalar::<_, i64>(
        constants_str::test_fixtures::VALUE_5FE3480D,
    )
    .bind(created.0)
    .fetch_all(&fixture.pool.0)
    .await
    .expect("82b0d9f3 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
        assert_eq!(assigned_permissions, vec![permission.0]);
        let final_roles_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        let final_roles_html = crate::admin_html_body(final_roles_response).await;
        crate::assert_admin_csr_shell(&final_roles_html);

        let delete_body =
        super::AdminHtmlTestFormBody::try_from(format!("role_id={}&confirmation=true", created.0))
            .expect("e1547a60 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
        let delete_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::RoleDelete.get(),
            ),
            super::StdAdminApiTestStrRef::from(delete_body.0.as_str()),
        )
        .await;
        assert_eq!(delete_response.status(), http::StatusCode::SEE_OTHER);
        let deleted_count = sqlx::query_scalar::<_, i64>(constants_str::test_fixtures::VALUE_D4A7F1E9)
        .bind(created.0)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("2db479f8 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
        assert_eq!(deleted_count, constants_i64::ZERO);
        let deleted_roles_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        let deleted_roles_html = crate::admin_html_body(deleted_roles_response).await;
        crate::assert_admin_csr_shell(&deleted_roles_html);
        fixture.lock.0.rollback().await.expect("674dc2a9 postgresql_html_roles_crud_covers_every_frontend_field_separately invariant must hold");
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn postgresql_html_settings_updates_and_reads_every_field_separately() {
        let fixture = crate::admin_html_test_fixture().await;
        let site_name_a =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_98A13EB2);
        let site_name_b =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_ABCC7908);
        let route_a = super::StdAdminApiTestStrRef::from(
            server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
        );
        let route_b =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_DB2C56E6);
        let tab_title_a =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_F7D2459A);
        let tab_title_b =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_74AF8A89);
        let main_logo_a =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_2C8B94AD);
        let main_logo_b =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_91EAC748);
        let primary_color_a =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_CD527CD2);
        let primary_color_b =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_3CFDA7DC);
        let organization_name_a =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_DA7C4DC3);
        let organization_name_b =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_4918294B);
        let organization_contacts_a =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_2AFAD82D);
        let organization_contacts_b =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_E7FDD028);
        let support_url_a =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_AB22006C);
        let support_url_b =
            super::StdAdminApiTestStrRef::from(constants_str::test_fixtures::VALUE_4D525EFD);
        let states = [
            super::AdminHtmlSettingsTestValues {
                default_admin_route: route_a,
                main_logo: main_logo_a,
                organization_contacts: organization_contacts_a,
                organization_name: organization_name_a,
                primary_color: primary_color_a,
                site_name: site_name_a,
                support_url: support_url_a,
                tab_title: tab_title_a,
            },
            super::AdminHtmlSettingsTestValues {
                site_name: site_name_b,
                ..super::AdminHtmlSettingsTestValues {
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
            super::AdminHtmlSettingsTestValues {
                default_admin_route: route_b,
                main_logo: main_logo_a,
                organization_contacts: organization_contacts_a,
                organization_name: organization_name_a,
                primary_color: primary_color_a,
                site_name: site_name_b,
                support_url: support_url_a,
                tab_title: tab_title_a,
            },
            super::AdminHtmlSettingsTestValues {
                tab_title: tab_title_b,
                ..super::AdminHtmlSettingsTestValues {
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
            super::AdminHtmlSettingsTestValues {
                main_logo: main_logo_b,
                default_admin_route: route_b,
                organization_contacts: organization_contacts_a,
                organization_name: organization_name_a,
                primary_color: primary_color_a,
                site_name: site_name_b,
                support_url: support_url_a,
                tab_title: tab_title_b,
            },
            super::AdminHtmlSettingsTestValues {
                primary_color: primary_color_b,
                default_admin_route: route_b,
                main_logo: main_logo_b,
                organization_contacts: organization_contacts_a,
                organization_name: organization_name_a,
                site_name: site_name_b,
                support_url: support_url_a,
                tab_title: tab_title_b,
            },
            super::AdminHtmlSettingsTestValues {
                organization_name: organization_name_b,
                default_admin_route: route_b,
                main_logo: main_logo_b,
                organization_contacts: organization_contacts_a,
                primary_color: primary_color_b,
                site_name: site_name_b,
                support_url: support_url_a,
                tab_title: tab_title_b,
            },
            super::AdminHtmlSettingsTestValues {
                organization_contacts: organization_contacts_b,
                default_admin_route: route_b,
                main_logo: main_logo_b,
                organization_name: organization_name_b,
                primary_color: primary_color_b,
                site_name: site_name_b,
                support_url: support_url_a,
                tab_title: tab_title_b,
            },
            super::AdminHtmlSettingsTestValues {
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
            let update_response = crate::admin_html_response(
                fixture_ref,
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    server_admin_contract::admin_html_action::AdminHtmlAction::SettingsUpdate.get(),
                ),
                super::StdAdminApiTestStrRef::from(form_body.0.as_str()),
            )
            .await;
            assert_eq!(update_response.status(), http::StatusCode::SEE_OTHER);
            let read_response = crate::admin_html_response(
                fixture_ref,
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(
                    server_admin_contract::admin_frontend_path::AdminFrontendPath::Settings.get(),
                ),
                super::StdAdminApiTestStrRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                ),
            )
            .await;
            assert_eq!(read_response.status(), http::StatusCode::OK);
            let read_html = crate::admin_html_body(read_response).await;
            crate::assert_admin_csr_shell(&read_html);
        })
        .await;
        let stored = sqlx::query_as::<
        _,
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    >(
        constants_str::test_fixtures::VALUE_F1866337,
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("a8f201de postgresql_html_settings_updates_and_reads_every_field_separately invariant must hold");
        assert_eq!(stored.0, site_name_b.0);
        assert_eq!(stored.1, route_b.0);
        assert_eq!(stored.2, tab_title_b.0);
        assert_eq!(stored.3, main_logo_b.0);
        assert_eq!(stored.4, primary_color_b.0);
        assert_eq!(stored.5, organization_name_b.0);
        assert_eq!(stored.6, organization_contacts_b.0);
        assert_eq!(stored.7, support_url_b.0);
        let empty =
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX);
        let clear_states = [
            (
                super::AdminHtmlSettingsTestValues {
                    default_admin_route: route_b,
                    main_logo: main_logo_b,
                    organization_contacts: organization_contacts_b,
                    organization_name: organization_name_b,
                    primary_color: primary_color_b,
                    site_name: site_name_b,
                    support_url: support_url_b,
                    tab_title: empty,
                },
                constants_usize::ONE,
            ),
            (
                super::AdminHtmlSettingsTestValues {
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
                super::AdminHtmlSettingsTestValues {
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
                super::AdminHtmlSettingsTestValues {
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
                super::AdminHtmlSettingsTestValues {
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
                super::AdminHtmlSettingsTestValues {
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
            let clear_response = crate::admin_html_response(
                fixture_ref,
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    server_admin_contract::admin_html_action::AdminHtmlAction::SettingsUpdate.get(),
                ),
                super::StdAdminApiTestStrRef::from(form_body.0.as_str()),
            )
            .await;
            assert_eq!(clear_response.status(), http::StatusCode::SEE_OTHER);
            let optional_values = sqlx::query_as::<
                _,
                (
                    String,
                    String,
                    String,
                    String,
                    String,
                    String,
                ),
            >(
                constants_str::test_fixtures::VALUE_8CB85C2C,
            )
            .fetch_one(&fixture_ref.pool.0)
            .await
            .expect("d418f9c0 postgresql_html_settings_updates_and_reads_every_field_separately invariant must hold");
            assert_eq!(
                [
                    (
                        optional_values.0.as_str(),
                        constants_str::catalog::ADMIN,
                    ),
                    (
                        optional_values.1.as_str(),
                        constants_str::catalog::ADMIN_DEFAULT_MAIN_LOGO,
                    ),
                    (
                        optional_values.2.as_str(),
                        constants_str::catalog::PRIMARY_COLOR_DEFAULT,
                    ),
                    (
                        optional_values.3.as_str(),
                        constants_str::catalog::ADMIN,
                    ),
                    (
                        optional_values.4.as_str(),
                        constants_str::catalog::ADMIN_DEFAULT_ORGANIZATION_CONTACTS,
                    ),
                    (
                        optional_values.5.as_str(),
                        constants_str::catalog::ADMIN_DEFAULT_SUPPORT_URL,
                    ),
                ]
                .iter()
                .filter(|(value, default)| value == default)
                .count(),
                expected_cleared,
            );
        },
    )
    .await;
        fixture.lock.0.rollback().await.expect("c7659b40 postgresql_html_settings_updates_and_reads_every_field_separately invariant must hold");
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn postgresql_initial_administrator_password_must_change_before_admin_access() {
        let fixture = crate::admin_html_test_fixture_with_password_change(
            server_admin_contract::admin_bool::AdminBool::from(true),
        )
        .await;
        let users_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(users_response.status(), http::StatusCode::SEE_OTHER);
        assert_eq!(
            users_response.headers().get(http::header::LOCATION),
            Some(&http::HeaderValue::from_static(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Profile.get(),
            ))
        );
        let profile_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Profile.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(profile_response.status(), http::StatusCode::OK);
        let correct_password =
        serde_json::from_str::<String>(constants_str::catalog::CORRECT_PASSWORD).expect("e20a72a8 postgresql_initial_administrator_password_must_change_before_admin_access invariant must hold");
        let change_password_body = super::AdminHtmlTestFormBody::try_from(format!(
        "current_password={correct_password}&new_password=Initial-administrator-changed-pass2",
    ))
    .expect("b42a390d postgresql_initial_administrator_password_must_change_before_admin_access invariant must hold");
        let change_password_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::ProfilePassword.get(),
            ),
            super::StdAdminApiTestStrRef::from(change_password_body.0.as_str()),
        )
        .await;
        assert_eq!(
            change_password_response.status(),
            http::StatusCode::SEE_OTHER
        );
        let password_change_required = sqlx::query_scalar::<_, bool>(
        constants_str::integration_fixtures::SELECT_MUST_CHANGE_PASSWORD_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("ea57fc2d postgresql_initial_administrator_password_must_change_before_admin_access invariant must hold");
        assert!(!password_change_required);
        let post_change_users_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(post_change_users_response.status(), http::StatusCode::OK);
        fixture.lock.0.rollback().await.expect("6a8ce0f3 postgresql_initial_administrator_password_must_change_before_admin_access invariant must hold");
    }

    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn postgresql_html_profile_reads_every_field_and_changes_own_password() {
        let fixture = crate::admin_html_test_fixture().await;
        let profile_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Profile.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(profile_response.status(), http::StatusCode::OK);
        let profile_html = crate::admin_html_body(profile_response).await;
        crate::assert_admin_csr_shell(&profile_html);

        let original_password_hash = sqlx::query_scalar::<_, String>(
        constants_str::catalog::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("c09b5e4e postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
        let (current_session_id, user_id) = sqlx::query_as::<_, (uuid::Uuid, i64)>(
        constants_str::test_fixtures::VALUE_9605FF41,
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("ae46b7c1 postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
        let other_session_id = uuid::Uuid::from_u128(2u128);
        let _inserted_other_session = sqlx::query(
        constants_str::test_fixtures::VALUE_324717BB,
    )
    .bind(other_session_id)
    .bind(user_id)
    .execute(&fixture.pool.0)
    .await
    .expect("3e216ecd postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
        let _inserted_other_refresh_token = sqlx::query(
        constants_str::test_fixtures::VALUE_0FCC992D,
    )
    .bind(uuid::Uuid::from_u128(3u128))
    .bind(user_id)
    .execute(&fixture.pool.0)
    .await
    .expect("d61fc342 postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
        let correct_password =
        serde_json::from_str::<String>(constants_str::catalog::CORRECT_PASSWORD).expect("c59b011a postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
        let change_password_body = super::AdminHtmlTestFormBody::try_from(format!(
        "current_password={correct_password}&new_password=Html-profile-pass2",
    ))
    .expect("c93d69e3 postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
        let change_password_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::ProfilePassword.get(),
            ),
            super::StdAdminApiTestStrRef::from(change_password_body.0.as_str()),
        )
        .await;
        assert_eq!(
            change_password_response.status(),
            http::StatusCode::SEE_OTHER
        );
        let changed_password_hash = sqlx::query_scalar::<_, String>(
        constants_str::catalog::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("696330ca postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
        assert_ne!(changed_password_hash, original_password_hash);
        let current_session_revoked = sqlx::query_scalar::<_, bool>(
        constants_str::test_fixtures::VALUE_26E35E53,
    )
    .bind(current_session_id)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("38923e84 postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
        assert!(!current_session_revoked);
        let other_session_revoked = sqlx::query_scalar::<_, bool>(
        constants_str::test_fixtures::VALUE_26E35E53,
    )
    .bind(other_session_id)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("f0168dc5 postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
        assert!(other_session_revoked);
        let active_refresh_token_count = sqlx::query_scalar::<_, i64>(
        constants_str::test_fixtures::VALUE_52BB5B18,
    )
    .fetch_one(&fixture.pool.0)
    .await
    .expect("740d6dc9 postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
        assert_eq!(active_refresh_token_count, constants_i64::ZERO);
        let authenticated_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Profile.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(authenticated_response.status(), http::StatusCode::OK);
        fixture.lock.0.rollback().await.expect("737bbbe6 postgresql_html_profile_reads_every_field_and_changes_own_password invariant must hold");
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn postgresql_html_sessions_reads_every_field_and_revokes_session() {
        let fixture = crate::admin_html_test_fixture().await;
        let admin_id =
        sqlx::query_scalar::<_, i64>(constants_str::catalog::SELECT_ID_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN)
            .fetch_one(&fixture.pool.0)
            .await
            .expect("7f0a7c64 postgresql_html_sessions_reads_every_field_and_revokes_session invariant must hold");
        let (session_id, _created_at, _expires_at) = sqlx::query_as::<_, (uuid::Uuid, String, String)>(
        constants_str::integration_fixtures::SERVER_ADMIN_LIST_ACTIVE_SESSIONS_SQL,
    )
    .bind(admin_id)
    .bind(100i64)
    .bind(constants_i64::ZERO)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("32e44a86 postgresql_html_sessions_reads_every_field_and_revokes_session invariant must hold");
        let sessions_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Sessions.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(sessions_response.status(), http::StatusCode::OK);
        let sessions_html = crate::admin_html_body(sessions_response).await;
        crate::assert_admin_csr_shell(&sessions_html);

        let revoke_body =
        super::AdminHtmlTestFormBody::try_from(format!("session_id={session_id}&confirmation=true"))
            .expect("2f8bea59 postgresql_html_sessions_reads_every_field_and_revokes_session invariant must hold");
        let revoke_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::SessionRevoke.get(),
            ),
            super::StdAdminApiTestStrRef::from(revoke_body.0.as_str()),
        )
        .await;
        assert_eq!(revoke_response.status(), http::StatusCode::SEE_OTHER);
        let revoked = sqlx::query_scalar::<_, bool>(
        constants_str::test_fixtures::VALUE_26E35E53,
    )
    .bind(session_id)
    .fetch_one(&fixture.pool.0)
    .await
    .expect("e443902e postgresql_html_sessions_reads_every_field_and_revokes_session invariant must hold");
        assert!(revoked);
        let rejected_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Sessions.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(rejected_response.status(), http::StatusCode::SEE_OTHER);
        fixture.lock.0.rollback().await.expect("9f41b8bd postgresql_html_sessions_reads_every_field_and_revokes_session invariant must hold");
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn postgresql_html_router_registers_every_owned_page_and_action() {
        let fixture = crate::admin_html_test_fixture().await;
        let fixture_ref = &fixture;
        futures::StreamExt::fold(
            futures::StreamExt::filter(
                futures::stream::iter(
                    server_admin_contract::admin_frontend_path::AdminFrontendPath::all_pages(),
                ),
                |path| {
                    std::future::ready(!matches!(
                        path,
                        server_admin_contract::admin_frontend_path::AdminFrontendPath::Metrics
                            | server_admin_contract::admin_frontend_path::AdminFrontendPath::Permissions
                            | server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles
                            | server_admin_contract::admin_frontend_path::AdminFrontendPath::Tables
                            | server_admin_contract::admin_frontend_path::AdminFrontendPath::Users
                    ))
                },
            ),
            (),
            async |(), path| {
                let response = crate::admin_html_response(
                    fixture_ref,
                    super::HttpAdminApiTestMethod::from(http::Method::GET),
                    super::StdAdminApiTestStrRef::from(path.get()),
                    super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
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
                    server_admin_contract::admin_frontend_path::AdminFrontendPath::Profile
                        | server_admin_contract::admin_frontend_path::AdminFrontendPath::Sessions
                        | server_admin_contract::admin_frontend_path::AdminFrontendPath::Settings
                ) {
                    let html = crate::admin_html_body(response).await;
                    crate::assert_admin_csr_shell(&html);
                }
            },
        )
        .await;
        futures::StreamExt::fold(
            futures::stream::iter(server_admin_contract::admin_data_table::AdminDataTable::ALL),
            (),
            async |(), table| {
                let uri = table.frontend_path();
                let response = crate::admin_html_response(
                    fixture_ref,
                    super::HttpAdminApiTestMethod::from(http::Method::GET),
                    super::StdAdminApiTestStrRef::from(uri.as_ref()),
                    super::StdAdminApiTestStrRef::from(
                        constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                    ),
                )
                .await;
                assert_eq!(
                    response.status(),
                    http::StatusCode::OK,
                    "table view {table} failed"
                );
                let html = crate::admin_html_body(response).await;
                crate::assert_admin_csr_shell(&html);
            },
        )
        .await;
        futures::StreamExt::fold(
        futures::stream::iter(server_admin_contract::admin_html_action::AdminHtmlAction::ALL),
        (),
        async |(), action| {
            let response = tower::ServiceExt::oneshot(
                fixture_ref.router.0.clone(),
                crate::html_request_with_peer(
                    super::HttpAdminApiTestMethod::from(http::Method::POST),
                    super::StdAdminApiTestStrRef::from(action.get()),
                    super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
                    None,
                )
                .0,
            )
            .await
            .expect("d9567273 postgresql_html_router_registers_every_owned_page_and_action invariant must hold");
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
        fixture.lock.0.rollback().await.expect(
        "c0c53cdc postgresql_html_router_registers_every_owned_page_and_action invariant must hold",
    );
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering() {
        let fixture = crate::admin_html_test_fixture().await;
        let unauthenticated_response = tower::ServiceExt::oneshot(
        fixture.router.0.clone(),
        crate::html_request_with_peer(
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get()),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
            None,
        )
        .0,
    )
    .await
    .expect("184ec7b2 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        assert_eq!(
            unauthenticated_response.status(),
            http::StatusCode::SEE_OTHER
        );
        assert_eq!(
            unauthenticated_response
                .headers()
                .get(http::header::LOCATION),
            Some(&http::HeaderValue::from_static(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::SignIn.get(),
            )),
        );

        let login = constants_str::test_fixtures::VALUE_0E3DA187;
        let valid_body = super::AdminHtmlTestFormBody::try_from(format!(
        "login={login}&display_name=HTML+Form+Contract+User&password=Html-form-pass1"
    ))
    .expect("94b36ec1 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        let missing_csrf_response = tower::ServiceExt::oneshot(
        fixture.router.0.clone(),
        crate::html_request_with_peer(
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(server_admin_contract::admin_html_action::AdminHtmlAction::UserCreate.get()),
            super::StdAdminApiTestStrRef::from(valid_body.0.as_str()),
            None,
        )
        .0,
    )
    .await
    .expect("e6013d7a postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        assert_eq!(missing_csrf_response.status(), http::StatusCode::FORBIDDEN);
        let unknown_field_body =
        super::AdminHtmlTestFormBody::try_from(format!("{}&unknown_field=true", valid_body.0))
            .expect("af2948d3 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        let unknown_field_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserCreate.get(),
            ),
            super::StdAdminApiTestStrRef::from(unknown_field_body.0.as_str()),
        )
        .await;
        assert_eq!(
            unknown_field_response.status(),
            http::StatusCode::UNPROCESSABLE_ENTITY
        );
        let create_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserCreate.get(),
            ),
            super::StdAdminApiTestStrRef::from(valid_body.0.as_str()),
        )
        .await;
        assert_eq!(create_response.status(), http::StatusCode::SEE_OTHER);
        let duplicate_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserCreate.get(),
            ),
            super::StdAdminApiTestStrRef::from(valid_body.0.as_str()),
        )
        .await;
        assert_eq!(duplicate_response.status(), http::StatusCode::CONFLICT);
        let created_id = sqlx::query_scalar::<_, i64>(constants_str::test_fixtures::VALUE_A2A63B95)
        .bind(login)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("378a4e50 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        let filtered_path = super::AdminHtmlTestFormBody::try_from(format!(
        "{}?search={login}",
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get()
    ))
    .expect("60bf2c91 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        let filtered_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(filtered_path.0.as_str()),
            super::StdAdminApiTestStrRef::from(constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(filtered_response.status(), http::StatusCode::OK);
        let filtered_html = crate::admin_html_body(filtered_response).await;
        crate::assert_admin_csr_shell(&filtered_html);

        let role_id = sqlx::query_scalar::<_, i64>(constants_str::integration_fixtures::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("bc10a764 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        let stale_roles_body = super::AdminHtmlTestFormBody::try_from(format!(
        "user_id={created_id}&expected_role_ids={role_id}"
    ))
    .expect("1934ad6f postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        let stale_roles_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserRoles.get(),
            ),
            super::StdAdminApiTestStrRef::from(stale_roles_body.0.as_str()),
        )
        .await;
        assert_eq!(stale_roles_response.status(), http::StatusCode::CONFLICT);

        let role_name = constants_str::test_fixtures::VALUE_F9B1D97F;
        let create_role_body =
        super::AdminHtmlTestFormBody::try_from(format!("name={role_name}")).expect("8cf4260d postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        let create_role_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::RoleCreate.get(),
            ),
            super::StdAdminApiTestStrRef::from(create_role_body.0.as_str()),
        )
        .await;
        assert_eq!(create_role_response.status(), http::StatusCode::SEE_OTHER);
        let duplicate_role_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::RoleCreate.get(),
            ),
            super::StdAdminApiTestStrRef::from(create_role_body.0.as_str()),
        )
        .await;
        assert_eq!(duplicate_role_response.status(), http::StatusCode::CONFLICT);
        let created_role_id = sqlx::query_scalar::<_, i64>(constants_str::test_fixtures::VALUE_44E1D290)
        .bind(role_name)
        .fetch_one(&fixture.pool.0)
        .await
        .expect("2643be19 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        let permission_id =
        sqlx::query_scalar::<_, i64>(constants_str::test_fixtures::VALUE_1491D3FA)
            .fetch_one(&fixture.pool.0)
            .await
            .expect("d8134c5b postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        let stale_permissions_body = super::AdminHtmlTestFormBody::try_from(format!(
        "role_id={created_role_id}&expected_permission_ids={permission_id}"
    ))
    .expect("49fac702 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        let stale_permissions_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::RolePermissions.get(),
            ),
            super::StdAdminApiTestStrRef::from(stale_permissions_body.0.as_str()),
        )
        .await;
        assert_eq!(
            stale_permissions_response.status(),
            http::StatusCode::CONFLICT
        );
        let delete_role_body =
        super::AdminHtmlTestFormBody::try_from(format!("role_id={created_role_id}&confirmation=true"))
            .expect("f1c637d8 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        let delete_role_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::RoleDelete.get(),
            ),
            super::StdAdminApiTestStrRef::from(delete_role_body.0.as_str()),
        )
        .await;
        assert_eq!(delete_role_response.status(), http::StatusCode::SEE_OTHER);

        let unknown_delete_body = super::AdminHtmlTestFormBody::try_from(String::from(
        constants_str::test_fixtures::VALUE_8F942A25,
    ))
    .expect("d96b20e4 postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        let unknown_delete_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserDelete.get(),
            ),
            super::StdAdminApiTestStrRef::from(unknown_delete_body.0.as_str()),
        )
        .await;
        assert_eq!(unknown_delete_response.status(), http::StatusCode::CONFLICT);

        let delete_body =
        super::AdminHtmlTestFormBody::try_from(format!("user_id={created_id}&confirmation=true"))
            .expect("4cf9072d postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
        let delete_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::UserDelete.get(),
            ),
            super::StdAdminApiTestStrRef::from(delete_body.0.as_str()),
        )
        .await;
        assert_eq!(delete_response.status(), http::StatusCode::SEE_OTHER);
        fixture.lock.0.rollback().await.expect("7361eb5c postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering invariant must hold");
    }
}
mod maintenance {
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn postgresql_optimistic_revision_allows_one_concurrent_writer() {
        let database_url = std::env::var(constants_str::catalog::ENV_NAMES_DATABASE_URL).expect(
        "63a09eec postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold",
    );
        let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4u32)
        .connect(database_url.as_str())
        .await
        .expect("2480f8c4 postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold");
        let _drop_before = sqlx::query(
        constants_str::catalog::DROP_TABLE_IF_EXISTS_PG_TABLE_OPTIMISTIC_REVISION_TEST,
    )
    .execute(&pool)
    .await
    .expect(
        "e5e1f7cb postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold",
    );
        let _create = sqlx::query(constants_str::catalog::CREATE_TABLE_PG_TABLE_OPTIMISTIC_REVISION_TEST_ID_BIGINT_PRIMARY_KEY_REVISION)
        .execute(&pool)
        .await
        .expect("a75bc224 postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold");
        let _insert = sqlx::query(
        constants_str::catalog::INSERT_INTO_PG_TABLE_OPTIMISTIC_REVISION_TEST_ID_REVISION_VALUE_VALUES_1,
    )
    .execute(&pool)
    .await
    .expect(
        "da271038 postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold",
    );
        let update = constants_str::catalog::UPDATE_PG_TABLE_OPTIMISTIC_REVISION_TEST_SET_VALUE_DOLLAR_1_REVISION_REVISION;
        let (left, right) = tokio::join!(
        sqlx::query_scalar::<_, i64>(update)
            .bind(constants_i64::ONE)
            .bind(
                pg_table::pg_table_revision::PgTableRevision::try_from(constants_str::catalog::VALUE_0.to_owned())
                    .expect("979fa4b2 postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold")
            )
            .fetch_optional(&pool),
        sqlx::query_scalar::<_, i64>(update)
            .bind(2i64)
            .bind(
                pg_table::pg_table_revision::PgTableRevision::try_from(constants_str::catalog::VALUE_0.to_owned())
                    .expect("589ea31d postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold")
            )
            .fetch_optional(&pool),
    );
        let outcomes = [left.expect("a1a1382a postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold"), right.expect("8406b933 postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold")];
        assert_eq!(
            outcomes.iter().filter(|value| value.is_some()).count(),
            constants_usize::ONE
        );
        assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT revision FROM pg_table_optimistic_revision_test WHERE id=1",
        )
        .fetch_one(&pool)
        .await
        .expect("c0f01a04 postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold"),
        constants_i64::ONE
    );
        let stale = sqlx::query_scalar::<_, i64>(update)
        .bind(3i64)
        .bind(
            pg_table::pg_table_revision::PgTableRevision::try_from(constants_str::catalog::VALUE_0.to_owned())
                .expect("a3a08aeb postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold"),
        )
        .fetch_optional(&pool)
        .await
        .expect("964e3ef4 postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold");
        assert_eq!(stale, None);
        let _drop_after = sqlx::query(constants_str::catalog::DROP_TABLE_PG_TABLE_OPTIMISTIC_REVISION_TEST)
        .execute(&pool)
        .await
        .expect("a4d77f54 postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold");
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn postgresql_cleanup_is_batched_and_preserves_append_only_policy() {
        let database_url = std::env::var(constants_str::catalog::ENV_NAMES_DATABASE_URL).expect("7316cf4d postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3u32)
        .connect(database_url.as_str())
        .await
        .expect("f6a51733 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        let mut admin_db_test_lock = pool.begin().await.expect("847caf57 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        let _locked = sqlx::query(constants_str::integration_fixtures::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
        .execute(&mut *admin_db_test_lock)
        .await
        .expect("8c298fef postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        let mut idempotency_test_isolation = pool.begin().await.expect("f56c4c85 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        pg_crud_common::lock_pg_relation_resources::lock_pg_relation_resources(
        pg_crud_common::sqlx_pg_relation_lock_connection_ref::SqlxPgRelationLockConnectionRef::from(&mut *idempotency_test_isolation),
        &pg_crud_common::pg_relation_lock_namespace::PgRelationLockNamespace::try_from(constants_str::catalog::ACTOR_ATOMIC.to_owned())
            .expect("861fe23d postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold"),
        &pg_crud_common::pg_relation_resource_ids::PgRelationResourceIds::try_from(vec![
            pg_crud_common::pg_relation_resource_id::PgRelationResourceId::from(constants_i64::ONE),
        ])
        .expect("a18f804c postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold"),
    )
    .await
    .expect("fab61374 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        server_admin::prepare_postgresql::prepare_postgresql(app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool))
        .await
        .expect("029cb682 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        pg_table::ensure_pg_table_idempotency_schema::ensure_pg_table_idempotency_schema(app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool))
        .await
        .expect("eb08dffc postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        let _clear = sqlx::query(constants_str::integration_fixtures::TRUNCATE_ADMIN_ACCESS_SESSIONS_ADMIN_REFRESH_TOKENS_ADMIN_LOGIN_ATTEMPTS_ADMIN_RATE)
        .execute(&pool)
        .await
        .expect("e1b22572 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        let _attempts = sqlx::query(constants_str::catalog::INSERT_INTO_ADMIN_LOGIN_ATTEMPTS_LOGIN_SUCCEEDED_ATTEMPTED_AT_SELECT_OLD_VALUE)
        .execute(&pool)
        .await
        .expect("480b06eb postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        let _limits = sqlx::query(constants_str::catalog::INSERT_INTO_ADMIN_RATE_LIMITS_SCOPE_SUBJECT_WINDOW_STARTED_AT_REQUEST_COUNT_ALT)
        .execute(&pool)
        .await
        .expect("0375574d postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        let _audit = sqlx::query(
        constants_str::catalog::INSERT_INTO_ADMIN_AUDIT_LOG_ACTION_RESOURCE_SUCCEEDED_CREATED_AT_SELECT_TEST,
    )
    .execute(&pool)
    .await
    .expect("f50ef817 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        let retention =
        server_admin::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds::try_from(3_600i64).expect("ab892fc5 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        let config = server_admin::admin_cleanup_cfg::AdminCleanupCfg::new(
        server_admin::admin_cleanup_batch_size::AdminCleanupBatchSize::try_from(2i64).expect("1d97b31c postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold"),
        retention,
        retention,
        retention,
        retention,
        retention,
    );
        let report = server_admin::cleanup_admin_tables::cleanup_admin_tables(app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool), config)
        .await
        .expect("a422e8d4 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        assert_eq!(report.total_rows().to_string(), "6");
        let remaining = sqlx::query_as::<_, (i64, i64, i64)>(constants_str::catalog::SELECT_SELECT_COUNT_ASTERISK_FROM_ADMIN_LOGIN_ATTEMPTS_SELECT_COUNT_ASTERISK_FROM)
        .fetch_one(&pool)
        .await
        .expect("f37a3ab4 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
        assert_eq!(
            remaining,
            (constants_i64::ONE, constants_i64::ONE, constants_i64::ONE)
        );
        let ordinary_delete = sqlx::query(constants_str::catalog::DELETE_FROM_ADMIN_AUDIT_LOG)
            .execute(&pool)
            .await;
        assert!(matches!(ordinary_delete, Err(_error)));
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn postgresql_migration_creates_complete_schema() {
        let database_url = std::env::var(constants_str::catalog::ENV_NAMES_DATABASE_URL)
            .expect("b65d1786 postgresql_migration_creates_complete_schema invariant must hold");
        let base_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1u32)
            .connect(database_url.as_str())
            .await
            .expect("0047f74e postgresql_migration_creates_complete_schema invariant must hold");
        let _drop_schema = sqlx::raw_sql(
            constants_str::catalog::DROP_SCHEMA_IF_EXISTS_ADMIN_MIGRATION_FRESH_TEST_CASCADE,
        )
        .execute(&base_pool)
        .await
        .expect("df91b04d postgresql_migration_creates_complete_schema invariant must hold");
        let _create_schema = sqlx::raw_sql(
            constants_str::integration_fixtures::CREATE_SCHEMA_ADMIN_MIGRATION_FRESH_TEST,
        )
        .execute(&base_pool)
        .await
        .expect("02bcd1c2 postgresql_migration_creates_complete_schema invariant must hold");
        let connect = |schema: super::StdAdminApiTestStrRef<'static>| {
            let options = <sqlx::postgres::PgConnectOptions as std::str::FromStr>::from_str(
                database_url.as_str(),
            )
            .expect("aa7735db postgresql_migration_creates_complete_schema invariant must hold")
            .options([(constants_str::catalog::SEARCH_PATH, schema.0)]);
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(1u32)
                .connect_lazy_with(options)
        };
        let fresh_pool = connect(super::StdAdminApiTestStrRef::from(
            constants_str::catalog::ADMIN_MIGRATION_FRESH_TEST,
        ));
        let full = sqlx::migrate!("../server_admin_migrations");
        full.run(&fresh_pool)
            .await
            .expect("4b6c3bd6 postgresql_migration_creates_complete_schema invariant must hold");
        server_admin::validate_catalog_schema::validate_catalog_schema(
            pg_crud_common::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef::from(&fresh_pool),
            pg_crud_common::db_schema_name_ref::DbSchemaNameRef::from(
                constants_str::catalog::ADMIN_MIGRATION_FRESH_TEST,
            ),
        )
        .await
        .expect("fac299aa postgresql_migration_creates_complete_schema invariant must hold");
        let catalog_snapshot = pg_crud_common::inspect_postgres_catalog::inspect_postgres_catalog(
            pg_crud_common::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef::from(&fresh_pool),
            pg_crud_common::db_schema_name_ref::DbSchemaNameRef::from(
                constants_str::catalog::ADMIN_MIGRATION_FRESH_TEST,
            ),
        )
        .await
        .expect("518b93e4 postgresql_migration_creates_complete_schema invariant must hold");
        let fresh_pool_ref = &fresh_pool;
        let table_snapshots = futures::future::try_join_all(
            server_admin_contract::admin_data_table::AdminDataTable::PG_ORDER
                .into_iter()
                .map(async |table| {
                    pg_crud_common::inspect_postgres_table::inspect_postgres_table(
                        pg_crud_common::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef::from(
                            fresh_pool_ref,
                        ),
                        pg_crud_common::db_schema_name_ref::DbSchemaNameRef::from(
                            constants_str::catalog::ADMIN_MIGRATION_FRESH_TEST,
                        ),
                        pg_crud_common::db_table_name_ref::DbTableNameRef::from(
                            table.as_str().get(),
                        ),
                    )
                    .await
                    .map(|snapshot| (table, snapshot))
                }),
        )
        .await
        .expect("34d80f68 postgresql_migration_creates_complete_schema invariant must hold");
        let current_schema_snapshot = table_snapshots.into_iter().fold(
        format!(
            "# GENERATED FROM ORDERED SERVER ADMIN MIGRATIONS; DO NOT EDIT\n{catalog_snapshot:#?}\n"
        ),
        |mut output, (table, snapshot)| {
            output.push_str(format!("\n{table}\n{snapshot:#?}\n").as_str());
            output
        },
    );
        let current_schema_snapshot_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(constants_str::test_fixtures::ADMIN_CURRENT_SCHEMA_SNAPSHOT_PATH);
        if std::env::var_os(constants_str::test_fixtures::UPDATE_ADMIN_CURRENT_SCHEMA_SNAPSHOT)
            .is_some()
        {
            std::fs::write(
                current_schema_snapshot_path.as_path(),
                current_schema_snapshot.as_bytes(),
            )
            .expect("abe4d63f postgresql_migration_creates_complete_schema invariant must hold");
        }
        let expected_current_schema_snapshot = std::fs::read_to_string(
            current_schema_snapshot_path,
        )
        .expect("3af279e1 postgresql_migration_creates_complete_schema invariant must hold");
        assert_eq!(
            current_schema_snapshot, expected_current_schema_snapshot,
            "cb6ce4a9 migration-derived PostgreSQL schema snapshot changed"
        );
        let version = sqlx::query_scalar::<_, i64>(
            constants_str::catalog::SELECT_MAX_VERSION_FROM_ADMIN_MIGRATION_FRESH_TEST_SQLX_MIGRATIONS_WHERE,
        )
        .fetch_one(&base_pool)
        .await
        .expect("5c10c931 postgresql_migration_creates_complete_schema invariant must hold");
        assert_eq!(version, 13i64);
        let expected_tables = server_admin_contract::admin_data_table::AdminDataTable::PG_ORDER
            .map(|table| table.to_string())
            .into_iter()
            .collect::<std::collections::BTreeSet<String>>();
        let fresh_tables = sqlx::query_scalar::<_, String>(
            constants_str::catalog::SELECT_TABLE_NAME_FROM_INFORMATION_SCHEMA_TABLES_WHERE_TABLE_SCHEMA,
        )
        .bind(constants_str::catalog::ADMIN_MIGRATION_FRESH_TEST)
        .fetch_all(&base_pool)
        .await
        .expect("ab254ff4 postgresql_migration_creates_complete_schema invariant must hold")
        .into_iter()
        .collect::<std::collections::BTreeSet<String>>();
        assert_eq!(fresh_tables, expected_tables);
        fresh_pool.close().await;
        let _drop_after =
            sqlx::raw_sql(constants_str::catalog::DROP_SCHEMA_ADMIN_MIGRATION_FRESH_TEST_CASCADE)
                .execute(&base_pool)
                .await
                .expect(
                    "88dd90b8 postgresql_migration_creates_complete_schema invariant must hold",
                );
    }
}
mod policy {
    #[test]
    fn policy() {
        let read_excluded = <server_admin::admin_users::AdminUsers as pg_crud_common::db_table_schema::DbTableSchema>::read_excluded_columns();
        assert!(
            read_excluded
                .iter()
                .any(|field| field.as_ref() == constants_str::catalog::PASSWORD_HASH)
        );
        let create_excluded = <server_admin::admin_users::AdminUsers as pg_crud_common::db_table_schema::DbTableSchema>::create_excluded_columns();
        assert!(
            create_excluded
                .iter()
                .any(|field| field.as_ref() == constants_str::catalog::PASSWORD_HASH)
        );
    }
}
mod routing {
    #[tokio::test]
    async fn protected_routes_reject_missing_authentication_without_database_io() {
        let users_response = tower::ServiceExt::oneshot(
        crate::admin_api_test_router().0,
        http::Request::builder()
            .uri(
                frontend_contract::typed_route_path::typed_route_path::<server_admin_contract::admin_me_route::AdminMeRoute>()
                    .as_ref(),
            )
            .body(axum::body::Body::empty())
            .expect("b319e84d protected_routes_reject_missing_authentication_without_database_io invariant must hold"),
    )
    .await
    .expect("0ac617de protected_routes_reject_missing_authentication_without_database_io invariant must hold");
        assert_eq!(users_response.status(), http::StatusCode::UNAUTHORIZED);
        let response = tower::ServiceExt::oneshot(
        crate::admin_api_test_router().0,
        http::Request::builder()
            .uri(
                frontend_contract::typed_route_path::typed_route_path::<server_admin_contract::admin_list_users_route::AdminListUsersRoute>()
                    .as_ref(),
            )
            .body(axum::body::Body::empty())
            .expect("895e12fc protected_routes_reject_missing_authentication_without_database_io invariant must hold"),
    )
    .await
    .expect("1fe80ad3 protected_routes_reject_missing_authentication_without_database_io invariant must hold");
        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
    }
    #[tokio::test]
    #[allow(
        clippy::needless_for_each,
        reason = "repository policy requires iterator methods instead of for loops"
    )]
    async fn runtime_auth_router_contains_every_open_api_operation() {
        let document = serde_json::to_value(utoipa::openapi::OpenApi::from(
            server_admin::admin_api_open_api::admin_api_open_api(),
        ))
        .expect(
            "71599514 runtime_auth_router_contains_every_open_api_operation invariant must hold",
        );
        let paths = document
        .get(constants_str::catalog::PATHS)
        .and_then(serde_json::Value::as_object)
        .expect(
            "d908872f runtime_auth_router_contains_every_open_api_operation invariant must hold",
        );
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
                        constants_str::test_fixtures::ADMIN_SESSION_ID_PLACEHOLDER,
                        constants_str::catalog::VALUE_1,
                    )
                    .replace(
                        constants_str::test_fixtures::ADMIN_USER_ID_PLACEHOLDER,
                        constants_str::catalog::VALUE_1,
                    )
                    .replace(
                        constants_str::test_fixtures::ADMIN_ROLE_ID_PLACEHOLDER,
                        constants_str::catalog::VALUE_1,
                    );
                let method =
                    http::Method::from_bytes(documented_method.to_ascii_uppercase().as_bytes())
                        .expect("9d31a7e4 runtime_auth_router_contains_every_open_api_operation invariant must hold");
                async move {
                    (
                        documented_method,
                        documented_path,
                        tower::ServiceExt::oneshot(
                            crate::admin_api_test_router().0,
                            http::Request::builder()
                                .method(method)
                                .uri(runtime_path)
                                .body(axum::body::Body::empty())
                                .expect("a3d6fb65 runtime_auth_router_contains_every_open_api_operation invariant must hold"),
                        )
                        .await,
                    )
                }
            }),
    )
    .await;
        responses.into_iter().for_each(|(method, path, response)| {
        let status = response.expect("f7bd9f15 runtime_auth_router_contains_every_open_api_operation invariant must hold").status();
        assert!(
            status != http::StatusCode::METHOD_NOT_ALLOWED && status != http::StatusCode::NOT_FOUND,
            "runtime router does not expose documented operation {method} {path}"
        );
    });
    }
    #[tokio::test]
    async fn invalid_access_cookie_is_rejected_before_database_io() {
        let response = tower::ServiceExt::oneshot(
        crate::admin_api_test_router().0,
        http::Request::builder()
            .uri(
                frontend_contract::typed_route_path::typed_route_path::<
                    server_admin_contract::admin_me_route::AdminMeRoute,
                >()
                .as_ref(),
            )
            .header(
                http::header::COOKIE,
                constants_str::catalog::ADMIN_ACCESS_TOKEN_INVALID_JWT_TOKEN,
            )
            .body(axum::body::Body::empty())
            .expect(
                "819acd53 invalid_access_cookie_is_rejected_before_database_io invariant must hold",
            ),
    )
    .await
    .expect("c3af0891 invalid_access_cookie_is_rejected_before_database_io invariant must hold");
        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
    }
    #[tokio::test]
    async fn unknown_admin_api_route_is_not_captured_by_spa_fallback() {
        let response = tower::ServiceExt::oneshot(
        crate::admin_api_test_router().0,
        http::Request::builder()
            .uri(constants_str::catalog::NOT_AN_API_ROUTE)
            .body(axum::body::Body::empty())
            .expect("1ca76f8d unknown_admin_api_route_is_not_captured_by_spa_fallback invariant must hold"),
    )
    .await
    .expect("ce417390 unknown_admin_api_route_is_not_captured_by_spa_fallback invariant must hold");
        assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
    }
    #[tokio::test]
    async fn wrong_admin_http_method_uses_problem_details_contract() {
        let response = tower::ServiceExt::oneshot(
        crate::admin_api_test_router().0,
        http::Request::builder()
            .method(http::Method::GET)
            .uri(
                frontend_contract::typed_route_path::typed_route_path::<server_admin_contract::admin_sign_in_route::AdminSignInRoute>()
                    .as_ref(),
            )
            .body(axum::body::Body::empty())
            .expect("4eb1c098 wrong_admin_http_method_uses_problem_details_contract invariant must hold"),
    )
    .await
    .expect("6764152a wrong_admin_http_method_uses_problem_details_contract invariant must hold");
        assert_eq!(response.status(), http::StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(
            response.headers().get(http::header::CONTENT_TYPE),
            Some(&http::HeaderValue::from_static("application/problem+json")),
        );
    }
    #[tokio::test]
    async fn invalid_admin_json_uses_problem_details_and_body_limit_contract() {
        let malformed_response = tower::ServiceExt::oneshot(
        crate::admin_api_test_router().0,
        crate::request_with_peer(
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::typed_route_path::<server_admin_contract::admin_sign_in_route::AdminSignInRoute>()
                    .as_ref(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::integration_fixtures::LOGIN_ALT),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("5fb0627d invalid_admin_json_uses_problem_details_and_body_limit_contract invariant must hold");
        assert_eq!(
            malformed_response.status(),
            http::StatusCode::UNPROCESSABLE_ENTITY
        );
        assert_eq!(
            malformed_response.headers().get(http::header::CONTENT_TYPE),
            Some(&http::HeaderValue::from_static("application/problem+json")),
        );
        let body_limit = <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::body_limit()
        .expect("a60751db invalid_admin_json_uses_problem_details_and_body_limit_contract invariant must hold")
        .get();
        let oversized_password =
            constants_str::catalog::X.repeat(body_limit.saturating_add(constants_usize::ONE));
        let oversized_body = format!(r#"{{"login":"admin","password":"{oversized_password}"}}"#);
        let oversized_response = tower::ServiceExt::oneshot(
        crate::admin_api_test_router().0,
        crate::request_with_peer(
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(
                frontend_contract::typed_route_path::typed_route_path::<server_admin_contract::admin_sign_in_route::AdminSignInRoute>()
                    .as_ref(),
            ),
            super::StdAdminApiTestStrRef::from(oversized_body.as_str()),
            None,
            None,
        )
        .0,
    )
    .await
    .expect("fcd3dd3f invalid_admin_json_uses_problem_details_and_body_limit_contract invariant must hold");
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
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_sign_in_route::AdminSignInRoute,
                    >()
                    .as_ref(),
                )
                .header(
                    http::header::CONTENT_TYPE,
                    constants_str::catalog::APPLICATION_JSON,
                );
            if let Some(value) = origin {
                builder = builder.header(http::header::ORIGIN, value);
            }
            if let Some(value) = referer {
                builder = builder.header(http::header::REFERER, value);
            }
            let mut request = builder
            .body(axum::body::Body::from(
                constants_str::integration_fixtures::LOGIN_ADMIN_PASSWORD_PASSWORD,
            ))
            .expect(
                "168060a3 sign_in_requires_trusted_origin_without_database_io invariant must hold",
            );
            let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
            constants_str::catalog::VALUE_127_0_0_1_43210
                .parse::<std::net::SocketAddr>()
                .expect("c90cba14 sign_in_requires_trusted_origin_without_database_io invariant must hold"),
        ));
            request
        };
        let missing_origin_response = tower::ServiceExt::oneshot(
            crate::admin_api_test_router().0,
            make_request(None, None),
        )
        .await
        .expect("ed2f56fb sign_in_requires_trusted_origin_without_database_io invariant must hold");
        assert_eq!(
            missing_origin_response.status(),
            http::StatusCode::UNAUTHORIZED
        );
        let blocked_origin_response = tower::ServiceExt::oneshot(
            crate::admin_api_test_router().0,
            make_request(
                Some(constants_str::catalog::HTTP_BLOCKED_EXAMPLE),
                Some(constants_str::catalog::HTTP_LOCALHOST_ADMIN_SIGN_IN),
            ),
        )
        .await
        .expect("df43c793 sign_in_requires_trusted_origin_without_database_io invariant must hold");
        assert_eq!(
            blocked_origin_response.status(),
            http::StatusCode::UNAUTHORIZED
        );
    }
}
mod schema {
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn generated_admin_descriptors_match_applied_migrations() {
        let database_url = std::env::var(constants_str::catalog::ENV_NAMES_DATABASE_URL).expect(
            "7e62af41 generated_admin_descriptors_match_applied_migrations invariant must hold",
        );
        let pool = super::SqlxAdminApiTestPool::from(
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect(database_url.as_str())
            .await
            .expect(
                "20250c41 generated_admin_descriptors_match_applied_migrations invariant must hold",
            ),
    );
        let mut admin_db_test_lock = pool.0.begin().await.expect(
            "50eb5d64 generated_admin_descriptors_match_applied_migrations invariant must hold",
        );
        let _locked = sqlx::query(
            constants_str::integration_fixtures::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS,
        )
        .execute(&mut *admin_db_test_lock)
        .await
        .expect(
            "77883cf4 generated_admin_descriptors_match_applied_migrations invariant must hold",
        );
        server_admin::prepare_postgresql::prepare_postgresql(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
        )
        .await
        .expect(
            "9eceddf1 generated_admin_descriptors_match_applied_migrations invariant must hold",
        );
        server_admin::validate_catalog_schema::validate_catalog_schema(
            pg_crud_common::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef::from(&pool.0),
            pg_crud_common::db_schema_name_ref::DbSchemaNameRef::from(
                constants_str::catalog::PUBLIC,
            ),
        )
        .await
        .expect(
            "7a31cf02 generated_admin_descriptors_match_applied_migrations invariant must hold",
        );
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn admin_string_policies_match_postgresql_constraints() {
        let database_url = std::env::var(constants_str::catalog::ENV_NAMES_DATABASE_URL).expect(
            "93fcb3de admin_string_policies_match_postgresql_constraints invariant must hold",
        );
        let pool = super::SqlxAdminApiTestPool::from(
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect(database_url.as_str())
            .await
            .expect(
                "d48c868d admin_string_policies_match_postgresql_constraints invariant must hold",
            ),
    );
        let mut admin_db_test_lock = pool.0.begin().await.expect(
            "99ced936 admin_string_policies_match_postgresql_constraints invariant must hold",
        );
        let _locked = sqlx::query(
            constants_str::integration_fixtures::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS,
        )
        .execute(&mut *admin_db_test_lock)
        .await
        .expect("168b689c admin_string_policies_match_postgresql_constraints invariant must hold");
        server_admin::prepare_postgresql::prepare_postgresql(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
        )
        .await
        .expect("a453b862 admin_string_policies_match_postgresql_constraints invariant must hold");
        let valid_login = server_admin_contract::admin_login::AdminLogin::try_from(
            constants_str::test_fixtures::SSOT_LOGIN_VALID.to_owned(),
        )
        .is_ok();
        assert_eq!(
            server_admin_contract::admin_bool::AdminBool::from(valid_login),
            crate::postgres_accepts_admin_user_policy_values(
                &pool,
                super::StdAdminApiTestStrRef(constants_str::test_fixtures::SSOT_DISPLAY_NAME_VALID),
                super::StdAdminApiTestStrRef(constants_str::test_fixtures::SSOT_LOGIN_VALID),
            )
            .await
        );
        let invalid_login = server_admin_contract::admin_login::AdminLogin::try_from(
            constants_str::test_fixtures::SSOT_LOGIN_INVALID_CASE.to_owned(),
        )
        .is_ok();
        assert_eq!(
            server_admin_contract::admin_bool::AdminBool::from(invalid_login),
            crate::postgres_accepts_admin_user_policy_values(
                &pool,
                super::StdAdminApiTestStrRef(constants_str::test_fixtures::SSOT_DISPLAY_NAME_VALID),
                super::StdAdminApiTestStrRef(constants_str::test_fixtures::SSOT_LOGIN_INVALID_CASE),
            )
            .await
        );
        let invalid_display =
            server_admin_contract::admin_display_name::AdminDisplayName::try_from(
                constants_str::test_fixtures::SSOT_DISPLAY_NAME_PADDED.to_owned(),
            )
            .is_ok();
        assert_eq!(
            server_admin_contract::admin_bool::AdminBool::from(invalid_display),
            crate::postgres_accepts_admin_user_policy_values(
                &pool,
                super::StdAdminApiTestStrRef(
                    constants_str::test_fixtures::SSOT_DISPLAY_NAME_PADDED
                ),
                super::StdAdminApiTestStrRef(constants_str::test_fixtures::SSOT_LOGIN_VALID),
            )
            .await
        );
        let valid_role = server_admin_contract::admin_role_name::AdminRoleName::try_from(
            constants_str::test_fixtures::SSOT_ROLE_VALID.to_owned(),
        )
        .is_ok();
        assert_eq!(
            server_admin_contract::admin_bool::AdminBool::from(valid_role),
            crate::postgres_accepts_admin_role_policy_value(
                &pool,
                super::StdAdminApiTestStrRef(constants_str::test_fixtures::SSOT_ROLE_VALID),
            )
            .await
        );
        let invalid_role = server_admin_contract::admin_role_name::AdminRoleName::try_from(
            constants_str::test_fixtures::SSOT_ROLE_INVALID_CASE.to_owned(),
        )
        .is_ok();
        assert_eq!(
            server_admin_contract::admin_bool::AdminBool::from(invalid_role),
            crate::postgres_accepts_admin_role_policy_value(
                &pool,
                super::StdAdminApiTestStrRef(constants_str::test_fixtures::SSOT_ROLE_INVALID_CASE),
            )
            .await
        );
    }
}

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
    value: server_admin_contract::admin_role_id::AdminRoleId,
) -> server_admin_contract::admin_role_ids::AdminRoleIds {
    server_admin_contract::admin_role_ids::AdminRoleIds::try_from(vec![value])
        .expect("69bc51bc one_admin_role_id invariant must hold")
}
fn empty_admin_role_ids() -> server_admin_contract::admin_role_ids::AdminRoleIds {
    server_admin_contract::admin_role_ids::AdminRoleIds::try_from(Vec::new())
        .expect("d5ccd621 empty_admin_role_ids invariant must hold")
}
fn env<T>(value: StdAdminApiTestStrRef<'_>) -> T
where
    T: config_lib::try_from_std_env_var_ok::TryFromStdEnvVarOk,
    T::Error: std::fmt::Debug,
{
    T::try_from_std_env_var_ok(
        config_lib::std_env_var_ok::StdEnvVarOk::try_from(value.0.to_owned())
            .expect("92b71c4e env invariant must hold"),
    )
    .expect("afe20c19 env invariant must hold")
}
fn admin_api_test_router() -> AxumAdminApiTestRouter {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy(
            constants_str::catalog::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION,
        )
        .expect("27db915c router invariant must hold");
    let state = server_admin::admin_auth_svc_state::AdminAuthSvcState::try_new(
        app_state::sqlx_pg_pool::SqlxPgPool::from(pool),
        &env::<config_lib::admin_jwt_secret::AdminJwtSecret>(StdAdminApiTestStrRef::from(
            constants_str::catalog::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES,
        )),
        &env::<config_lib::admin_access_token_ttl_seconds::AdminAccessTokenTtlSeconds>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_900),
        ),
        &env::<config_lib::admin_refresh_token_ttl_seconds::AdminRefreshTokenTtlSeconds>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_3600),
        ),
        &env::<config_lib::admin_session_limit::AdminSessionLimit>(StdAdminApiTestStrRef::from(
            constants_str::catalog::VALUE_20,
        )),
        &env::<config_lib::admin_sign_in_rate_limit::AdminSignInRateLimit>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_2),
        ),
        &env::<config_lib::admin_login_failure_limit::AdminLoginFailureLimit>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_10),
        ),
        &env::<config_lib::admin_password_hash_concurrency::AdminPasswordHashConcurrency>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_1),
        ),
        &env::<config_lib::admin_cookie_secure::AdminCookieSecure>(StdAdminApiTestStrRef::from(
            constants_str::catalog::FALSE,
        )),
        &env::<config_lib::admin_token_issuer::AdminTokenIssuer>(StdAdminApiTestStrRef::from(
            constants_str::catalog::INTEGRATION_TEST,
        )),
        &env::<config_lib::admin_token_audience::AdminTokenAudience>(StdAdminApiTestStrRef::from(
            constants_str::catalog::INTEGRATION_TEST_ADMIN,
        )),
        &config_lib::domain_types::CorsAllowOrigin(
            constants_str::catalog::HTTP_LOCALHOST.to_owned(),
        ),
    )
    .expect("f7d8c961 router invariant must hold");
    AxumAdminApiTestRouter::from(axum::Router::from(
        server_admin::admin_auth_routes::admin_auth_routes(
            server_admin::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc::from(
                std::sync::Arc::new(state),
            ),
        ),
    ))
}
fn router_with_pool(pool: &SqlxAdminApiTestPool) -> AxumAdminApiTestRouter {
    let state = server_admin::admin_auth_svc_state::AdminAuthSvcState::try_new(
        app_state::sqlx_pg_pool::SqlxPgPool::from(pool.0.clone()),
        &env::<config_lib::admin_jwt_secret::AdminJwtSecret>(StdAdminApiTestStrRef::from(
            constants_str::catalog::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES,
        )),
        &env::<config_lib::admin_access_token_ttl_seconds::AdminAccessTokenTtlSeconds>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_900),
        ),
        &env::<config_lib::admin_refresh_token_ttl_seconds::AdminRefreshTokenTtlSeconds>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_3600),
        ),
        &env::<config_lib::admin_session_limit::AdminSessionLimit>(StdAdminApiTestStrRef::from(
            constants_str::catalog::VALUE_20,
        )),
        &env::<config_lib::admin_sign_in_rate_limit::AdminSignInRateLimit>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_2),
        ),
        &env::<config_lib::admin_login_failure_limit::AdminLoginFailureLimit>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_10),
        ),
        &env::<config_lib::admin_password_hash_concurrency::AdminPasswordHashConcurrency>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_1),
        ),
        &env::<config_lib::admin_cookie_secure::AdminCookieSecure>(StdAdminApiTestStrRef::from(
            constants_str::catalog::FALSE,
        )),
        &env::<config_lib::admin_token_issuer::AdminTokenIssuer>(StdAdminApiTestStrRef::from(
            constants_str::catalog::INTEGRATION_TEST,
        )),
        &env::<config_lib::admin_token_audience::AdminTokenAudience>(StdAdminApiTestStrRef::from(
            constants_str::catalog::INTEGRATION_TEST_ADMIN,
        )),
        &config_lib::domain_types::CorsAllowOrigin(
            constants_str::catalog::HTTP_LOCALHOST.to_owned(),
        ),
    )
    .expect("a59d73c1 router_with_pool invariant must hold");
    AxumAdminApiTestRouter::from(axum::Router::from(
        server_admin::admin_auth_routes::admin_auth_routes(
            server_admin::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc::from(
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
        StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_127_0_0_1_43210),
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
        .header(
            http::header::CONTENT_TYPE,
            constants_str::catalog::APPLICATION_JSON,
        )
        .header(http::header::ORIGIN, constants_str::catalog::HTTP_LOCALHOST);
    if let Some(value) = cookie {
        builder = builder.header(http::header::COOKIE, value.0);
    }
    if let Some(value) = csrf {
        builder = builder.header(constants_str::catalog::X_CSRF_TOKEN_ALT, value.0);
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
            constants_str::test_fixtures::APPLICATION_X_WWW_FORM_URLENCODED,
        )
        .header(http::header::ORIGIN, constants_str::catalog::HTTP_LOCALHOST);
    if let Some(value) = cookie {
        builder = builder.header(http::header::COOKIE, value.0);
    }
    let mut request = builder
        .body(axum::body::Body::from(body.0.to_owned()))
        .expect("9f211b84 html_request_with_peer invariant must hold");
    let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
        constants_str::catalog::VALUE_127_0_0_1_43210
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
            .contains("src=\"/admin/assets/admin_csr_application.js?v=20260801-37\""),
        "CSR application script is missing"
    );
    assert!(!body.0.contains("<table"), "server rendered a data table");
    assert!(!body.0.contains("<form"), "server rendered a data form");
}
#[expect(
    clippy::missing_assert_message,
    reason = "the asserted status identifies the failed fixture stage"
)]
async fn admin_html_test_fixture_with_password_change(
    password_change_required: server_admin_contract::admin_bool::AdminBool,
) -> AdminHtmlTestFixture {
    let database_url = std::env::var(constants_str::catalog::ENV_NAMES_DATABASE_URL)
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
    let _locked =
        sqlx::query(constants_str::integration_fixtures::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
            .execute(&mut *lock)
            .await
            .expect("a6b7c8d9 admin_html_test_fixture_with_password_change invariant must hold");
    server_admin::prepare_postgresql::prepare_postgresql(
        app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
    )
    .await
    .expect("45de3a61 admin_html_test_fixture_with_password_change invariant must hold");
    let _truncated = sqlx::query(
        constants_str::catalog::TRUNCATE_ADMIN_RATE_LIMITS_ADMIN_AUDIT_LOG_ADMIN_LOGIN_ATTEMPTS_ADMIN_ACCESS,
    )
    .execute(&pool.0)
    .await
    .expect("cf37a9e2 admin_html_test_fixture_with_password_change invariant must hold");
    let _deleted_non_system_roles = sqlx::query(constants_str::test_fixtures::VALUE_4BCE193A)
        .execute(&pool.0)
        .await
        .expect("b267a647 admin_html_test_fixture_with_password_change invariant must hold");
    let password = serde_json::from_str::<
        server_admin_contract::admin_new_password::AdminNewPassword,
    >(constants_str::catalog::CORRECT_PASSWORD)
    .expect("d20a35e4 admin_html_test_fixture_with_password_change invariant must hold");
    let hasher = server_admin::admin_password_hasher::AdminPasswordHasher::new(
        server_admin::runtime_admin_password_hash_concurrency::RuntimeAdminPasswordHashConcurrency::from(
            std::num::NonZeroUsize::new(constants_usize::ONE).expect(
                "560498ab admin_html_test_fixture_with_password_change invariant must hold",
            ),
        ),
    );
    let _created_admin_id =
        server_admin::create_initial_administrator::create_initial_administrator(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
            server_admin_contract::admin_login::AdminLogin::try_from(
                constants_str::catalog::ADMIN_ALT.to_owned(),
            )
            .expect("6a417bde admin_html_test_fixture_with_password_change invariant must hold"),
            server_admin_contract::admin_display_name::AdminDisplayName::try_from(
                constants_str::catalog::ADMIN.to_owned(),
            )
            .expect("703fc568 admin_html_test_fixture_with_password_change invariant must hold"),
            password,
            &hasher,
        )
        .await
        .expect("1e29c87f admin_html_test_fixture_with_password_change invariant must hold");
    if !bool::from(password_change_required) {
        let _updated = sqlx::query(
            constants_str::integration_fixtures::UPDATE_ADMIN_USERS_SET_MUST_CHANGE_PASSWORD_FALSE,
        )
        .execute(&pool.0)
        .await
        .expect("a37042f1 admin_html_test_fixture_with_password_change invariant must hold");
    }
    let state = server_admin::admin_auth_svc_state::AdminAuthSvcState::try_new(
        app_state::sqlx_pg_pool::SqlxPgPool::from(pool.0.clone()),
        &env::<config_lib::admin_jwt_secret::AdminJwtSecret>(StdAdminApiTestStrRef::from(
            constants_str::catalog::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES,
        )),
        &env::<config_lib::admin_access_token_ttl_seconds::AdminAccessTokenTtlSeconds>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_900),
        ),
        &env::<config_lib::admin_refresh_token_ttl_seconds::AdminRefreshTokenTtlSeconds>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_3600),
        ),
        &env::<config_lib::admin_session_limit::AdminSessionLimit>(StdAdminApiTestStrRef::from(
            constants_str::catalog::VALUE_20,
        )),
        &env::<config_lib::admin_sign_in_rate_limit::AdminSignInRateLimit>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_20),
        ),
        &env::<config_lib::admin_login_failure_limit::AdminLoginFailureLimit>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_10),
        ),
        &env::<config_lib::admin_password_hash_concurrency::AdminPasswordHashConcurrency>(
            StdAdminApiTestStrRef::from(constants_str::catalog::VALUE_1),
        ),
        &env::<config_lib::admin_cookie_secure::AdminCookieSecure>(StdAdminApiTestStrRef::from(
            constants_str::catalog::FALSE,
        )),
        &env::<config_lib::admin_token_issuer::AdminTokenIssuer>(StdAdminApiTestStrRef::from(
            constants_str::catalog::INTEGRATION_TEST,
        )),
        &env::<config_lib::admin_token_audience::AdminTokenAudience>(StdAdminApiTestStrRef::from(
            constants_str::catalog::INTEGRATION_TEST_ADMIN,
        )),
        &config_lib::domain_types::CorsAllowOrigin(
            constants_str::catalog::HTTP_LOCALHOST.to_owned(),
        ),
    )
    .expect("ec39b61d admin_html_test_fixture_with_password_change invariant must hold");
    let router = AxumAdminApiTestRouter::from(axum::Router::from(
        server_admin::html_routes_with_swagger::html_routes_with_swagger(
            server_admin::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc::from(
                std::sync::Arc::new(state),
            ),
            server_admin::admin_html_swagger_enabled::AdminHtmlSwaggerEnabled::from(true),
        ),
    ));
    let correct_password = serde_json::from_str::<String>(constants_str::catalog::CORRECT_PASSWORD)
        .expect("825e50c7 admin_html_test_fixture_with_password_change invariant must hold");
    let sign_in_body = AdminHtmlTestFormBody::try_from(format!(
        "login={}&password={correct_password}",
        constants_str::catalog::ADMIN_ALT,
    ))
    .expect("9df2164c admin_html_test_fixture_with_password_change invariant must hold");
    let sign_in_response = tower::ServiceExt::oneshot(
        router.0.clone(),
        html_request_with_peer(
            HttpAdminApiTestMethod::from(http::Method::POST),
            StdAdminApiTestStrRef::from(
                server_admin_contract::admin_html_action::AdminHtmlAction::SignIn.get(),
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
        StdAdminApiTestStrRef::from(constants_str::catalog::ADMIN_ACCESS_TOKEN),
    );
    let refresh = cookie_value(
        HttpAdminApiTestResponseRef::from(&sign_in_response),
        StdAdminApiTestStrRef::from(constants_str::catalog::ADMIN_REFRESH_TOKEN_ALT),
    );
    let csrf = cookie_value(
        HttpAdminApiTestResponseRef::from(&sign_in_response),
        StdAdminApiTestStrRef::from(constants_str::catalog::ADMIN_CSRF_TOKEN_ALT),
    );
    AdminHtmlTestFixture {
        cookie: StdAdminApiTestCookie::try_from(format!(
            "{}{access}; {}{refresh}; {}{csrf}",
            constants_str::catalog::ADMIN_ACCESS_TOKEN,
            constants_str::catalog::ADMIN_REFRESH_TOKEN_ALT,
            constants_str::catalog::ADMIN_CSRF_TOKEN_ALT,
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
        server_admin_contract::admin_bool::AdminBool::from(false),
    )
    .await
}
async fn postgres_accepts_admin_user_policy_values(
    pool: &SqlxAdminApiTestPool,
    display_name: StdAdminApiTestStrRef<'_>,
    login: StdAdminApiTestStrRef<'_>,
) -> server_admin_contract::admin_bool::AdminBool {
    let mut transaction = pool
        .0
        .begin()
        .await
        .expect("e6f2cdf7 postgres_accepts_admin_user_policy_values invariant must hold");
    let accepted = sqlx::query(constants_str::test_fixtures::INSERT_ADMIN_USER_POLICY_PROBE)
        .bind(login.0)
        .bind(display_name.0)
        .bind(constants_str::catalog::X)
        .execute(&mut *transaction)
        .await
        .is_ok();
    transaction
        .rollback()
        .await
        .expect("fc4eec8f postgres_accepts_admin_user_policy_values invariant must hold");
    server_admin_contract::admin_bool::AdminBool::from(accepted)
}
async fn postgres_accepts_admin_role_policy_value(
    pool: &SqlxAdminApiTestPool,
    name: StdAdminApiTestStrRef<'_>,
) -> server_admin_contract::admin_bool::AdminBool {
    let mut transaction = pool
        .0
        .begin()
        .await
        .expect("77c2db82 postgres_accepts_admin_role_policy_value invariant must hold");
    let accepted = sqlx::query(constants_str::test_fixtures::INSERT_ADMIN_ROLE_POLICY_PROBE)
        .bind(name.0)
        .execute(&mut *transaction)
        .await
        .is_ok();
    transaction
        .rollback()
        .await
        .expect("aa9b0106 postgres_accepts_admin_role_policy_value invariant must hold");
    server_admin_contract::admin_bool::AdminBool::from(accepted)
}
