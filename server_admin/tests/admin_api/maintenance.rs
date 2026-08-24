#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_optimistic_revision_allows_one_concurrent_writer() {
    let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL).expect(
        "63a09eec postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold",
    );
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(4u32)
        .connect(database_url.as_str())
        .await
        .expect("2480f8c4 postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold");
    let _drop_before = sqlx::query(
        constants_str::DROP_TABLE_IF_EXISTS_PG_TABLE_OPTIMISTIC_REVISION_TEST,
    )
    .execute(&pool)
    .await
    .expect(
        "e5e1f7cb postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold",
    );
    let _create = sqlx::query(constants_str::CREATE_TABLE_PG_TABLE_OPTIMISTIC_REVISION_TEST_ID_BIGINT_PRIMARY_KEY_REVISION)
        .execute(&pool)
        .await
        .expect("a75bc224 postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold");
    let _insert = sqlx::query(
        constants_str::INSERT_INTO_PG_TABLE_OPTIMISTIC_REVISION_TEST_ID_REVISION_VALUE_VALUES_1,
    )
    .execute(&pool)
    .await
    .expect(
        "da271038 postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold",
    );
    let update = constants_str::UPDATE_PG_TABLE_OPTIMISTIC_REVISION_TEST_SET_VALUE_DOLLAR_1_REVISION_REVISION;
    let (left, right) = tokio::join!(
        sqlx::query_scalar::<_, i64>(update)
            .bind(constants_i64::ONE)
            .bind(
                pg_table::PgTableRevision::try_from(constants_str::VALUE_0.to_owned())
                    .expect("979fa4b2 postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold")
            )
            .fetch_optional(&pool),
        sqlx::query_scalar::<_, i64>(update)
            .bind(2i64)
            .bind(
                pg_table::PgTableRevision::try_from(constants_str::VALUE_0.to_owned())
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
            pg_table::PgTableRevision::try_from(constants_str::VALUE_0.to_owned())
                .expect("a3a08aeb postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold"),
        )
        .fetch_optional(&pool)
        .await
        .expect("964e3ef4 postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold");
    assert_eq!(stale, None);
    let _drop_after = sqlx::query(constants_str::DROP_TABLE_PG_TABLE_OPTIMISTIC_REVISION_TEST)
        .execute(&pool)
        .await
        .expect("a4d77f54 postgresql_optimistic_revision_allows_one_concurrent_writer invariant must hold");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn postgresql_cleanup_is_batched_and_preserves_append_only_policy() {
    let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL).expect("7316cf4d postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(3u32)
        .connect(database_url.as_str())
        .await
        .expect("f6a51733 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
    let mut admin_db_test_lock = pool.begin().await.expect("847caf57 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
    let _locked = sqlx::query(constants_str::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
        .execute(&mut *admin_db_test_lock)
        .await
        .expect("8c298fef postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
    let mut idempotency_test_isolation = pool.begin().await.expect("f56c4c85 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
    pg_crud_common::lock_pg_relation_resources(
        pg_crud_common::SqlxPgRelationLockConnectionRef::from(&mut *idempotency_test_isolation),
        &pg_crud_common::PgRelationLockNamespace::try_from(constants_str::ACTOR_ATOMIC.to_owned())
            .expect("861fe23d postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold"),
        &pg_crud_common::PgRelationResourceIds::try_from(vec![
            pg_crud_common::PgRelationResourceId::from(constants_i64::ONE),
        ])
        .expect("a18f804c postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold"),
    )
    .await
    .expect("fab61374 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
    server_admin::prep_pg(app_state::SqlxPgPoolRef::from(&pool))
        .await
        .expect("029cb682 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
    pg_table::ensure_pg_table_idempotency_schema(app_state::SqlxPgPoolRef::from(&pool))
        .await
        .expect("eb08dffc postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
    let _clear = sqlx::query(constants_str::TRUNCATE_ADMIN_ACCESS_SESSIONS_ADMIN_REFRESH_TOKENS_ADMIN_LOGIN_ATTEMPTS_ADMIN_RATE)
        .execute(&pool)
        .await
        .expect("e1b22572 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
    let _attempts = sqlx::query(constants_str::INSERT_INTO_ADMIN_LOGIN_ATTEMPTS_LOGIN_SUCCEEDED_ATTEMPTED_AT_SELECT_OLD_VALUE)
        .execute(&pool)
        .await
        .expect("480b06eb postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
    let _limits = sqlx::query(constants_str::INSERT_INTO_ADMIN_RATE_LIMITS_SCOPE_SUBJECT_WINDOW_STARTED_AT_REQUEST_COUNT_ALT)
        .execute(&pool)
        .await
        .expect("0375574d postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
    let _audit = sqlx::query(
        constants_str::INSERT_INTO_ADMIN_AUDIT_LOG_ACTION_RESOURCE_SUCCEEDED_CREATED_AT_SELECT_TEST,
    )
    .execute(&pool)
    .await
    .expect("f50ef817 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
    let retention =
        server_admin::AdminCleanupRetentionSeconds::try_from(3_600i64).expect("ab892fc5 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
    let config = server_admin::AdminCleanupCfg::new(
        server_admin::AdminCleanupBatchSize::try_from(2i64).expect("1d97b31c postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold"),
        retention,
        retention,
        retention,
        retention,
        retention,
    );
    let report = server_admin::cleanup_admin_tables(app_state::SqlxPgPoolRef::from(&pool), config)
        .await
        .expect("a422e8d4 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
    assert_eq!(report.total_rows().to_string(), "6");
    let remaining = sqlx::query_as::<_, (i64, i64, i64)>(constants_str::SELECT_SELECT_COUNT_ASTERISK_FROM_ADMIN_LOGIN_ATTEMPTS_SELECT_COUNT_ASTERISK_FROM)
        .fetch_one(&pool)
        .await
        .expect("f37a3ab4 postgresql_cleanup_is_batched_and_preserves_append_only_policy invariant must hold");
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
async fn postgresql_migration_creates_complete_schema() {
    let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL)
        .expect("b65d1786 postgresql_migration_creates_complete_schema invariant must hold");
    let base_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1u32)
        .connect(database_url.as_str())
        .await
        .expect("0047f74e postgresql_migration_creates_complete_schema invariant must hold");
    let _drop_schema =
        sqlx::raw_sql(constants_str::DROP_SCHEMA_IF_EXISTS_ADMIN_MIGRATION_FRESH_TEST_CASCADE)
            .execute(&base_pool)
            .await
            .expect("df91b04d postgresql_migration_creates_complete_schema invariant must hold");
    let _create_schema = sqlx::raw_sql(constants_str::CREATE_SCHEMA_ADMIN_MIGRATION_FRESH_TEST)
        .execute(&base_pool)
        .await
        .expect("02bcd1c2 postgresql_migration_creates_complete_schema invariant must hold");
    let connect = |schema: StdAdminApiTestStrRef<'static>| {
        let options = <sqlx::postgres::PgConnectOptions as std::str::FromStr>::from_str(
            database_url.as_str(),
        )
        .expect("aa7735db postgresql_migration_creates_complete_schema invariant must hold")
        .options([(constants_str::SEARCH_PATH, schema.0)]);
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(1u32)
            .connect_lazy_with(options)
    };
    let fresh_pool = connect(StdAdminApiTestStrRef::from(
        constants_str::ADMIN_MIGRATION_FRESH_TEST,
    ));
    let full = sqlx::migrate!("./migrations");
    full.run(&fresh_pool)
        .await
        .expect("4b6c3bd6 postgresql_migration_creates_complete_schema invariant must hold");
    server_admin::generated_tables::validate_catalog_schema(
        pg_crud_common::SqlxPgPoolRef::from(&fresh_pool),
        pg_crud_common::DbSchemaNameRef::from(constants_str::ADMIN_MIGRATION_FRESH_TEST),
    )
    .await
    .expect("fac299aa postgresql_migration_creates_complete_schema invariant must hold");
    let catalog_snapshot = pg_crud_common::inspect_postgres_catalog(
        pg_crud_common::SqlxPgPoolRef::from(&fresh_pool),
        pg_crud_common::DbSchemaNameRef::from(constants_str::ADMIN_MIGRATION_FRESH_TEST),
    )
    .await
    .expect("518b93e4 postgresql_migration_creates_complete_schema invariant must hold");
    let fresh_pool_ref = &fresh_pool;
    let table_snapshots = futures::future::try_join_all(
        server_admin_contract::AdminDataTable::PG_ORDER
            .into_iter()
            .map(async |table| {
                pg_crud_common::inspect_postgres_table(
                    pg_crud_common::SqlxPgPoolRef::from(fresh_pool_ref),
                    pg_crud_common::DbSchemaNameRef::from(
                        constants_str::ADMIN_MIGRATION_FRESH_TEST,
                    ),
                    pg_crud_common::DbTableNameRef::from(table.as_str().get()),
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
        .join(constants_str::ADMIN_CURRENT_SCHEMA_SNAPSHOT_PATH);
    if std::env::var_os(constants_str::UPDATE_ADMIN_CURRENT_SCHEMA_SNAPSHOT).is_some() {
        std::fs::write(
            current_schema_snapshot_path.as_path(),
            current_schema_snapshot.as_bytes(),
        )
        .expect("abe4d63f postgresql_migration_creates_complete_schema invariant must hold");
    }
    let expected_current_schema_snapshot = std::fs::read_to_string(current_schema_snapshot_path)
        .expect("3af279e1 postgresql_migration_creates_complete_schema invariant must hold");
    assert_eq!(
        current_schema_snapshot, expected_current_schema_snapshot,
        "cb6ce4a9 migration-derived PostgreSQL schema snapshot changed"
    );
    let version = sqlx::query_scalar::<_, i64>(
        constants_str::SELECT_MAX_VERSION_FROM_ADMIN_MIGRATION_FRESH_TEST_SQLX_MIGRATIONS_WHERE,
    )
    .fetch_one(&base_pool)
    .await
    .expect("5c10c931 postgresql_migration_creates_complete_schema invariant must hold");
    assert_eq!(version, 13i64);
    let expected_tables = server_admin_contract::AdminDataTable::PG_ORDER
        .map(|table| table.to_string())
        .into_iter()
        .collect::<std::collections::BTreeSet<String>>();
    let fresh_tables = sqlx::query_scalar::<_, String>(
        constants_str::SELECT_TABLE_NAME_FROM_INFORMATION_SCHEMA_TABLES_WHERE_TABLE_SCHEMA,
    )
    .bind(constants_str::ADMIN_MIGRATION_FRESH_TEST)
    .fetch_all(&base_pool)
    .await
    .expect("ab254ff4 postgresql_migration_creates_complete_schema invariant must hold")
    .into_iter()
    .collect::<std::collections::BTreeSet<String>>();
    assert_eq!(fresh_tables, expected_tables);
    fresh_pool.close().await;
    let _drop_after = sqlx::raw_sql(constants_str::DROP_SCHEMA_ADMIN_MIGRATION_FRESH_TEST_CASCADE)
        .execute(&base_pool)
        .await
        .expect("88dd90b8 postgresql_migration_creates_complete_schema invariant must hold");
}
#[cfg(test)]
use super::StdAdminApiTestStrRef;
