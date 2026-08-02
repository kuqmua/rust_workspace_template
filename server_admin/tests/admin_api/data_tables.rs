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
#[cfg(test)]
use super::{
    HttpAdminApiTestMethod, StdAdminApiTestStrRef, admin_html_test_fixture, request_with_peer,
    router_with_pool,
};
