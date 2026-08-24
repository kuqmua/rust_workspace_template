#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn generated_admin_descriptors_match_applied_migrations() {
    let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL).expect(
        "7e62af41 generated_admin_descriptors_match_applied_migrations invariant must hold",
    );
    let pool = SqlxAdminApiTestPool::from(
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
    let _locked = sqlx::query(constants_str::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
        .execute(&mut *admin_db_test_lock)
        .await
        .expect(
            "77883cf4 generated_admin_descriptors_match_applied_migrations invariant must hold",
        );
    server_admin::prep_pg(app_state::domain_types::SqlxPgPoolRef::from(&pool.0))
        .await
        .expect(
            "9eceddf1 generated_admin_descriptors_match_applied_migrations invariant must hold",
        );
    server_admin::generated_tables::validate_catalog_schema(
        pg_crud_common::SqlxPgPoolRef::from(&pool.0),
        pg_crud_common::DbSchemaNameRef::from(constants_str::PUBLIC),
    )
    .await
    .expect("7a31cf02 generated_admin_descriptors_match_applied_migrations invariant must hold");
}
#[tokio::test]
#[ignore = "requires PostgreSQL; run through workspace_test_runner database"]
async fn admin_string_policies_match_postgresql_constraints() {
    let database_url = std::env::var(constants_str::ENV_NAMES_DATABASE_URL)
        .expect("93fcb3de admin_string_policies_match_postgresql_constraints invariant must hold");
    let pool = SqlxAdminApiTestPool::from(
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect(database_url.as_str())
            .await
            .expect(
                "d48c868d admin_string_policies_match_postgresql_constraints invariant must hold",
            ),
    );
    let mut admin_db_test_lock =
        pool.0.begin().await.expect(
            "99ced936 admin_string_policies_match_postgresql_constraints invariant must hold",
        );
    let _locked = sqlx::query(constants_str::SELECT_PG_ADVISORY_XACT_LOCK_ADMIN_TESTS)
        .execute(&mut *admin_db_test_lock)
        .await
        .expect("168b689c admin_string_policies_match_postgresql_constraints invariant must hold");
    server_admin::prep_pg(app_state::domain_types::SqlxPgPoolRef::from(&pool.0))
        .await
        .expect("a453b862 admin_string_policies_match_postgresql_constraints invariant must hold");
    let valid_login =
        server_admin_contract::AdminLogin::try_from(constants_str::SSOT_LOGIN_VALID.to_owned())
            .is_ok();
    assert_eq!(
        server_admin_contract::AdminBool::from(valid_login),
        postgres_accepts_admin_user_policy_values(
            &pool,
            StdAdminApiTestStrRef(constants_str::SSOT_DISPLAY_NAME_VALID),
            StdAdminApiTestStrRef(constants_str::SSOT_LOGIN_VALID),
        )
        .await
    );
    let invalid_login = server_admin_contract::AdminLogin::try_from(
        constants_str::SSOT_LOGIN_INVALID_CASE.to_owned(),
    )
    .is_ok();
    assert_eq!(
        server_admin_contract::AdminBool::from(invalid_login),
        postgres_accepts_admin_user_policy_values(
            &pool,
            StdAdminApiTestStrRef(constants_str::SSOT_DISPLAY_NAME_VALID),
            StdAdminApiTestStrRef(constants_str::SSOT_LOGIN_INVALID_CASE),
        )
        .await
    );
    let invalid_display = server_admin_contract::AdminDisplayName::try_from(
        constants_str::SSOT_DISPLAY_NAME_PADDED.to_owned(),
    )
    .is_ok();
    assert_eq!(
        server_admin_contract::AdminBool::from(invalid_display),
        postgres_accepts_admin_user_policy_values(
            &pool,
            StdAdminApiTestStrRef(constants_str::SSOT_DISPLAY_NAME_PADDED),
            StdAdminApiTestStrRef(constants_str::SSOT_LOGIN_VALID),
        )
        .await
    );
    let valid_role =
        server_admin_contract::AdminRoleName::try_from(constants_str::SSOT_ROLE_VALID.to_owned())
            .is_ok();
    assert_eq!(
        server_admin_contract::AdminBool::from(valid_role),
        postgres_accepts_admin_role_policy_value(
            &pool,
            StdAdminApiTestStrRef(constants_str::SSOT_ROLE_VALID),
        )
        .await
    );
    let invalid_role = server_admin_contract::AdminRoleName::try_from(
        constants_str::SSOT_ROLE_INVALID_CASE.to_owned(),
    )
    .is_ok();
    assert_eq!(
        server_admin_contract::AdminBool::from(invalid_role),
        postgres_accepts_admin_role_policy_value(
            &pool,
            StdAdminApiTestStrRef(constants_str::SSOT_ROLE_INVALID_CASE),
        )
        .await
    );
}
#[cfg(test)]
use super::{
    SqlxAdminApiTestPool, StdAdminApiTestStrRef, postgres_accepts_admin_role_policy_value,
    postgres_accepts_admin_user_policy_values,
};
