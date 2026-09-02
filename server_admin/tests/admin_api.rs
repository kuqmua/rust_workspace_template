// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(unused_crate_dependencies)]
// integration target inherits the library dependency graph while exercising the assembled public router
// The owner module retains lint-sensitive semantics from the original implementation.
#![allow(clippy::tests_outside_test_module)] // every item in this integration target is compiled exclusively by the test harness
mod test_data_tables {
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_postgresql_data_table_api_reads_every_public_field_from_every_table() {
        let fixture = crate::admin_html_test_fixture().await;
        let _cleanup_status = sqlx::query(constants_str::VALUE_6E1CBD4B)
            .execute(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_70DFA001);
        let _rate_limit = sqlx::query(constants_str::VALUE_91A1975C)
            .execute(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_F8F27048);
        futures::StreamExt::fold(
            futures::stream::iter(
                server_admin_contract::admin_data_table::AdminDataTable::PG_ORDER,
            ),
            (),
            async |(), table| {
                let uri = format!("/tables/{table}?limit=100&offset=0");
                let response = tower::ServiceExt::oneshot(
                    crate::router_with_pool(&fixture.pool).0,
                    crate::request_with_peer(
                        super::HttpAdminApiTestMethod::from(http::Method::GET),
                        super::StdAdminApiTestStrRef::from(uri.as_str()),
                        super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                        Some(super::StdAdminApiTestStrRef::from(
                            fixture.cookie.0.as_str(),
                        )),
                        None,
                    )
                    .0,
                )
                .await
                .expect(constants_str::DIAGNOSTIC_4B58A9BA);
                assert_eq!(
                    response.status(),
                    http::StatusCode::OK,
                    "table API {table} failed"
                );
                let body =
                    axum::body::to_bytes(response.into_body(), constants_usize::VALUE_1_048_576)
                        .await
                        .expect(constants_str::DIAGNOSTIC_78547EED);
                let view = serde_json::from_slice::<
                    server_admin_contract::admin_data_table_view::AdminDataTableView,
                >(body.as_ref())
                .expect(constants_str::DIAGNOSTIC_6D2A32E6);
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
        fixture
            .lock
            .0
            .rollback()
            .await
            .expect(constants_str::DIAGNOSTIC_83226FD7);
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_postgresql_generated_mutation_idempotency_contract() {
        let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL)
            .expect(constants_str::DIAGNOSTIC_40C1E398);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4u32)
            .connect(database_url.as_str())
            .await
            .expect(constants_str::DIAGNOSTIC_CB6830BC);
        let mut idempotency_test_isolation = pool
            .begin()
            .await
            .expect(constants_str::DIAGNOSTIC_EA1D891D);
        pg_crud_common::lock_pg_relation_resources::lock_pg_relation_resources(
            pg_crud_common::sqlx_pg_relation_lock_connection_ref::SqlxPgRelationLockConnectionRef::from(
                &mut *idempotency_test_isolation,
            ),
            &pg_crud_common::pg_relation_lock_namespace::PgRelationLockNamespace::try_from(
                constants_str::ACTOR_ATOMIC.to_owned(),
            )
            .expect(constants_str::DIAGNOSTIC_136C5ACC),
            &pg_crud_common::pg_relation_resource_ids::PgRelationResourceIds::try_from(vec![
                pg_crud_common::pg_relation_resource_id::PgRelationResourceId::from(constants_i64::ONE),
            ])
            .expect(constants_str::DIAGNOSTIC_8B0C7AE1),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_508DB033);
        pg_table::ensure_pg_table_idempotency_schema::ensure_pg_table_idempotency_schema(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_6C338824);
        let _truncate_result = sqlx::query(constants_str::TRUNCATE_PG_TABLE_IDEMPOTENCY)
            .execute(&pool)
            .await
            .expect(constants_str::DIAGNOSTIC_D93BEB69);
        let make_request =
            |actor: super::StdAdminApiTestStrRef<'_>,
             route: super::StdAdminApiTestStrRef<'_>,
             key: super::StdAdminApiTestStrRef<'_>,
             body: pg_table::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef<'_>| {
                pg_table::pg_table_idempotency_request::PgTableIdempotencyRequest::new(
            pg_table::pg_table_idempotency_scope::PgTableIdempotencyScope::new(
                pg_table::pg_table_idempotency_actor::PgTableIdempotencyActor::try_from(actor.0.to_owned()).expect(constants_str::DIAGNOSTIC_E6640036),
                pg_table::pg_table_idempotency_method::PgTableIdempotencyMethod::try_from(constants_str::POST.to_owned())
                    .expect(constants_str::DIAGNOSTIC_94BC0508),
                pg_table::pg_table_idempotency_route::PgTableIdempotencyRoute::try_from(route.0.to_owned()).expect(constants_str::DIAGNOSTIC_4E8C040F),
                pg_table::pg_table_idempotency_key::PgTableIdempotencyKey::try_from(key.0.to_owned()).expect(constants_str::DIAGNOSTIC_2028024D),
            ),
            body,
        )
            };
        let first_request = make_request(
            super::StdAdminApiTestStrRef::from(constants_str::ACTOR_A),
            super::StdAdminApiTestStrRef::from(constants_str::ITEMS_CM),
            super::StdAdminApiTestStrRef::from(constants_str::KEY_A),
            pg_table::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(
                br#"{"value":1}"#.as_slice(),
            ),
        );
        let first = pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            &first_request,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_C8B3565C);
        assert_eq!(
            first,
            pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::Acquired
        );
        let pending = pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            &first_request,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_C5C45332);
        assert_eq!(
            pending,
            pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::InProgress
        );
        let conflicting_request = make_request(
            super::StdAdminApiTestStrRef::from(constants_str::ACTOR_A),
            super::StdAdminApiTestStrRef::from(constants_str::ITEMS_CM),
            super::StdAdminApiTestStrRef::from(constants_str::KEY_A),
            pg_table::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(
                br#"{"value":2}"#.as_slice(),
            ),
        );
        let conflict = pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            &conflicting_request,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_7F419767);
        assert_eq!(
            conflict,
            pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::Conflict
        );
        let response_body = br#"{"desirable":{"id":1}}"#;
        pg_table::complete_pg_table_idempotency::complete_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            &first_request,
            pg_table::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus::try_from(201u16).expect(constants_str::DIAGNOSTIC_4DF2DD1F),
            pg_table::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(response_body.as_slice()),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_9106C1E6);
        let replay = pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            &first_request,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_0721B23F);
        let pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::Replay(replay_value) =
            replay
        else {
            std::panic::panic_any(constants_str::PANIC_9F97FB0D);
        };
        assert_eq!(
        replay_value.into_parts(),
        (
            pg_table::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus::try_from(201u16).expect(constants_str::DIAGNOSTIC_F89D923D),
            pg_table::pg_table_idempotency_body::PgTableIdempotencyBody::try_from(response_body.to_vec()).expect(constants_str::DIAGNOSTIC_4A01ED0E),
        )
    );
        let other_actor = make_request(
            super::StdAdminApiTestStrRef::from(constants_str::ACTOR_B),
            super::StdAdminApiTestStrRef::from(constants_str::ITEMS_CM),
            super::StdAdminApiTestStrRef::from(constants_str::KEY_A),
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
            .expect(constants_str::DIAGNOSTIC_E581D572),
            pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::Acquired
        );
        pg_table::release_pg_table_idempotency::release_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            &other_actor,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_31E0437D);
        assert_eq!(
            pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
                app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
                &other_actor
            )
            .await
            .expect(constants_str::DIAGNOSTIC_FE57D4DC),
            pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::Acquired
        );
        let concurrent = make_request(
            super::StdAdminApiTestStrRef::from(constants_str::ACTOR_CONCURRENT),
            super::StdAdminApiTestStrRef::from(constants_str::ITEMS_CM),
            super::StdAdminApiTestStrRef::from(constants_str::KEY_CONCURRENT),
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
            left.expect(constants_str::DIAGNOSTIC_874153EC),
            right.expect(constants_str::DIAGNOSTIC_64C4CC46),
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
            constants_str::CREATE_TABLE_IF_NOT_EXISTS_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST_ID_BIGINT,
        )
        .execute(&pool)
        .await
        .expect(constants_str::DIAGNOSTIC_AF066E8B);
        let _atomic_clear = sqlx::query(constants_str::TRUNCATE_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST)
            .execute(&pool)
            .await
            .expect(constants_str::DIAGNOSTIC_3130E593);
        let atomic = make_request(
            super::StdAdminApiTestStrRef::from(constants_str::ACTOR_ATOMIC),
            super::StdAdminApiTestStrRef::from(constants_str::ITEMS_CO),
            super::StdAdminApiTestStrRef::from(constants_str::KEY_ATOMIC),
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
            .expect(constants_str::DIAGNOSTIC_925EA283),
            pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::Acquired
        );
        let mut rollback_tx = pool
            .begin()
            .await
            .expect(constants_str::DIAGNOSTIC_FCBA80E1);
        let _mutation =
            sqlx::query(constants_str::INSERT_INTO_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST_ID_VALUES_1)
                .execute(&mut *rollback_tx)
                .await
                .expect(constants_str::DIAGNOSTIC_67503E70);
        pg_table::complete_pg_table_idempotency_in_connection::complete_pg_table_idempotency_in_connection(
            pg_table::sqlx_pg_table_pg_connection_ref::SqlxPgTablePgConnectionRef::from(&mut *rollback_tx),
            &atomic,
            pg_table::pg_table_idempotency_response_status::PgTableIdempotencyResponseStatus::try_from(201u16).expect(constants_str::DIAGNOSTIC_98BB1DB9),
            pg_table::pg_table_idempotency_body_ref::PgTableIdempotencyBodyRef::from(br#"{"id":1}"#.as_slice()),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_8AD86515);
        rollback_tx
            .rollback()
            .await
            .expect(constants_str::DIAGNOSTIC_11CFCB27);
        let mutation_count = sqlx::query_scalar::<_, i64>(
            constants_str::SELECT_COUNT_ASTERISK_FROM_PG_TABLE_IDEMPOTENCY_ATOMIC_TEST,
        )
        .fetch_one(&pool)
        .await
        .expect(constants_str::DIAGNOSTIC_84E57AB6);
        assert_eq!(mutation_count, constants_i64::ZERO);
        assert_eq!(
            pg_table::begin_pg_table_idempotency::begin_pg_table_idempotency(
                app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
                &atomic
            )
            .await
            .expect(constants_str::DIAGNOSTIC_3903BF53),
            pg_table::pg_table_idempotency_begin::PgTableIdempotencyBegin::InProgress
        );
        pg_table::release_pg_table_idempotency::release_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            &atomic,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_67973E68);
        let _age_records = sqlx::query(
            constants_str::UPDATE_PG_TABLE_IDEMPOTENCY_SET_CREATED_AT_TIMESTAMPTZ_2000_01_01_00,
        )
        .execute(&pool)
        .await
        .expect(constants_str::DIAGNOSTIC_A46F7336);
        let before_cleanup = sqlx::query_scalar::<_, i64>(
            constants_str::SELECT_COUNT_ASTERISK_FROM_PG_TABLE_IDEMPOTENCY,
        )
        .fetch_one(&pool)
        .await
        .expect(constants_str::DIAGNOSTIC_2C080F6D);
        let cleaned = pg_table::cleanup_pg_table_idempotency::cleanup_pg_table_idempotency(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            pg_table::pg_table_idempotency_cleanup_retention_seconds::PgTableIdempotencyCleanupRetentionSeconds::try_from(3_600i64).expect(constants_str::DIAGNOSTIC_52189299),
            pg_table::pg_table_idempotency_cleanup_retention_seconds::PgTableIdempotencyCleanupRetentionSeconds::try_from(3_600i64).expect(constants_str::DIAGNOSTIC_FA6DC1D7),
            pg_table::pg_table_idempotency_cleanup_batch_size::PgTableIdempotencyCleanupBatchSize::try_from(2i64).expect(constants_str::DIAGNOSTIC_1780D6B1),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_B1BA49CC);
        assert_eq!(u64::from(cleaned), 2u64);
        let after_cleanup = sqlx::query_scalar::<_, i64>(
            constants_str::SELECT_COUNT_ASTERISK_FROM_PG_TABLE_IDEMPOTENCY,
        )
        .fetch_one(&pool)
        .await
        .expect(constants_str::DIAGNOSTIC_6863201E);
        assert_eq!(
            before_cleanup
                .checked_sub(after_cleanup)
                .expect(constants_str::DIAGNOSTIC_F93ED3CF),
            2i64
        );
    }
}
mod test_flow {
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_flow() {
        let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL)
            .expect(constants_str::DIAGNOSTIC_AC0CB9E3);
        let pool = super::SqlxAdminApiTestPool::from(
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(database_url.as_str())
                .await
                .expect(constants_str::DIAGNOSTIC_A3E1F57C),
        );
        let mut admin_db_test_lock = pool
            .0
            .begin()
            .await
            .expect(constants_str::DIAGNOSTIC_4DFB6865);
        let _locked = sqlx::query(constants_str::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
            .execute(&mut *admin_db_test_lock)
            .await
            .expect(constants_str::DIAGNOSTIC_693B147F);
        server_admin::prepare_postgresql::prepare_postgresql(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_0EA8D516);
        server_admin::prepare_postgresql::prepare_postgresql(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_676C00F1);
        server_admin::validate_catalog_schema::validate_catalog_schema(
            pg_crud_common::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef::from(&pool.0),
            pg_crud_common::db_schema_name_ref::DbSchemaNameRef::from(constants_str::PUBLIC),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_65CE07E9);
        let observed_permissions = sqlx::query_scalar::<_, String>(
            constants_str::SELECT_NAME_FROM_ADMIN_PERMISSIONS_ORDER_BY_NAME,
        )
        .fetch_all(&pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_DB765F20);
        let expected_permissions = server_admin_contract::admin_permission::AdminPermission::ALL
            .into_iter()
            .map(|permission| permission.as_str().as_ref().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(observed_permissions, expected_permissions);
        let _deleted_permission = sqlx::query(constants_str::DELETE_ADMIN_PERMISSION_BY_NAME)
            .bind(
                server_admin_contract::admin_permission::AdminPermission::ALL
                    .first()
                    .expect(constants_str::DIAGNOSTIC_26D95EA4)
                    .as_str()
                    .as_ref(),
            )
            .execute(&pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_9D762F8C);
        server_admin::prepare_postgresql::prepare_postgresql(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_EA3F641D);
        let reconciled_permissions = sqlx::query_scalar::<_, String>(
            constants_str::SELECT_NAME_FROM_ADMIN_PERMISSIONS_ORDER_BY_NAME,
        )
        .fetch_all(&pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_458AB19E);
        assert_eq!(reconciled_permissions, expected_permissions);
        let _truncate_result = sqlx::query(
        constants_str::TRUNCATE_ADMIN_RATE_LIMITS_ADMIN_AUDIT_LOG_ADMIN_LOGIN_ATTEMPTS_ADMIN_ACCESS,
    )
    .execute(&pool.0)
    .await
    .expect(constants_str::DIAGNOSTIC_97B5AD2F);
        let password = serde_json::from_str::<
            server_admin_contract::admin_new_password::AdminNewPassword,
        >(constants_str::CORRECT_PASSWORD)
        .expect(constants_str::DIAGNOSTIC_703A8DF2);
        let hasher = server_admin::admin_password_hasher::AdminPasswordHasher::new(
            server_admin::runtime_admin_password_hash_concurrency::RuntimeAdminPasswordHashConcurrency::from(
                std::num::NonZeroUsize::new(1).expect(constants_str::DIAGNOSTIC_271F96D4),
            ),
        );
        let _admin_id = server_admin::create_initial_administrator::create_initial_administrator(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
            server_admin_contract::admin_login::AdminLogin::try_from(
                constants_str::ADMIN_ALT.to_owned(),
            )
            .expect(constants_str::DIAGNOSTIC_98C7E04A),
            server_admin_contract::admin_display_name::AdminDisplayName::try_from(
                constants_str::ADMIN.to_owned(),
            )
            .expect(constants_str::DIAGNOSTIC_48EFED01),
            password,
            &hasher,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_E2C94D67);
        let password_change_required = sqlx::query_scalar::<_, bool>(
            constants_str::SELECT_MUST_CHANGE_PASSWORD_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
        )
        .fetch_one(&pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_81F3C9D2);
        assert!(password_change_required);
        let original_password_hash = sqlx::query_scalar::<_, String>(
            constants_str::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
        )
        .fetch_one(&pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_1282B56E);
        let repeated_password = serde_json::from_str::<
            server_admin_contract::admin_new_password::AdminNewPassword,
        >(constants_str::DIFFERENT_PASSWORD)
        .expect(constants_str::DIAGNOSTIC_E411F376);
        assert!(matches!(
        server_admin::create_initial_administrator::create_initial_administrator(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
            server_admin_contract::admin_login::AdminLogin::try_from(constants_str::VALUE_23996F85.to_owned()).expect(constants_str::DIAGNOSTIC_8359CA1A),
            server_admin_contract::admin_display_name::AdminDisplayName::try_from(constants_str::VALUE_C7DE0670.to_owned())
                .expect(constants_str::DIAGNOSTIC_D968DDDB),
            repeated_password,
            &hasher,
        )
        .await,
        Err(server_admin::initial_administrator_creation_error::InitialAdministratorCreationError::AlreadyInitialized)
    ));
        let preserved_password_hash = sqlx::query_scalar::<_, String>(
            constants_str::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
        )
        .fetch_one(&pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_65FF827E);
        assert_eq!(preserved_password_hash, original_password_hash);
        let administrator_count =
            sqlx::query_scalar::<_, i64>(constants_str::SELECT_COUNT_ASTERISK_FROM_ADMIN_USERS)
                .fetch_one(&pool.0)
                .await
                .expect(constants_str::DIAGNOSTIC_AE89C3BD);
        assert_eq!(administrator_count, constants_i64::ONE);
        let admin_id = sqlx::query_scalar::<_, i64>(
            constants_str::SELECT_ID_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
        )
        .fetch_one(&pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_A61329BF);
        let dangling_role_links = sqlx::query_scalar::<_, i64>(
            constants_str::SELECT_COUNT_ASTERISK_FROM_ADMIN_USER_ROLES_LINK_LEFT_JOIN_ADMIN_USERS,
        )
        .fetch_one(&pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_08EF120F);
        assert_eq!(dangling_role_links, constants_i64::ZERO);
        let dangling_permission_links = sqlx::query_scalar::<_, i64>(
        constants_str::SELECT_COUNT_ASTERISK_FROM_ADMIN_ROLE_PERMISSIONS_LINK_LEFT_JOIN_ADMIN_ROLES,
    )
    .fetch_one(&pool.0)
    .await
    .expect(constants_str::DIAGNOSTIC_AEBF6DC8);
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
                    constants_str::LOGIN_ADMIN_PASSWORD_WRONG_PASSWORD,
                ),
                None,
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_5472EA19);
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
                    constants_str::LOGIN_ADMIN_PASSWORD_CORRECT_PASSWORD,
                ),
                None,
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_C245193E);
        assert_eq!(sign_in_response.status(), http::StatusCode::OK);
        let access = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&sign_in_response),
            super::StdAdminApiTestStrRef::from(constants_str::ADMIN_ACCESS_TOKEN),
        );
        let refresh = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&sign_in_response),
            super::StdAdminApiTestStrRef::from(constants_str::ADMIN_REFRESH_TOKEN_ALT),
        );
        let csrf = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&sign_in_response),
            super::StdAdminApiTestStrRef::from(constants_str::ADMIN_CSRF_TOKEN_ALT),
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
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_B67815EC);
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
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(cookie.as_str())),
                None,
                super::StdAdminApiTestStrRef::from(constants_str::VALUE_127_0_0_2_43210),
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_F11E0324);
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
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(
                    first_refresh_cookie.as_str(),
                )),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_9F0BE285);
        assert_eq!(refresh_response.status(), http::StatusCode::OK);
        let refreshed_access = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&refresh_response),
            super::StdAdminApiTestStrRef::from(constants_str::ADMIN_ACCESS_TOKEN),
        );
        assert!(
            refresh_response
                .headers()
                .get_all(http::header::SET_COOKIE)
                .iter()
                .filter_map(|value| value.to_str().ok())
                .any(|value| value.starts_with(constants_str::ADMIN_REFRESH_TOKEN_ALT))
        );
        let rotated_refresh = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&refresh_response),
            super::StdAdminApiTestStrRef::from(constants_str::ADMIN_REFRESH_TOKEN),
        );
        let refreshed_csrf = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&refresh_response),
            super::StdAdminApiTestStrRef::from(constants_str::ADMIN_CSRF_TOKEN_ALT),
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
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(
                    first_refresh_cookie.as_str(),
                )),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_B8C71E43);
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
                    constants_str::LOGIN_LOCKED_USER_PASSWORD_WRONG_PASSWORD,
                ),
                None,
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_8F72B01E);
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
                    constants_str::LOGIN_LOCKED_USER_PASSWORD_WRONG_PASSWORD,
                ),
                None,
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_2D94C01E);
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
                    constants_str::LOGIN_LOCKED_USER_PASSWORD_WRONG_PASSWORD,
                ),
                None,
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_7324AF80);
        assert_eq!(
            limited_response.status(),
            http::StatusCode::TOO_MANY_REQUESTS
        );
        let password_change_gate_response = tower::ServiceExt::oneshot(
        crate::router_with_pool(&pool).0,
        crate::request_with_peer(
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(frontend_contract::typed_route_path::typed_route_path::<server_admin_contract::admin_list_users_route::AdminListUsersRoute>().as_ref()),
            super::StdAdminApiTestStrRef::from(constants_str::LOGIN_LIMITED_USER_DISPLAY_NAME_LIMITED_USER_PASSWORD_LIMITED_PASSWORD),
            Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(super::StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect(constants_str::DIAGNOSTIC_D78B315C);
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
                    constants_str::CURRENT_PASSWORD_CORRECT_NEW_PASSWORD_CHANGED,
                ),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_820FBB75);
        assert_eq!(
            change_password_response.status(),
            http::StatusCode::NO_CONTENT
        );
        let csrf_denied_response = tower::ServiceExt::oneshot(
        crate::router_with_pool(&pool).0,
        crate::request_with_peer(
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(frontend_contract::typed_route_path::typed_route_path::<server_admin_contract::admin_list_users_route::AdminListUsersRoute>().as_ref()),
            super::StdAdminApiTestStrRef::from(constants_str::LOGIN_LIMITED_USER_DISPLAY_NAME_LIMITED_USER_PASSWORD_LIMITED_PASSWORD),
            Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect(constants_str::DIAGNOSTIC_153B847C);
        assert_eq!(csrf_denied_response.status(), http::StatusCode::FORBIDDEN);
        let create_response = tower::ServiceExt::oneshot(
        crate::router_with_pool(&pool).0,
        crate::request_with_peer(
            super::HttpAdminApiTestMethod::from(http::Method::POST),
            super::StdAdminApiTestStrRef::from(frontend_contract::typed_route_path::typed_route_path::<server_admin_contract::admin_list_users_route::AdminListUsersRoute>().as_ref()),
            super::StdAdminApiTestStrRef::from(constants_str::LOGIN_LIMITED_USER_DISPLAY_NAME_LIMITED_USER_PASSWORD_LIMITED_PASSWORD),
            Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
            Some(super::StdAdminApiTestStrRef::from(refreshed_csrf.0.as_str())),
        )
        .0,
    )
    .await
    .expect(constants_str::DIAGNOSTIC_C86A4310);
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
                    constants_str::LOGIN_LIMITED_USER_PASSWORD_LIMITED_PASSWORD,
                ),
                None,
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_A2D6139E);
        assert_eq!(limited_sign_in_response.status(), http::StatusCode::OK);
        let limited_access = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&limited_sign_in_response),
            super::StdAdminApiTestStrRef::from(constants_str::ADMIN_ACCESS_TOKEN),
        );
        let limited_refresh = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&limited_sign_in_response),
            super::StdAdminApiTestStrRef::from(constants_str::ADMIN_REFRESH_TOKEN_ALT),
        );
        let limited_csrf = crate::cookie_value(
            super::HttpAdminApiTestResponseRef::from(&limited_sign_in_response),
            super::StdAdminApiTestStrRef::from(constants_str::ADMIN_CSRF_TOKEN_ALT),
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
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(limited_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_617F08B9);
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
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(limited_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(limited_csrf.0.as_str())),
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_0F51DC7A);
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
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(limited_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_24EC178B);
        assert_eq!(
            revoked_all_response.status(),
            http::StatusCode::UNAUTHORIZED
        );
        let limited_id = sqlx::query_scalar::<_, i64>(
            constants_str::SELECT_ID_FROM_ADMIN_USERS_WHERE_LOGIN_LIMITED_USER,
        )
        .fetch_one(&pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_10C8F7D2);
        let update_user_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::PATCH),
                super::StdAdminApiTestStrRef::from(format!("/users/{limited_id}").as_str()),
                super::StdAdminApiTestStrRef::from(constants_str::DISPLAY_NAME_UPDATED_USER),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_623CDE18);
        assert_eq!(update_user_response.status(), http::StatusCode::NO_CONTENT);
        let ban_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(format!("/users/{limited_id}/ban").as_str()),
                super::StdAdminApiTestStrRef::from(constants_str::IS_BANNED_TRUE),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_94A7E1CB);
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
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(limited_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_FAC2138B);
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
                    constants_str::LOGIN_LIMITED_USER_PASSWORD_LIMITED_PASSWORD,
                ),
                None,
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_891D7CA2);
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
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_475AF63B);
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
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_C5F103DA);
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
                super::StdAdminApiTestStrRef::from(constants_str::NAME_TEMPORARY_ROLE),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_6D9384FE);
        assert_eq!(create_role_response.status(), http::StatusCode::CREATED);
        let role_id = sqlx::query_scalar::<_, i64>(
            constants_str::SELECT_ID_FROM_ADMIN_ROLES_WHERE_NAME_TEMPORARY_ROLE,
        )
        .fetch_one(&pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_1E53A0C7);
        let assign_role_body = serde_json::to_string(
            &server_admin_contract::admin_set_user_roles_req::AdminSetUserRolesReq::new(
                crate::empty_admin_role_ids(),
                crate::one_admin_role_id(
                    server_admin_contract::admin_role_id::AdminRoleId::try_from(role_id)
                        .expect(constants_str::DIAGNOSTIC_A82FC2E5),
                ),
            ),
        )
        .expect(constants_str::DIAGNOSTIC_BF02E516);
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
        .expect(constants_str::DIAGNOSTIC_F74095EB);
        assert_eq!(assign_role_response.status(), http::StatusCode::NO_CONTENT);
        let stale_role_body = serde_json::to_string(
            &server_admin_contract::admin_set_user_roles_req::AdminSetUserRolesReq::new(
                crate::empty_admin_role_ids(),
                crate::empty_admin_role_ids(),
            ),
        )
        .expect(constants_str::DIAGNOSTIC_1FD845D3);
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
        .expect(constants_str::DIAGNOSTIC_170158FB);
        assert_eq!(stale_role_response.status(), http::StatusCode::CONFLICT);
        let remove_role_body = serde_json::to_string(
            &server_admin_contract::admin_set_user_roles_req::AdminSetUserRolesReq::new(
                crate::one_admin_role_id(
                    server_admin_contract::admin_role_id::AdminRoleId::try_from(role_id)
                        .expect(constants_str::DIAGNOSTIC_C8994C27),
                ),
                crate::empty_admin_role_ids(),
            ),
        )
        .expect(constants_str::DIAGNOSTIC_23C416A1);
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
        .expect(constants_str::DIAGNOSTIC_A895D91F);
        assert_eq!(remove_role_response.status(), http::StatusCode::NO_CONTENT);
        let update_role_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::PATCH),
                super::StdAdminApiTestStrRef::from(format!("/roles/{role_id}").as_str()),
                super::StdAdminApiTestStrRef::from(constants_str::NAME_RENAMED_ROLE),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_4F08B7EC);
        assert_eq!(update_role_response.status(), http::StatusCode::NO_CONTENT);
        let delete_role_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::DELETE),
                super::StdAdminApiTestStrRef::from(format!("/roles/{role_id}").as_str()),
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_D7E1862C);
        assert_eq!(delete_role_response.status(), http::StatusCode::NO_CONTENT);
        let delete_user_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::DELETE),
                super::StdAdminApiTestStrRef::from(format!("/users/{limited_id}").as_str()),
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_C19BE784);
        assert_eq!(delete_user_response.status(), http::StatusCode::NO_CONTENT);
        let admin_role_id =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL)
                .fetch_one(&pool.0)
                .await
                .expect(constants_str::DIAGNOSTIC_20B5FB03);
        let remove_last_admin_role_body = serde_json::to_string(
            &server_admin_contract::admin_set_user_roles_req::AdminSetUserRolesReq::new(
                crate::one_admin_role_id(
                    server_admin_contract::admin_role_id::AdminRoleId::try_from(admin_role_id)
                        .expect(constants_str::DIAGNOSTIC_84FE96C8),
                ),
                crate::empty_admin_role_ids(),
            ),
        )
        .expect(constants_str::DIAGNOSTIC_1528B0D3);
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
        .expect(constants_str::DIAGNOSTIC_FE0DB65C);
        assert_eq!(
            remove_last_admin_role_response.status(),
            http::StatusCode::CONFLICT
        );
        let last_admin_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::DELETE),
                super::StdAdminApiTestStrRef::from(format!("/users/{admin_id}").as_str()),
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_E6175D82);
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
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_8103CD5F);
        assert_eq!(audit_response.status(), http::StatusCode::OK);
        let audit_page = axum::body::to_bytes(
            audit_response.into_body(),
            constants_usize::VALUE_1_048_576,
        )
        .await
        .map(|body| {
            serde_json::from_slice::<server_admin_contract::admin_audit_page::AdminAuditPage>(&body)
                .expect(constants_str::DIAGNOSTIC_ED125D4A)
        })
        .expect(constants_str::DIAGNOSTIC_50612A4D);
        assert!(audit_page.items().len() <= constants_usize::ONE);
        assert!(
            u64::from(audit_page.total())
                >= u64::try_from(audit_page.items().len())
                    .expect(constants_str::DIAGNOSTIC_03C133E9)
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
                        super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                        Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                        None,
                    )
                    .0,
                )
                .await
                .expect(constants_str::DIAGNOSTIC_A6FA9AEB);
                assert_eq!(response.status(), http::StatusCode::OK);
            },
        )
        .await;

        let sessions_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(constants_str::VALUE_9B6938A5),
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_449BF918);
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
            .expect(constants_str::DIAGNOSTIC_E544366C)
        })
        .expect(constants_str::DIAGNOSTIC_141DDCDC);
        assert!(sessions_page.items().len() <= constants_usize::ONE);
        assert!(
            u64::from(sessions_page.total())
                >= u64::try_from(sessions_page.items().len())
                    .expect(constants_str::DIAGNOSTIC_701A7A79)
        );

        let data_table_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(constants_str::VALUE_8F292E26),
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_CA94AEC1);
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
                .expect(constants_str::DIAGNOSTIC_E16283F4)
        })
        .expect(constants_str::DIAGNOSTIC_3F927581);
        assert!(data_table.items().len() <= constants_usize::ONE);
        assert!(
            u64::from(data_table.total())
                >= u64::try_from(data_table.items().len())
                    .expect(constants_str::DIAGNOSTIC_1440730F)
        );
        let filtered_data_table_response = tower::ServiceExt::oneshot(
        crate::router_with_pool(&pool).0,
        crate::request_with_peer(
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                format!(
                    "/tables/users?filter_field=login&filter_operation=eq&filter_value={}&limit=20&offset=0",
                    constants_str::ADMIN_ALT
                )
                .as_str(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
            Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
            None,
        )
        .0,
    )
    .await
    .expect(constants_str::DIAGNOSTIC_766F5654);
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
                .expect(constants_str::DIAGNOSTIC_02D611AB)
        })
        .expect(constants_str::DIAGNOSTIC_6DFE8F37);
        assert_eq!(u64::from(filtered_data_table.total()), 1u64);
        assert_eq!(filtered_data_table.items().len(), constants_usize::ONE);
        assert!(
            filtered_data_table
                .items()
                .first()
                .expect(constants_str::DIAGNOSTIC_753FA97C)
                .values()
                .iter()
                .any(|value| value.as_ref() == constants_str::ADMIN_ALT)
        );
        let empty_data_table_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(constants_str::VALUE_2C93E406),
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_1310E021);
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
                .expect(constants_str::DIAGNOSTIC_AA8376D3)
        })
        .expect(constants_str::DIAGNOSTIC_A98D6360);
        assert_eq!(u64::from(empty_data_table.total()), constants_u64::ZERO);
        assert!(empty_data_table.items().is_empty());
        let unsupported_filter_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(constants_str::VALUE_946CA218),
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_DD6D2544);
        assert_eq!(
            unsupported_filter_response.status(),
            http::StatusCode::UNPROCESSABLE_ENTITY
        );
        let incomplete_filter_response = tower::ServiceExt::oneshot(
            crate::router_with_pool(&pool).0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(constants_str::VALUE_5E6D79D4),
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_E9279B1F);
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
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                Some(super::StdAdminApiTestStrRef::from(
                    refreshed_csrf.0.as_str(),
                )),
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_EF71E50A);
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
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                Some(super::StdAdminApiTestStrRef::from(active_cookie.as_str())),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_54B9DC03);
        assert_eq!(revoked_response.status(), http::StatusCode::UNAUTHORIZED);
        let audit_outcomes = sqlx::query_as::<_, (bool, i64)>(constants_str::SELECT_SUCCEEDED_COUNT_ASTERISK_FROM_ADMIN_AUDIT_LOG_GROUP_BY_SUCCEEDED_ORDER)
        .fetch_all(&pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_3DE105A4);
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
mod test_html {
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_postgresql_html_users_crud_covers_every_frontend_field_separately() {
        let fixture = crate::admin_html_test_fixture().await;
        assert!(fixture.cookie.0.contains(fixture.csrf.0.as_str()));
        let login = constants_str::VALUE_2562E0C2;
        let updated_login = constants_str::VALUE_A582339C;
        let display_name = constants_str::VALUE_79B22AC4;
        let updated_display_name = constants_str::VALUE_8AE21450;
        let password = constants_str::VALUE_4EDBB68D;
        let updated_password = constants_str::VALUE_B6F4A0C4;
        let create_body = super::AdminHtmlTestFormBody::try_from(format!(
            "login={login}&display_name=HTML+CRUD+User&password={password}"
        ))
        .expect(constants_str::DIAGNOSTIC_801D9A43);
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
        let created =
            sqlx::query_as::<_, (i64, String, String, bool)>(constants_str::VALUE_1B03D1AA)
                .bind(login)
                .fetch_one(&fixture.pool.0)
                .await
                .expect(constants_str::DIAGNOSTIC_5DE4FC12);
        assert_eq!(created.1, login);
        assert_eq!(created.2, display_name);
        assert!(!created.3);
        let users_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(users_response.status(), http::StatusCode::OK);
        let users_html = crate::admin_html_body(users_response).await;
        crate::assert_admin_csr_shell(&users_html);

        let login_update_body = super::AdminHtmlTestFormBody::try_from(format!(
            "user_id={}&login={updated_login}&display_name=HTML+CRUD+User",
            created.0
        ))
        .expect(constants_str::DIAGNOSTIC_B0714F29);
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
        let login_update = sqlx::query_as::<_, (String, String)>(constants_str::VALUE_56386809)
            .bind(created.0)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_68FAE270);
        assert_eq!(
            login_update,
            (updated_login.to_owned(), display_name.to_owned())
        );

        let display_update_body = super::AdminHtmlTestFormBody::try_from(format!(
            "user_id={}&login={updated_login}&display_name=HTML+CRUD+User+Updated",
            created.0
        ))
        .expect(constants_str::DIAGNOSTIC_9A6EB324);
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
        let display_update = sqlx::query_as::<_, (String, String)>(constants_str::VALUE_56386809)
            .bind(created.0)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_10DF386A);
        assert_eq!(
            display_update,
            (updated_login.to_owned(), updated_display_name.to_owned())
        );

        let password_update_body = super::AdminHtmlTestFormBody::try_from(format!(
            "user_id={}&password={updated_password}",
            created.0
        ))
        .expect(constants_str::DIAGNOSTIC_CD82F375);
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
        let old_sign_in_body = super::AdminHtmlTestFormBody::try_from(format!(
            "login={updated_login}&password={password}"
        ))
        .expect(constants_str::DIAGNOSTIC_8C42D7E1);
        let old_sign_in_response = tower::ServiceExt::oneshot(
            fixture.router.0.clone(),
            crate::html_request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    server_admin_contract::admin_html_action::AdminHtmlAction::SignIn.get(),
                ),
                super::StdAdminApiTestStrRef::from(old_sign_in_body.0.as_str()),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_26AB3584);
        assert_eq!(
            old_sign_in_response.status(),
            http::StatusCode::UNAUTHORIZED
        );
        let new_sign_in_body = super::AdminHtmlTestFormBody::try_from(format!(
            "login={updated_login}&password={updated_password}"
        ))
        .expect(constants_str::DIAGNOSTIC_EF05A691);
        let new_sign_in_response = tower::ServiceExt::oneshot(
            fixture.router.0.clone(),
            crate::html_request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    server_admin_contract::admin_html_action::AdminHtmlAction::SignIn.get(),
                ),
                super::StdAdminApiTestStrRef::from(new_sign_in_body.0.as_str()),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_B9306C2E);
        assert_eq!(new_sign_in_response.status(), http::StatusCode::SEE_OTHER);

        let role_id =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL)
                .fetch_one(&fixture.pool.0)
                .await
                .expect(constants_str::DIAGNOSTIC_F1674AB9);
        let roles_update_body = super::AdminHtmlTestFormBody::try_from(format!(
            "user_id={}&expected_role_ids=&role_{role_id}={role_id}",
            created.0
        ))
        .expect(constants_str::DIAGNOSTIC_410E6A8C);
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
        let assigned_roles = sqlx::query_scalar::<_, i64>(constants_str::VALUE_4616DD96)
            .bind(created.0)
            .fetch_all(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_739CB4F5);
        assert_eq!(assigned_roles, [role_id]);

        let ban_body =
            super::AdminHtmlTestFormBody::try_from(format!("user_id={}&is_banned=true", created.0))
                .expect(constants_str::DIAGNOSTIC_A17FDC64);
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
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        let final_users_html = crate::admin_html_body(final_users_response).await;
        crate::assert_admin_csr_shell(&final_users_html);
        let unban_body = super::AdminHtmlTestFormBody::try_from(format!(
            "user_id={}&is_banned=false",
            created.0
        ))
        .expect(constants_str::DIAGNOSTIC_9D304DB3);
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
        let is_banned = sqlx::query_scalar::<_, bool>(constants_str::VALUE_A65908E0)
            .bind(created.0)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_55208887);
        assert!(!is_banned);
        let roles_clear_body = super::AdminHtmlTestFormBody::try_from(format!(
            "user_id={}&expected_role_ids={role_id}",
            created.0
        ))
        .expect(constants_str::DIAGNOSTIC_04B638DC);
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

        let delete_body = super::AdminHtmlTestFormBody::try_from(format!(
            "user_id={}&confirmation=true",
            created.0
        ))
        .expect(constants_str::DIAGNOSTIC_D4FE3069);
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
        let deleted_count = sqlx::query_scalar::<_, i64>(constants_str::VALUE_ED81ED3A)
            .bind(created.0)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_72C950EA);
        assert_eq!(deleted_count, constants_i64::ZERO);
        let deleted_users_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        let deleted_users_html = crate::admin_html_body(deleted_users_response).await;
        crate::assert_admin_csr_shell(&deleted_users_html);
        fixture
            .lock
            .0
            .rollback()
            .await
            .expect(constants_str::DIAGNOSTIC_93DB561A);
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_postgresql_html_roles_crud_covers_every_frontend_field_separately() {
        let fixture = crate::admin_html_test_fixture().await;
        let role_name = constants_str::VALUE_B20522BC;
        let updated_role_name = constants_str::VALUE_C940BA4C;
        let create_body = super::AdminHtmlTestFormBody::try_from(format!("name={role_name}"))
            .expect(constants_str::DIAGNOSTIC_C593E840);
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
        let created = sqlx::query_as::<_, (i64, String, bool)>(constants_str::VALUE_96DFAB96)
            .bind(role_name)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_196FBD27);
        assert_eq!(created.1, role_name);
        assert!(!created.2);
        let roles_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(roles_response.status(), http::StatusCode::OK);
        let roles_html = crate::admin_html_body(roles_response).await;
        crate::assert_admin_csr_shell(&roles_html);

        let update_body = super::AdminHtmlTestFormBody::try_from(format!(
            "role_id={}&name={updated_role_name}",
            created.0
        ))
        .expect(constants_str::DIAGNOSTIC_7EA84503);
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
        let updated = sqlx::query_scalar::<_, String>(constants_str::VALUE_59A3D59A)
            .bind(created.0)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_43F81D69);
        assert_eq!(updated, updated_role_name);

        let permission = sqlx::query_as::<_, (i64, String)>(constants_str::VALUE_F3C2734E)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_BA920F54);
        let permissions_body = super::AdminHtmlTestFormBody::try_from(format!(
            "role_id={}&expected_permission_ids=&permission_{}={}",
            created.0, permission.0, permission.0
        ))
        .expect(constants_str::DIAGNOSTIC_0D476C31);
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
        let assigned_permissions = sqlx::query_scalar::<_, i64>(constants_str::VALUE_5FE3480D)
            .bind(created.0)
            .fetch_all(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_82B0D9F3);
        assert_eq!(assigned_permissions, [permission.0]);
        let final_roles_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        let final_roles_html = crate::admin_html_body(final_roles_response).await;
        crate::assert_admin_csr_shell(&final_roles_html);

        let delete_body = super::AdminHtmlTestFormBody::try_from(format!(
            "role_id={}&confirmation=true",
            created.0
        ))
        .expect(constants_str::DIAGNOSTIC_E1547A60);
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
        let deleted_count = sqlx::query_scalar::<_, i64>(constants_str::VALUE_D4A7F1E9)
            .bind(created.0)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_2DB479F8);
        assert_eq!(deleted_count, constants_i64::ZERO);
        let deleted_roles_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Roles.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        let deleted_roles_html = crate::admin_html_body(deleted_roles_response).await;
        crate::assert_admin_csr_shell(&deleted_roles_html);
        fixture
            .lock
            .0
            .rollback()
            .await
            .expect(constants_str::DIAGNOSTIC_674DC2A9);
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_postgresql_html_settings_updates_and_reads_every_field_separately() {
        let fixture = crate::admin_html_test_fixture().await;
        let site_name_a = super::StdAdminApiTestStrRef::from(constants_str::VALUE_98A13EB2);
        let site_name_b = super::StdAdminApiTestStrRef::from(constants_str::VALUE_ABCC7908);
        let route_a = super::StdAdminApiTestStrRef::from(
            server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
        );
        let route_b = super::StdAdminApiTestStrRef::from(constants_str::VALUE_DB2C56E6);
        let tab_title_a = super::StdAdminApiTestStrRef::from(constants_str::VALUE_F7D2459A);
        let tab_title_b = super::StdAdminApiTestStrRef::from(constants_str::VALUE_74AF8A89);
        let main_logo_a = super::StdAdminApiTestStrRef::from(constants_str::VALUE_2C8B94AD);
        let main_logo_b = super::StdAdminApiTestStrRef::from(constants_str::VALUE_91EAC748);
        let primary_color_a = super::StdAdminApiTestStrRef::from(constants_str::VALUE_CD527CD2);
        let primary_color_b = super::StdAdminApiTestStrRef::from(constants_str::VALUE_3CFDA7DC);
        let organization_name_a = super::StdAdminApiTestStrRef::from(constants_str::VALUE_DA7C4DC3);
        let organization_name_b = super::StdAdminApiTestStrRef::from(constants_str::VALUE_4918294B);
        let organization_contacts_a =
            super::StdAdminApiTestStrRef::from(constants_str::VALUE_2AFAD82D);
        let organization_contacts_b =
            super::StdAdminApiTestStrRef::from(constants_str::VALUE_E7FDD028);
        let support_url_a = super::StdAdminApiTestStrRef::from(constants_str::VALUE_AB22006C);
        let support_url_b = super::StdAdminApiTestStrRef::from(constants_str::VALUE_4D525EFD);
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
        futures::StreamExt::fold(futures::stream::iter(states), (), async |(), values| {
            let form_body = values.form_body();
            let update_response = crate::admin_html_response(
                &fixture,
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    server_admin_contract::admin_html_action::AdminHtmlAction::SettingsUpdate.get(),
                ),
                super::StdAdminApiTestStrRef::from(form_body.0.as_str()),
            )
            .await;
            assert_eq!(update_response.status(), http::StatusCode::SEE_OTHER);
            let read_response = crate::admin_html_response(
                &fixture,
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(
                    server_admin_contract::admin_frontend_path::AdminFrontendPath::Settings.get(),
                ),
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
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
        >(constants_str::VALUE_F1866337)
        .fetch_one(&fixture.pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_A8F201DE);
        assert_eq!(stored.0, site_name_b.0);
        assert_eq!(stored.1, route_b.0);
        assert_eq!(stored.2, tab_title_b.0);
        assert_eq!(stored.3, main_logo_b.0);
        assert_eq!(stored.4, primary_color_b.0);
        assert_eq!(stored.5, organization_name_b.0);
        assert_eq!(stored.6, organization_contacts_b.0);
        assert_eq!(stored.7, support_url_b.0);
        let empty = super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX);
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
                    &fixture,
                    super::HttpAdminApiTestMethod::from(http::Method::POST),
                    super::StdAdminApiTestStrRef::from(
                        server_admin_contract::admin_html_action::AdminHtmlAction::SettingsUpdate
                            .get(),
                    ),
                    super::StdAdminApiTestStrRef::from(form_body.0.as_str()),
                )
                .await;
                assert_eq!(clear_response.status(), http::StatusCode::SEE_OTHER);
                let optional_values = sqlx::query_as::<
                    _,
                    (String, String, String, String, String, String),
                >(constants_str::VALUE_8CB85C2C)
                .fetch_one(&fixture.pool.0)
                .await
                .expect(constants_str::DIAGNOSTIC_D418F9C0);
                assert_eq!(
                    [
                        (optional_values.0.as_str(), constants_str::ADMIN,),
                        (
                            optional_values.1.as_str(),
                            constants_str::ADMIN_DEFAULT_MAIN_LOGO,
                        ),
                        (
                            optional_values.2.as_str(),
                            constants_str::PRIMARY_COLOR_DEFAULT,
                        ),
                        (optional_values.3.as_str(), constants_str::ADMIN,),
                        (
                            optional_values.4.as_str(),
                            constants_str::ADMIN_DEFAULT_ORGANIZATION_CONTACTS,
                        ),
                        (
                            optional_values.5.as_str(),
                            constants_str::ADMIN_DEFAULT_SUPPORT_URL,
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
        fixture
            .lock
            .0
            .rollback()
            .await
            .expect(constants_str::DIAGNOSTIC_C7659B40);
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_postgresql_initial_administrator_password_must_change_before_admin_access() {
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
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
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
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(profile_response.status(), http::StatusCode::OK);
        let correct_password = serde_json::from_str::<String>(constants_str::CORRECT_PASSWORD)
            .expect(constants_str::DIAGNOSTIC_E20A72A8);
        let change_password_body = super::AdminHtmlTestFormBody::try_from(format!(
            "current_password={correct_password}&new_password=Initial-administrator-changed-pass2",
        ))
        .expect(constants_str::DIAGNOSTIC_B42A390D);
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
            constants_str::SELECT_MUST_CHANGE_PASSWORD_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
        )
        .fetch_one(&fixture.pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_EA57FC2D);
        assert!(!password_change_required);
        let post_change_users_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(post_change_users_response.status(), http::StatusCode::OK);
        fixture
            .lock
            .0
            .rollback()
            .await
            .expect(constants_str::DIAGNOSTIC_6A8CE0F3);
    }

    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_postgresql_html_profile_reads_every_field_and_changes_own_password() {
        let fixture = crate::admin_html_test_fixture().await;
        let profile_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Profile.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(profile_response.status(), http::StatusCode::OK);
        let profile_html = crate::admin_html_body(profile_response).await;
        crate::assert_admin_csr_shell(&profile_html);

        let original_password_hash = sqlx::query_scalar::<_, String>(
            constants_str::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
        )
        .fetch_one(&fixture.pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_C09B5E4E);
        let (current_session_id, user_id) =
            sqlx::query_as::<_, (uuid::Uuid, i64)>(constants_str::VALUE_9605FF41)
                .fetch_one(&fixture.pool.0)
                .await
                .expect(constants_str::DIAGNOSTIC_AE46B7C1);
        let other_session_id = uuid::Uuid::from_u128(2u128);
        let _inserted_other_session = sqlx::query(constants_str::VALUE_324717BB)
            .bind(other_session_id)
            .bind(user_id)
            .execute(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_3E216ECD);
        let _inserted_other_refresh_token = sqlx::query(constants_str::VALUE_0FCC992D)
            .bind(uuid::Uuid::from_u128(3u128))
            .bind(user_id)
            .execute(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_D61FC342);
        let correct_password = serde_json::from_str::<String>(constants_str::CORRECT_PASSWORD)
            .expect(constants_str::DIAGNOSTIC_C59B011A);
        let change_password_body = super::AdminHtmlTestFormBody::try_from(format!(
            "current_password={correct_password}&new_password=Html-profile-pass2",
        ))
        .expect(constants_str::DIAGNOSTIC_C93D69E3);
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
            constants_str::SELECT_PASSWORD_HASH_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
        )
        .fetch_one(&fixture.pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_696330CA);
        assert_ne!(changed_password_hash, original_password_hash);
        let current_session_revoked = sqlx::query_scalar::<_, bool>(constants_str::VALUE_26E35E53)
            .bind(current_session_id)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_38923E84);
        assert!(!current_session_revoked);
        let other_session_revoked = sqlx::query_scalar::<_, bool>(constants_str::VALUE_26E35E53)
            .bind(other_session_id)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_F0168DC5);
        assert!(other_session_revoked);
        let active_refresh_token_count =
            sqlx::query_scalar::<_, i64>(constants_str::VALUE_52BB5B18)
                .fetch_one(&fixture.pool.0)
                .await
                .expect(constants_str::DIAGNOSTIC_740D6DC9);
        assert_eq!(active_refresh_token_count, constants_i64::ZERO);
        let authenticated_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Profile.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(authenticated_response.status(), http::StatusCode::OK);
        fixture
            .lock
            .0
            .rollback()
            .await
            .expect(constants_str::DIAGNOSTIC_737BBBE6);
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_postgresql_html_sessions_reads_every_field_and_revokes_session() {
        let fixture = crate::admin_html_test_fixture().await;
        let admin_id = sqlx::query_scalar::<_, i64>(
            constants_str::SELECT_ID_FROM_ADMIN_USERS_WHERE_LOGIN_ADMIN,
        )
        .fetch_one(&fixture.pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_7F0A7C64);
        let (session_id, _created_at, _expires_at) =
            sqlx::query_as::<_, (uuid::Uuid, String, String)>(
                constants_str::SERVER_ADMIN_LIST_ACTIVE_SESSIONS_SQL,
            )
            .bind(admin_id)
            .bind(100i64)
            .bind(constants_i64::ZERO)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_32E44A86);
        let sessions_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Sessions.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(sessions_response.status(), http::StatusCode::OK);
        let sessions_html = crate::admin_html_body(sessions_response).await;
        crate::assert_admin_csr_shell(&sessions_html);

        let revoke_body = super::AdminHtmlTestFormBody::try_from(format!(
            "session_id={session_id}&confirmation=true"
        ))
        .expect(constants_str::DIAGNOSTIC_2F8BEA59);
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
        let revoked = sqlx::query_scalar::<_, bool>(constants_str::VALUE_26E35E53)
            .bind(session_id)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_E443902E);
        assert!(revoked);
        let rejected_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Sessions.get(),
            ),
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(rejected_response.status(), http::StatusCode::SEE_OTHER);
        fixture
            .lock
            .0
            .rollback()
            .await
            .expect(constants_str::DIAGNOSTIC_9F41B8BD);
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_postgresql_html_router_registers_every_owned_page_and_action() {
        let fixture = crate::admin_html_test_fixture().await;
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
                    &fixture,
                    super::HttpAdminApiTestMethod::from(http::Method::GET),
                    super::StdAdminApiTestStrRef::from(path.get()),
                    super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
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
                    &fixture,
                    super::HttpAdminApiTestMethod::from(http::Method::GET),
                    super::StdAdminApiTestStrRef::from(uri.as_ref()),
                    super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
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
                    fixture.router.0.clone(),
                    crate::html_request_with_peer(
                        super::HttpAdminApiTestMethod::from(http::Method::POST),
                        super::StdAdminApiTestStrRef::from(action.get()),
                        super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                        None,
                    )
                    .0,
                )
                .await
                .expect(constants_str::DIAGNOSTIC_D9567273);
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
        fixture
            .lock
            .0
            .rollback()
            .await
            .expect(constants_str::DIAGNOSTIC_C0C53CDC);
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_postgresql_html_crud_forms_enforce_auth_csrf_validation_conflict_and_filtering() {
        let fixture = crate::admin_html_test_fixture().await;
        let unauthenticated_response = tower::ServiceExt::oneshot(
            fixture.router.0.clone(),
            crate::html_request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::GET),
                super::StdAdminApiTestStrRef::from(
                    server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
                ),
                super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_184EC7B2);
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

        let login = constants_str::VALUE_0E3DA187;
        let valid_body = super::AdminHtmlTestFormBody::try_from(format!(
            "login={login}&display_name=HTML+Form+Contract+User&password=Html-form-pass1"
        ))
        .expect(constants_str::DIAGNOSTIC_94B36EC1);
        let missing_csrf_response = tower::ServiceExt::oneshot(
            fixture.router.0.clone(),
            crate::html_request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    server_admin_contract::admin_html_action::AdminHtmlAction::UserCreate.get(),
                ),
                super::StdAdminApiTestStrRef::from(valid_body.0.as_str()),
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_E6013D7A);
        assert_eq!(missing_csrf_response.status(), http::StatusCode::FORBIDDEN);
        let unknown_field_body =
            super::AdminHtmlTestFormBody::try_from(format!("{}&unknown_field=true", valid_body.0))
                .expect(constants_str::DIAGNOSTIC_AF2948D3);
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
        let created_id = sqlx::query_scalar::<_, i64>(constants_str::VALUE_A2A63B95)
            .bind(login)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_378A4E50);
        let filtered_path = super::AdminHtmlTestFormBody::try_from(format!(
            "{}?search={login}",
            server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get()
        ))
        .expect(constants_str::DIAGNOSTIC_60BF2C91);
        let filtered_response = crate::admin_html_response(
            &fixture,
            super::HttpAdminApiTestMethod::from(http::Method::GET),
            super::StdAdminApiTestStrRef::from(filtered_path.0.as_str()),
            super::StdAdminApiTestStrRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX),
        )
        .await;
        assert_eq!(filtered_response.status(), http::StatusCode::OK);
        let filtered_html = crate::admin_html_body(filtered_response).await;
        crate::assert_admin_csr_shell(&filtered_html);

        let role_id =
            sqlx::query_scalar::<_, i64>(constants_str::SERVER_ADMIN_READ_ADMIN_ROLE_ID_SQL)
                .fetch_one(&fixture.pool.0)
                .await
                .expect(constants_str::DIAGNOSTIC_BC10A764);
        let stale_roles_body = super::AdminHtmlTestFormBody::try_from(format!(
            "user_id={created_id}&expected_role_ids={role_id}"
        ))
        .expect(constants_str::DIAGNOSTIC_1934AD6F);
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

        let role_name = constants_str::VALUE_F9B1D97F;
        let create_role_body = super::AdminHtmlTestFormBody::try_from(format!("name={role_name}"))
            .expect(constants_str::DIAGNOSTIC_8CF4260D);
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
        let created_role_id = sqlx::query_scalar::<_, i64>(constants_str::VALUE_44E1D290)
            .bind(role_name)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_2643BE19);
        let permission_id = sqlx::query_scalar::<_, i64>(constants_str::VALUE_1491D3FA)
            .fetch_one(&fixture.pool.0)
            .await
            .expect(constants_str::DIAGNOSTIC_D8134C5B);
        let stale_permissions_body = super::AdminHtmlTestFormBody::try_from(format!(
            "role_id={created_role_id}&expected_permission_ids={permission_id}"
        ))
        .expect(constants_str::DIAGNOSTIC_49FAC702);
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
        let delete_role_body = super::AdminHtmlTestFormBody::try_from(format!(
            "role_id={created_role_id}&confirmation=true"
        ))
        .expect(constants_str::DIAGNOSTIC_F1C637D8);
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

        let unknown_delete_body =
            super::AdminHtmlTestFormBody::try_from(String::from(constants_str::VALUE_8F942A25))
                .expect(constants_str::DIAGNOSTIC_D96B20E4);
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

        let delete_body = super::AdminHtmlTestFormBody::try_from(format!(
            "user_id={created_id}&confirmation=true"
        ))
        .expect(constants_str::DIAGNOSTIC_4CF9072D);
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
        fixture
            .lock
            .0
            .rollback()
            .await
            .expect(constants_str::DIAGNOSTIC_7361EB5C);
    }
}
mod test_maintenance {
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_postgresql_optimistic_revision_allows_one_concurrent_writer() {
        let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL)
            .expect(constants_str::DIAGNOSTIC_63A09EEC);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4u32)
            .connect(database_url.as_str())
            .await
            .expect(constants_str::DIAGNOSTIC_2480F8C4);
        let _drop_before =
            sqlx::query(constants_str::DROP_TABLE_IF_EXISTS_PG_TABLE_OPTIMISTIC_REVISION_TEST)
                .execute(&pool)
                .await
                .expect(constants_str::DIAGNOSTIC_E5E1F7CB);
        let _create = sqlx::query(constants_str::CREATE_TABLE_PG_TABLE_OPTIMISTIC_REVISION_TEST_ID_BIGINT_PRIMARY_KEY_REVISION)
        .execute(&pool)
        .await
        .expect(constants_str::DIAGNOSTIC_A75BC224);
        let _insert = sqlx::query(
            constants_str::INSERT_INTO_PG_TABLE_OPTIMISTIC_REVISION_TEST_ID_REVISION_VALUE_VALUES_1,
        )
        .execute(&pool)
        .await
        .expect(constants_str::DIAGNOSTIC_DA271038);
        let update = constants_str::UPDATE_PG_TABLE_OPTIMISTIC_REVISION_TEST_SET_VALUE_DOLLAR_1_REVISION_REVISION;
        let (left, right) = tokio::join!(
            sqlx::query_scalar::<_, i64>(update)
                .bind(constants_i64::ONE)
                .bind(
                    pg_table::pg_table_revision::PgTableRevision::try_from(
                        constants_str::VALUE_0.to_owned()
                    )
                    .expect(constants_str::DIAGNOSTIC_979FA4B2)
                )
                .fetch_optional(&pool),
            sqlx::query_scalar::<_, i64>(update)
                .bind(2i64)
                .bind(
                    pg_table::pg_table_revision::PgTableRevision::try_from(
                        constants_str::VALUE_0.to_owned()
                    )
                    .expect(constants_str::DIAGNOSTIC_589EA31D)
                )
                .fetch_optional(&pool),
        );
        let outcomes = [
            left.expect(constants_str::DIAGNOSTIC_A1A1382A),
            right.expect(constants_str::DIAGNOSTIC_8406B933),
        ];
        assert_eq!(
            outcomes.iter().filter(|value| value.is_some()).count(),
            constants_usize::ONE
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>(constants_str::VALUE_5A622B1F,)
                .fetch_one(&pool)
                .await
                .expect(constants_str::DIAGNOSTIC_C0F01A04),
            constants_i64::ONE
        );
        let stale = sqlx::query_scalar::<_, i64>(update)
            .bind(3i64)
            .bind(
                pg_table::pg_table_revision::PgTableRevision::try_from(
                    constants_str::VALUE_0.to_owned(),
                )
                .expect(constants_str::DIAGNOSTIC_A3A08AEB),
            )
            .fetch_optional(&pool)
            .await
            .expect(constants_str::DIAGNOSTIC_964E3EF4);
        assert_eq!(stale, None);
        let _drop_after = sqlx::query(constants_str::DROP_TABLE_PG_TABLE_OPTIMISTIC_REVISION_TEST)
            .execute(&pool)
            .await
            .expect(constants_str::DIAGNOSTIC_A4D77F54);
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_postgresql_cleanup_is_batched_and_preserves_append_only_policy() {
        let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL)
            .expect(constants_str::DIAGNOSTIC_7316CF4D);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(3u32)
            .connect(database_url.as_str())
            .await
            .expect(constants_str::DIAGNOSTIC_F6A51733);
        let mut admin_db_test_lock = pool
            .begin()
            .await
            .expect(constants_str::DIAGNOSTIC_847CAF57);
        let _locked = sqlx::query(constants_str::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
            .execute(&mut *admin_db_test_lock)
            .await
            .expect(constants_str::DIAGNOSTIC_8C298FEF);
        let mut idempotency_test_isolation = pool
            .begin()
            .await
            .expect(constants_str::DIAGNOSTIC_F56C4C85);
        pg_crud_common::lock_pg_relation_resources::lock_pg_relation_resources(
        pg_crud_common::sqlx_pg_relation_lock_connection_ref::SqlxPgRelationLockConnectionRef::from(&mut *idempotency_test_isolation),
        &pg_crud_common::pg_relation_lock_namespace::PgRelationLockNamespace::try_from(constants_str::ACTOR_ATOMIC.to_owned())
            .expect(constants_str::DIAGNOSTIC_861FE23D),
        &pg_crud_common::pg_relation_resource_ids::PgRelationResourceIds::try_from(vec![
            pg_crud_common::pg_relation_resource_id::PgRelationResourceId::from(constants_i64::ONE),
        ])
        .expect(constants_str::DIAGNOSTIC_A18F804C),
    )
    .await
    .expect(constants_str::DIAGNOSTIC_FAB61374);
        server_admin::prepare_postgresql::prepare_postgresql(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_029CB682);
        pg_table::ensure_pg_table_idempotency_schema::ensure_pg_table_idempotency_schema(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_EB08DFFC);
        let _clear = sqlx::query(constants_str::TRUNCATE_ADMIN_ACCESS_SESSIONS_ADMIN_REFRESH_TOKENS_ADMIN_LOGIN_ATTEMPTS_ADMIN_RATE)
        .execute(&pool)
        .await
        .expect(constants_str::DIAGNOSTIC_E1B22572);
        let _attempts = sqlx::query(constants_str::INSERT_INTO_ADMIN_LOGIN_ATTEMPTS_LOGIN_SUCCEEDED_ATTEMPTED_AT_SELECT_OLD_VALUE)
        .execute(&pool)
        .await
        .expect(constants_str::DIAGNOSTIC_480B06EB);
        let _limits = sqlx::query(constants_str::INSERT_INTO_ADMIN_RATE_LIMITS_SCOPE_SUBJECT_WINDOW_STARTED_AT_REQUEST_COUNT_ALT)
        .execute(&pool)
        .await
        .expect(constants_str::DIAGNOSTIC_0375574D);
        let _audit = sqlx::query(
        constants_str::INSERT_INTO_ADMIN_AUDIT_LOG_ACTION_RESOURCE_SUCCEEDED_CREATED_AT_SELECT_TEST,
    )
    .execute(&pool)
    .await
    .expect(constants_str::DIAGNOSTIC_F50EF817);
        let retention =
            server_admin::admin_cleanup_retention_seconds::AdminCleanupRetentionSeconds::try_from(
                3_600i64,
            )
            .expect(constants_str::DIAGNOSTIC_AB892FC5);
        let config = server_admin::admin_cleanup_cfg::AdminCleanupCfg::new(
            server_admin::admin_cleanup_batch_size::AdminCleanupBatchSize::try_from(2i64)
                .expect(constants_str::DIAGNOSTIC_1D97B31C),
            retention,
            retention,
            retention,
            retention,
            retention,
        );
        let report = server_admin::cleanup_admin_tables::cleanup_admin_tables(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool),
            config,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_A422E8D4);
        assert_eq!(
            report.total_rows().to_string(),
            constants_str::VALUE_E7F6C011
        );
        let remaining = sqlx::query_as::<_, (i64, i64, i64)>(constants_str::SELECT_SELECT_COUNT_ASTERISK_FROM_ADMIN_LOGIN_ATTEMPTS_SELECT_COUNT_ASTERISK_FROM)
        .fetch_one(&pool)
        .await
        .expect(constants_str::DIAGNOSTIC_F37A3AB4);
        assert_eq!(
            remaining,
            (constants_i64::ONE, constants_i64::ONE, constants_i64::ONE)
        );
        let ordinary_delete = sqlx::query(constants_str::DELETE_FROM_ADMIN_AUDIT_LOG)
            .execute(&pool)
            .await;
        assert!(matches!(ordinary_delete, Err(_error)));
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_postgresql_migration_creates_complete_schema() {
        let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL)
            .expect(constants_str::DIAGNOSTIC_B65D1786);
        let base_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1u32)
            .connect(database_url.as_str())
            .await
            .expect(constants_str::DIAGNOSTIC_0047F74E);
        let _drop_schema =
            sqlx::raw_sql(constants_str::DROP_SCHEMA_IF_EXISTS_ADMIN_MIGRATION_FRESH_TEST_CASCADE)
                .execute(&base_pool)
                .await
                .expect(constants_str::DIAGNOSTIC_DF91B04D);
        let _create_schema = sqlx::raw_sql(constants_str::CREATE_SCHEMA_ADMIN_MIGRATION_FRESH_TEST)
            .execute(&base_pool)
            .await
            .expect(constants_str::DIAGNOSTIC_02BCD1C2);
        let connect = |schema: super::StdAdminApiTestStrRef<'static>| {
            let options = <sqlx::postgres::PgConnectOptions as std::str::FromStr>::from_str(
                database_url.as_str(),
            )
            .expect(constants_str::DIAGNOSTIC_AA7735DB)
            .options([(constants_str::SEARCH_PATH, schema.0)]);
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(1u32)
                .connect_lazy_with(options)
        };
        let fresh_pool = connect(super::StdAdminApiTestStrRef::from(
            constants_str::ADMIN_MIGRATION_FRESH_TEST,
        ));
        let full = sqlx::migrate!("../server_admin_migrations");
        full.run(&fresh_pool)
            .await
            .expect(constants_str::DIAGNOSTIC_4B6C3BD6);
        server_admin::validate_catalog_schema::validate_catalog_schema(
            pg_crud_common::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef::from(&fresh_pool),
            pg_crud_common::db_schema_name_ref::DbSchemaNameRef::from(
                constants_str::ADMIN_MIGRATION_FRESH_TEST,
            ),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_FAC299AA);
        let catalog_snapshot = pg_crud_common::inspect_postgres_catalog::inspect_postgres_catalog(
            pg_crud_common::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef::from(&fresh_pool),
            pg_crud_common::db_schema_name_ref::DbSchemaNameRef::from(
                constants_str::ADMIN_MIGRATION_FRESH_TEST,
            ),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_518B93E4);
        let table_snapshots = futures::future::try_join_all(
            server_admin_contract::admin_data_table::AdminDataTable::PG_ORDER
                .into_iter()
                .map(async |table| {
                    pg_crud_common::inspect_postgres_table::inspect_postgres_table(
                        pg_crud_common::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef::from(
                            &fresh_pool,
                        ),
                        pg_crud_common::db_schema_name_ref::DbSchemaNameRef::from(
                            constants_str::ADMIN_MIGRATION_FRESH_TEST,
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
        .expect(constants_str::DIAGNOSTIC_34D80F68);
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
            .join(constants_str::ADMIN_CURRENT_SCHEMA_SNAPSHOT_PATH);
        if std::env::var_os(constants_str::UPDATE_ADMIN_CURRENT_SCHEMA_SNAPSHOT).is_some() {
            std::fs::write(
                current_schema_snapshot_path.as_path(),
                current_schema_snapshot.as_bytes(),
            )
            .expect(constants_str::DIAGNOSTIC_ABE4D63F);
        }
        let expected_current_schema_snapshot =
            std::fs::read_to_string(current_schema_snapshot_path)
                .expect(constants_str::DIAGNOSTIC_3AF279E1);
        assert_eq!(
            current_schema_snapshot, expected_current_schema_snapshot,
            "cb6ce4a9 migration-derived PostgreSQL schema snapshot changed"
        );
        let version = sqlx::query_scalar::<_, i64>(
            constants_str::SELECT_MAX_VERSION_FROM_ADMIN_MIGRATION_FRESH_TEST_SQLX_MIGRATIONS_WHERE,
        )
        .fetch_one(&base_pool)
        .await
        .expect(constants_str::DIAGNOSTIC_5C10C931);
        assert_eq!(version, 13i64);
        let expected_tables = server_admin_contract::admin_data_table::AdminDataTable::PG_ORDER
            .map(|table| table.to_string())
            .into_iter()
            .collect::<std::collections::BTreeSet<String>>();
        let fresh_tables = sqlx::query_scalar::<_, String>(
            constants_str::SELECT_TABLE_NAME_FROM_INFORMATION_SCHEMA_TABLES_WHERE_TABLE_SCHEMA,
        )
        .bind(constants_str::ADMIN_MIGRATION_FRESH_TEST)
        .fetch_all(&base_pool)
        .await
        .expect(constants_str::DIAGNOSTIC_AB254FF4)
        .into_iter()
        .collect::<std::collections::BTreeSet<String>>();
        assert_eq!(fresh_tables, expected_tables);
        fresh_pool.close().await;
        let _drop_after =
            sqlx::raw_sql(constants_str::DROP_SCHEMA_ADMIN_MIGRATION_FRESH_TEST_CASCADE)
                .execute(&base_pool)
                .await
                .expect(constants_str::DIAGNOSTIC_88DD90B8);
    }
}
mod test_policy {
    #[test]
    fn test_policy() {
        let read_excluded = <server_admin::admin_users::AdminUsers as pg_crud_common::db_table_schema::DbTableSchema>::read_excluded_columns();
        assert!(
            read_excluded
                .iter()
                .any(|field| field.as_ref() == constants_str::PASSWORD_HASH)
        );
        let create_excluded = <server_admin::admin_users::AdminUsers as pg_crud_common::db_table_schema::DbTableSchema>::create_excluded_columns();
        assert!(
            create_excluded
                .iter()
                .any(|field| field.as_ref() == constants_str::PASSWORD_HASH)
        );
    }
}
mod test_routing {
    #[tokio::test]
    async fn test_protected_routes_reject_missing_authentication_without_database_io() {
        let users_response = tower::ServiceExt::oneshot(
            crate::admin_api_test_router().0,
            http::Request::builder()
                .uri(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_me_route::AdminMeRoute,
                    >()
                    .as_ref(),
                )
                .body(axum::body::Body::empty())
                .expect(constants_str::DIAGNOSTIC_B319E84D),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_0AC617DE);
        assert_eq!(users_response.status(), http::StatusCode::UNAUTHORIZED);
        let response = tower::ServiceExt::oneshot(
            crate::admin_api_test_router().0,
            http::Request::builder()
                .uri(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_list_users_route::AdminListUsersRoute,
                    >()
                    .as_ref(),
                )
                .body(axum::body::Body::empty())
                .expect(constants_str::DIAGNOSTIC_895E12FC),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_1FE80AD3);
        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
    }
    #[tokio::test]
    #[allow(
        clippy::needless_for_each,
        reason = "repository policy requires iterator methods instead of for loops"
    )]
    async fn test_runtime_auth_router_contains_every_open_api_operation() {
        let document = serde_json::to_value(utoipa::openapi::OpenApi::from(
            server_admin::admin_api_open_api::admin_api_open_api(),
        ))
        .expect(constants_str::DIAGNOSTIC_71599514);
        let paths = document
            .get(constants_str::PATHS)
            .and_then(serde_json::Value::as_object)
            .expect(constants_str::DIAGNOSTIC_D908872F);
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
                            constants_str::ADMIN_SESSION_ID_PLACEHOLDER,
                            constants_str::VALUE_1,
                        )
                        .replace(
                            constants_str::ADMIN_USER_ID_PLACEHOLDER,
                            constants_str::VALUE_1,
                        )
                        .replace(
                            constants_str::ADMIN_ROLE_ID_PLACEHOLDER,
                            constants_str::VALUE_1,
                        );
                    let method =
                        http::Method::from_bytes(documented_method.to_ascii_uppercase().as_bytes())
                            .expect(constants_str::DIAGNOSTIC_9D31A7E4);
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
                                    .expect(constants_str::DIAGNOSTIC_A3D6FB65),
                            )
                            .await,
                        )
                    }
                }),
        )
        .await;
        responses.into_iter().for_each(|(method, path, response)| {
            let status = response.expect(constants_str::DIAGNOSTIC_F7BD9F15).status();
            assert!(
                status != http::StatusCode::METHOD_NOT_ALLOWED
                    && status != http::StatusCode::NOT_FOUND,
                "runtime router does not expose documented operation {method} {path}"
            );
        });
    }
    #[tokio::test]
    async fn test_invalid_access_cookie_is_rejected_before_database_io() {
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
                    constants_str::ADMIN_ACCESS_TOKEN_INVALID_JWT_TOKEN,
                )
                .body(axum::body::Body::empty())
                .expect(constants_str::DIAGNOSTIC_819ACD53),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_C3AF0891);
        assert_eq!(response.status(), http::StatusCode::UNAUTHORIZED);
    }
    #[tokio::test]
    async fn test_unknown_admin_api_route_is_not_captured_by_spa_fallback() {
        let response = tower::ServiceExt::oneshot(
            crate::admin_api_test_router().0,
            http::Request::builder()
                .uri(constants_str::NOT_AN_API_ROUTE)
                .body(axum::body::Body::empty())
                .expect(constants_str::DIAGNOSTIC_1CA76F8D),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_CE417390);
        assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
    }
    #[tokio::test]
    async fn test_wrong_admin_http_method_uses_problem_details_contract() {
        let response = tower::ServiceExt::oneshot(
            crate::admin_api_test_router().0,
            http::Request::builder()
                .method(http::Method::GET)
                .uri(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_sign_in_route::AdminSignInRoute,
                    >()
                    .as_ref(),
                )
                .body(axum::body::Body::empty())
                .expect(constants_str::DIAGNOSTIC_4EB1C098),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_6764152A);
        assert_eq!(response.status(), http::StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(
            response.headers().get(http::header::CONTENT_TYPE),
            Some(&http::HeaderValue::from_static(
                constants_str::APPLICATION_PROBLEM_PLUS_JSON
            )),
        );
    }
    #[tokio::test]
    async fn test_invalid_admin_json_uses_problem_details_and_body_limit_contract() {
        let malformed_response = tower::ServiceExt::oneshot(
            crate::admin_api_test_router().0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_sign_in_route::AdminSignInRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(constants_str::LOGIN_ALT),
                None,
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_5FB0627D);
        assert_eq!(
            malformed_response.status(),
            http::StatusCode::UNPROCESSABLE_ENTITY
        );
        assert_eq!(
            malformed_response.headers().get(http::header::CONTENT_TYPE),
            Some(&http::HeaderValue::from_static(
                constants_str::APPLICATION_PROBLEM_PLUS_JSON
            )),
        );
        let body_limit = <server_admin_contract::admin_route::AdminAuthenticationRouteFamily as frontend_contract::route_family::RouteFamily>::body_limit()
        .expect(constants_str::DIAGNOSTIC_A60751DB)
        .get();
        let oversized_password =
            constants_str::X.repeat(body_limit.saturating_add(constants_usize::ONE));
        let oversized_body = format!(r#"{{"login":"admin","password":"{oversized_password}"}}"#);
        let oversized_response = tower::ServiceExt::oneshot(
            crate::admin_api_test_router().0,
            crate::request_with_peer(
                super::HttpAdminApiTestMethod::from(http::Method::POST),
                super::StdAdminApiTestStrRef::from(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_sign_in_route::AdminSignInRoute,
                    >()
                    .as_ref(),
                ),
                super::StdAdminApiTestStrRef::from(oversized_body.as_str()),
                None,
                None,
            )
            .0,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_FCD3DD3F);
        assert_eq!(
            oversized_response.status(),
            http::StatusCode::PAYLOAD_TOO_LARGE
        );
        assert_eq!(
            oversized_response.headers().get(http::header::CONTENT_TYPE),
            Some(&http::HeaderValue::from_static(
                constants_str::APPLICATION_PROBLEM_PLUS_JSON
            )),
        );
    }
    #[tokio::test]
    async fn test_sign_in_requires_trusted_origin_without_database_io() {
        let make_request = |origin, referer| {
            let mut builder = http::Request::builder()
                .method(http::Method::POST)
                .uri(
                    frontend_contract::typed_route_path::typed_route_path::<
                        server_admin_contract::admin_sign_in_route::AdminSignInRoute,
                    >()
                    .as_ref(),
                )
                .header(http::header::CONTENT_TYPE, constants_str::APPLICATION_JSON);
            if let Some(value) = origin {
                builder = builder.header(http::header::ORIGIN, value);
            }
            if let Some(value) = referer {
                builder = builder.header(http::header::REFERER, value);
            }
            let mut request = builder
                .body(axum::body::Body::from(
                    constants_str::LOGIN_ADMIN_PASSWORD_PASSWORD,
                ))
                .expect(constants_str::DIAGNOSTIC_168060A3);
            let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
                constants_str::VALUE_127_0_0_1_43210
                    .parse::<std::net::SocketAddr>()
                    .expect(constants_str::DIAGNOSTIC_C90CBA14),
            ));
            request
        };
        let missing_origin_response =
            tower::ServiceExt::oneshot(crate::admin_api_test_router().0, make_request(None, None))
                .await
                .expect(constants_str::DIAGNOSTIC_ED2F56FB);
        assert_eq!(
            missing_origin_response.status(),
            http::StatusCode::UNAUTHORIZED
        );
        let blocked_origin_response = tower::ServiceExt::oneshot(
            crate::admin_api_test_router().0,
            make_request(
                Some(constants_str::HTTP_BLOCKED_EXAMPLE),
                Some(constants_str::HTTP_LOCALHOST_ADMIN_SIGN_IN),
            ),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_DF43C793);
        assert_eq!(
            blocked_origin_response.status(),
            http::StatusCode::UNAUTHORIZED
        );
    }
}
mod test_schema {
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_generated_admin_descriptors_match_applied_migrations() {
        let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL)
            .expect(constants_str::DIAGNOSTIC_7E62AF41);
        let pool = super::SqlxAdminApiTestPool::from(
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(2)
                .connect(database_url.as_str())
                .await
                .expect(constants_str::DIAGNOSTIC_20250C41),
        );
        let mut admin_db_test_lock = pool
            .0
            .begin()
            .await
            .expect(constants_str::DIAGNOSTIC_50EB5D64);
        let _locked = sqlx::query(constants_str::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
            .execute(&mut *admin_db_test_lock)
            .await
            .expect(constants_str::DIAGNOSTIC_77883CF4);
        server_admin::prepare_postgresql::prepare_postgresql(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_9ECEDDF1);
        server_admin::validate_catalog_schema::validate_catalog_schema(
            pg_crud_common::sqlx_pg_catalog_pool_ref::SqlxPgCatalogPoolRef::from(&pool.0),
            pg_crud_common::db_schema_name_ref::DbSchemaNameRef::from(constants_str::PUBLIC),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_7A31CF02);
    }
    #[tokio::test]
    #[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
    async fn test_admin_string_policies_match_postgresql_constraints() {
        let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL)
            .expect(constants_str::DIAGNOSTIC_93FCB3DE);
        let pool = super::SqlxAdminApiTestPool::from(
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(2)
                .connect(database_url.as_str())
                .await
                .expect(constants_str::DIAGNOSTIC_D48C868D),
        );
        let mut admin_db_test_lock = pool
            .0
            .begin()
            .await
            .expect(constants_str::DIAGNOSTIC_99CED936);
        let _locked = sqlx::query(constants_str::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
            .execute(&mut *admin_db_test_lock)
            .await
            .expect(constants_str::DIAGNOSTIC_168B689C);
        server_admin::prepare_postgresql::prepare_postgresql(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_A453B862);
        let valid_login = server_admin_contract::admin_login::AdminLogin::try_from(
            constants_str::SSOT_LOGIN_VALID.to_owned(),
        )
        .is_ok();
        assert_eq!(
            server_admin_contract::admin_bool::AdminBool::from(valid_login),
            crate::postgres_accepts_admin_user_policy_values(
                &pool,
                super::StdAdminApiTestStrRef(constants_str::SSOT_DISPLAY_NAME_VALID),
                super::StdAdminApiTestStrRef(constants_str::SSOT_LOGIN_VALID),
            )
            .await
        );
        let invalid_login = server_admin_contract::admin_login::AdminLogin::try_from(
            constants_str::SSOT_LOGIN_INVALID_CASE.to_owned(),
        )
        .is_ok();
        assert_eq!(
            server_admin_contract::admin_bool::AdminBool::from(invalid_login),
            crate::postgres_accepts_admin_user_policy_values(
                &pool,
                super::StdAdminApiTestStrRef(constants_str::SSOT_DISPLAY_NAME_VALID),
                super::StdAdminApiTestStrRef(constants_str::SSOT_LOGIN_INVALID_CASE),
            )
            .await
        );
        let invalid_display =
            server_admin_contract::admin_display_name::AdminDisplayName::try_from(
                constants_str::SSOT_DISPLAY_NAME_PADDED.to_owned(),
            )
            .is_ok();
        assert_eq!(
            server_admin_contract::admin_bool::AdminBool::from(invalid_display),
            crate::postgres_accepts_admin_user_policy_values(
                &pool,
                super::StdAdminApiTestStrRef(constants_str::SSOT_DISPLAY_NAME_PADDED),
                super::StdAdminApiTestStrRef(constants_str::SSOT_LOGIN_VALID),
            )
            .await
        );
        let valid_role = server_admin_contract::admin_role_name::AdminRoleName::try_from(
            constants_str::SSOT_ROLE_VALID.to_owned(),
        )
        .is_ok();
        assert_eq!(
            server_admin_contract::admin_bool::AdminBool::from(valid_role),
            crate::postgres_accepts_admin_role_policy_value(
                &pool,
                super::StdAdminApiTestStrRef(constants_str::SSOT_ROLE_VALID),
            )
            .await
        );
        let invalid_role = server_admin_contract::admin_role_name::AdminRoleName::try_from(
            constants_str::SSOT_ROLE_INVALID_CASE.to_owned(),
        )
        .is_ok();
        assert_eq!(
            server_admin_contract::admin_bool::AdminBool::from(invalid_role),
            crate::postgres_accepts_admin_role_policy_value(
                &pool,
                super::StdAdminApiTestStrRef(constants_str::SSOT_ROLE_INVALID_CASE),
            )
            .await
        );
    }
}

#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
)]
struct StdAdminApiTestStrRef<'value_lt>(&'value_lt str);
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::FromInner)]
struct AxumAdminApiTestRouter(axum::Router);
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::FromInner)]
struct SqlxAdminApiTestPool(sqlx::PgPool);
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::FromInner)]
struct SqlxAdminHtmlTestTransaction(sqlx::Transaction<'static, sqlx::Postgres>);
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::FromInner)]
struct HttpAdminApiTestMethod(http::Method);
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::FromInner)]
struct HttpAdminApiTestRequest(http::Request<axum::body::Body>);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
struct HttpAdminHtmlTestResponse(http::Response<axum::body::Body>);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype::FromInner,
)]
struct HttpAdminApiTestResponseRef<'value_lt>(&'value_lt http::Response<axum::body::Body>);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::BoundedStringWrapper,
)]
#[bounded_string(max = 16384)]
#[derive(proc_macro_newtype::Display)]
struct StdAdminApiTestCookie(bounded_types::bounded_string::BoundedString<0usize, 16384, false>);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::BoundedStringWrapper,
)]
#[bounded_string(max = 1_048_576)]
struct AdminHtmlTestBody(bounded_types::bounded_string::BoundedString<0usize, 1_048_576, false>);
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::BoundedStringWrapper,
)]
#[bounded_string(max = 65_536)]
struct AdminHtmlTestFormBody(bounded_types::bounded_string::BoundedString<0usize, 65_536, false>);
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
struct AdminHtmlTestFixture {
    cookie: StdAdminApiTestCookie,
    csrf: StdAdminApiTestCookie,
    lock: SqlxAdminHtmlTestTransaction,
    pool: SqlxAdminApiTestPool,
    router: AxumAdminApiTestRouter,
}
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
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
        .expect(constants_str::DIAGNOSTIC_C2AF6158)
    }
}

fn one_admin_role_id(
    value: server_admin_contract::admin_role_id::AdminRoleId,
) -> server_admin_contract::admin_role_ids::AdminRoleIds {
    server_admin_contract::admin_role_ids::AdminRoleIds::try_from(vec![value])
        .expect(constants_str::DIAGNOSTIC_69BC51BC)
}
fn empty_admin_role_ids() -> server_admin_contract::admin_role_ids::AdminRoleIds {
    server_admin_contract::admin_role_ids::AdminRoleIds::try_from(Vec::new())
        .expect(constants_str::DIAGNOSTIC_D5CCD621)
}
fn env<T>(value: StdAdminApiTestStrRef<'_>) -> T
where
    T: config_lib::try_from_std_env_var_ok::TryFromStdEnvVarOk,
    T::Error: std::fmt::Debug,
{
    T::try_from_std_env_var_ok(
        config_lib::std_env_var_ok::StdEnvVarOk::try_from(value.0.to_owned())
            .expect(constants_str::DIAGNOSTIC_92B71C4E),
    )
    .expect(constants_str::DIAGNOSTIC_AFE20C19)
}
fn admin_api_test_router() -> AxumAdminApiTestRouter {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy(constants_str::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION)
        .expect(constants_str::DIAGNOSTIC_27DB915C);
    let state = server_admin::admin_auth_svc_state::AdminAuthSvcState::try_new(
        app_state::sqlx_pg_pool::SqlxPgPool::from(pool),
        &env::<config_lib::admin_jwt_secret::AdminJwtSecret>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES,
        )),
        &env::<config_lib::admin_access_token_ttl_seconds::AdminAccessTokenTtlSeconds>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_900),
        ),
        &env::<config_lib::admin_refresh_token_ttl_seconds::AdminRefreshTokenTtlSeconds>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_3600),
        ),
        &env::<config_lib::admin_session_limit::AdminSessionLimit>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_20,
        )),
        &env::<config_lib::admin_sign_in_rate_limit::AdminSignInRateLimit>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_2),
        ),
        &env::<config_lib::admin_login_failure_limit::AdminLoginFailureLimit>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_10),
        ),
        &env::<config_lib::admin_password_hash_concurrency::AdminPasswordHashConcurrency>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_1),
        ),
        &env::<config_lib::admin_cookie_secure::AdminCookieSecure>(StdAdminApiTestStrRef::from(
            constants_str::FALSE,
        )),
        &env::<config_lib::admin_token_issuer::AdminTokenIssuer>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST,
        )),
        &env::<config_lib::admin_token_audience::AdminTokenAudience>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST_ADMIN,
        )),
        &config_lib::domain_types::CorsAllowOrigin::try_from(
            constants_str::HTTP_LOCALHOST.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_396509B1),
    )
    .expect(constants_str::DIAGNOSTIC_F7D8C961);
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
            constants_str::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES,
        )),
        &env::<config_lib::admin_access_token_ttl_seconds::AdminAccessTokenTtlSeconds>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_900),
        ),
        &env::<config_lib::admin_refresh_token_ttl_seconds::AdminRefreshTokenTtlSeconds>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_3600),
        ),
        &env::<config_lib::admin_session_limit::AdminSessionLimit>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_20,
        )),
        &env::<config_lib::admin_sign_in_rate_limit::AdminSignInRateLimit>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_2),
        ),
        &env::<config_lib::admin_login_failure_limit::AdminLoginFailureLimit>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_10),
        ),
        &env::<config_lib::admin_password_hash_concurrency::AdminPasswordHashConcurrency>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_1),
        ),
        &env::<config_lib::admin_cookie_secure::AdminCookieSecure>(StdAdminApiTestStrRef::from(
            constants_str::FALSE,
        )),
        &env::<config_lib::admin_token_issuer::AdminTokenIssuer>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST,
        )),
        &env::<config_lib::admin_token_audience::AdminTokenAudience>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST_ADMIN,
        )),
        &config_lib::domain_types::CorsAllowOrigin::try_from(
            constants_str::HTTP_LOCALHOST.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_192C6B7A),
    )
    .expect(constants_str::DIAGNOSTIC_A59D73C1);
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
        .expect(constants_str::DIAGNOSTIC_7D924F8A);
    let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
        peer.0
            .parse::<std::net::SocketAddr>()
            .expect(constants_str::DIAGNOSTIC_D80FC31B),
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
        .expect(constants_str::DIAGNOSTIC_9F211B84);
    let _previous_peer = request.extensions_mut().insert(axum::extract::ConnectInfo(
        constants_str::VALUE_127_0_0_1_43210
            .parse::<std::net::SocketAddr>()
            .expect(constants_str::DIAGNOSTIC_BCD41A67),
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
            StdAdminApiTestCookie::try_from(value).expect(constants_str::DIAGNOSTIC_B9A203E6)
        })
        .expect(constants_str::DIAGNOSTIC_360DE719)
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
    .expect(constants_str::DIAGNOSTIC_3CB98672)
}
async fn admin_html_body(response: HttpAdminHtmlTestResponse) -> AdminHtmlTestBody {
    axum::body::to_bytes(response.0.into_body(), constants_usize::VALUE_1_048_576)
        .await
        .map(|bytes| String::from_utf8(bytes.to_vec()).expect(constants_str::DIAGNOSTIC_86547438))
        .map(|body| AdminHtmlTestBody::try_from(body).expect(constants_str::DIAGNOSTIC_EC7261CD))
        .expect(constants_str::DIAGNOSTIC_8B54DE37)
}
fn assert_admin_csr_shell(body: &AdminHtmlTestBody) {
    assert!(
        body.0.contains(constants_str::VALUE_03DEA637),
        "CSR root is missing"
    );
    assert!(
        body.0.contains(constants_str::VALUE_C84BBF51),
        "CSR application script is missing"
    );
    assert!(
        !body.0.contains(constants_str::VALUE_EA8C92A5),
        "server rendered a data table"
    );
    assert!(
        !body.0.contains(constants_str::VALUE_C23058ED),
        "server rendered a data form"
    );
}
#[expect(
    clippy::missing_assert_message,
    reason = "the asserted status identifies the failed fixture stage"
)]
async fn admin_html_test_fixture_with_password_change(
    password_change_required: server_admin_contract::admin_bool::AdminBool,
) -> AdminHtmlTestFixture {
    let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL)
        .expect(constants_str::DIAGNOSTIC_FBE54D19);
    let pool = SqlxAdminApiTestPool::from(
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5u32)
            .connect(database_url.as_str())
            .await
            .expect(constants_str::DIAGNOSTIC_AC089D31),
    );
    let mut lock = pool
        .0
        .begin()
        .await
        .expect(constants_str::DIAGNOSTIC_37480E56);
    let _locked = sqlx::query(constants_str::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
        .execute(&mut *lock)
        .await
        .expect(constants_str::DIAGNOSTIC_A6B7C8D9);
    server_admin::prepare_postgresql::prepare_postgresql(
        app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
    )
    .await
    .expect(constants_str::DIAGNOSTIC_45DE3A61);
    let _truncated = sqlx::query(
        constants_str::TRUNCATE_ADMIN_RATE_LIMITS_ADMIN_AUDIT_LOG_ADMIN_LOGIN_ATTEMPTS_ADMIN_ACCESS,
    )
    .execute(&pool.0)
    .await
    .expect(constants_str::DIAGNOSTIC_CF37A9E2);
    let _deleted_non_system_roles = sqlx::query(constants_str::VALUE_4BCE193A)
        .execute(&pool.0)
        .await
        .expect(constants_str::DIAGNOSTIC_B267A647);
    let password = serde_json::from_str::<
        server_admin_contract::admin_new_password::AdminNewPassword,
    >(constants_str::CORRECT_PASSWORD)
    .expect(constants_str::DIAGNOSTIC_D20A35E4);
    let hasher = server_admin::admin_password_hasher::AdminPasswordHasher::new(
        server_admin::runtime_admin_password_hash_concurrency::RuntimeAdminPasswordHashConcurrency::from(
            std::num::NonZeroUsize::new(constants_usize::ONE).expect(constants_str::DIAGNOSTIC_560498AB),
        ),
    );
    let _created_admin_id =
        server_admin::create_initial_administrator::create_initial_administrator(
            app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(&pool.0),
            server_admin_contract::admin_login::AdminLogin::try_from(
                constants_str::ADMIN_ALT.to_owned(),
            )
            .expect(constants_str::DIAGNOSTIC_6A417BDE),
            server_admin_contract::admin_display_name::AdminDisplayName::try_from(
                constants_str::ADMIN.to_owned(),
            )
            .expect(constants_str::DIAGNOSTIC_703FC568),
            password,
            &hasher,
        )
        .await
        .expect(constants_str::DIAGNOSTIC_1E29C87F);
    if !bool::from(password_change_required) {
        let _updated =
            sqlx::query(constants_str::UPDATE_ADMIN_USERS_SET_MUST_CHANGE_PASSWORD_FALSE)
                .execute(&pool.0)
                .await
                .expect(constants_str::DIAGNOSTIC_A37042F1);
    }
    let state = server_admin::admin_auth_svc_state::AdminAuthSvcState::try_new(
        app_state::sqlx_pg_pool::SqlxPgPool::from(pool.0.clone()),
        &env::<config_lib::admin_jwt_secret::AdminJwtSecret>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST_JWT_SECRET_AT_LEAST_32_BYTES,
        )),
        &env::<config_lib::admin_access_token_ttl_seconds::AdminAccessTokenTtlSeconds>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_900),
        ),
        &env::<config_lib::admin_refresh_token_ttl_seconds::AdminRefreshTokenTtlSeconds>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_3600),
        ),
        &env::<config_lib::admin_session_limit::AdminSessionLimit>(StdAdminApiTestStrRef::from(
            constants_str::VALUE_20,
        )),
        &env::<config_lib::admin_sign_in_rate_limit::AdminSignInRateLimit>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_20),
        ),
        &env::<config_lib::admin_login_failure_limit::AdminLoginFailureLimit>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_10),
        ),
        &env::<config_lib::admin_password_hash_concurrency::AdminPasswordHashConcurrency>(
            StdAdminApiTestStrRef::from(constants_str::VALUE_1),
        ),
        &env::<config_lib::admin_cookie_secure::AdminCookieSecure>(StdAdminApiTestStrRef::from(
            constants_str::FALSE,
        )),
        &env::<config_lib::admin_token_issuer::AdminTokenIssuer>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST,
        )),
        &env::<config_lib::admin_token_audience::AdminTokenAudience>(StdAdminApiTestStrRef::from(
            constants_str::INTEGRATION_TEST_ADMIN,
        )),
        &config_lib::domain_types::CorsAllowOrigin::try_from(
            constants_str::HTTP_LOCALHOST.to_owned(),
        )
        .expect(constants_str::DIAGNOSTIC_4BFC42C7),
    )
    .expect(constants_str::DIAGNOSTIC_EC39B61D);
    let router = AxumAdminApiTestRouter::from(axum::Router::from(
        server_admin::html_routes_with_swagger::html_routes_with_swagger(
            server_admin::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc::from(
                std::sync::Arc::new(state),
            ),
            server_admin::admin_html_swagger_enabled::AdminHtmlSwaggerEnabled::from(true),
        ),
    ));
    let correct_password = serde_json::from_str::<String>(constants_str::CORRECT_PASSWORD)
        .expect(constants_str::DIAGNOSTIC_825E50C7);
    let sign_in_body = AdminHtmlTestFormBody::try_from(format!(
        "login={}&password={correct_password}",
        constants_str::ADMIN_ALT,
    ))
    .expect(constants_str::DIAGNOSTIC_9DF2164C);
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
    .expect(constants_str::DIAGNOSTIC_68A2CB40);
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
        .expect(constants_str::DIAGNOSTIC_A4DF94D1),
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
        .expect(constants_str::DIAGNOSTIC_E6F2CDF7);
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
        .expect(constants_str::DIAGNOSTIC_FC4EEC8F);
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
        .expect(constants_str::DIAGNOSTIC_77C2DB82);
    let accepted = sqlx::query(constants_str::INSERT_ADMIN_ROLE_POLICY_PROBE)
        .bind(name.0)
        .execute(&mut *transaction)
        .await
        .is_ok();
    transaction
        .rollback()
        .await
        .expect(constants_str::DIAGNOSTIC_AA9B0106);
    server_admin_contract::admin_bool::AdminBool::from(accepted)
}
