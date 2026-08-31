#[tokio::test]
async fn test_admin_service_tests() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy(constants_str::POSTGRES_ADMIN_INTEGRATION_ONLY_127_0_0_1_ADMIN_INTEGRATION)
        .expect("5bd94807 auth_state_rejects_empty_cors_origin_entries invariant must hold");
    assert!(matches!(
        crate::application_tests_helper::auth_state(
            pool,
            constants_str::TEST_CORS_ORIGINS_WITH_EMPTY_ENTRY,
        ),
        Err(crate::admin_auth_svc_state_build_error::AdminAuthSvcStateBuildError::AllowedOrigin)
    ));
}
