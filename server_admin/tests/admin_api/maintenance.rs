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
    let catalog_snapshot = pg_crud_common::inspect_postgres_catalog(
        pg_crud_common::SqlxPgPoolRef::from(&fresh_pool),
        pg_crud_common::DbSchemaNameRef::from(str_constants::ADMIN_MIGRATION_FRESH_TEST),
    )
    .await
    .expect("518b93e4");
    let fresh_pool_ref = &fresh_pool;
    let table_snapshots = futures::future::try_join_all(
        server_admin_contract::AdminDataTable::PG_ORDER
            .into_iter()
            .map(async |table| {
                pg_crud_common::inspect_postgres_table(
                    pg_crud_common::SqlxPgPoolRef::from(fresh_pool_ref),
                    pg_crud_common::DbSchemaNameRef::from(
                        str_constants::ADMIN_MIGRATION_FRESH_TEST,
                    ),
                    pg_crud_common::DbTableNameRef::from(table.as_str().get()),
                )
                .await
                .map(|snapshot| (table, snapshot))
            }),
    )
    .await
    .expect("34d80f68");
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
        .join(str_constants::ADMIN_CURRENT_SCHEMA_SNAPSHOT_PATH);
    if std::env::var_os(str_constants::UPDATE_ADMIN_CURRENT_SCHEMA_SNAPSHOT).is_some() {
        std::fs::write(
            current_schema_snapshot_path.as_path(),
            current_schema_snapshot.as_bytes(),
        )
        .expect("abe4d63f");
    }
    let expected_current_schema_snapshot =
        std::fs::read_to_string(current_schema_snapshot_path).expect("3af279e1");
    assert_eq!(
        current_schema_snapshot, expected_current_schema_snapshot,
        "cb6ce4a9 migration-derived PostgreSQL schema snapshot changed"
    );
    let version = sqlx::query_scalar::<_, i64>(
        str_constants::SELECT_MAX_VERSION_FROM_ADMIN_MIGRATION_FRESH_TEST_SQLX_MIGRATIONS_WHERE,
    )
    .fetch_one(&base_pool)
    .await
    .expect("5c10c931");
    assert_eq!(version, 13i64);
    let expected_tables = server_admin_contract::AdminDataTable::PG_ORDER
        .map(|table| table.to_string())
        .into_iter()
        .collect::<std::collections::BTreeSet<String>>();
    let fresh_tables = sqlx::query_scalar::<_, String>(
        str_constants::SELECT_TABLE_NAME_FROM_INFORMATION_SCHEMA_TABLES_WHERE_TABLE_SCHEMA,
    )
    .bind(str_constants::ADMIN_MIGRATION_FRESH_TEST)
    .fetch_all(&base_pool)
    .await
    .expect("ab254ff4")
    .into_iter()
    .collect::<std::collections::BTreeSet<String>>();
    assert_eq!(fresh_tables, expected_tables);
    fresh_pool.close().await;
    let _drop_after = sqlx::raw_sql(str_constants::DROP_SCHEMA_ADMIN_MIGRATION_FRESH_TEST_CASCADE)
        .execute(&base_pool)
        .await
        .expect("88dd90b8");
}
#[cfg(test)]
use super::StdAdminApiTestStrRef;
