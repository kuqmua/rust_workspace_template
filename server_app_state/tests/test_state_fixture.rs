#![allow(
    unused_crate_dependencies,
    reason = "this integration target verifies public fixture state and budgets while other package dependencies support configuration construction"
)]

#[cfg(test)]
#[cfg(feature = "test-utils")]
mod tests {
    #[tokio::test]
    #[ignore = "constructs a lazy SQLx pool; requires a provisioned native Tokio process and sends no database queries"]
    async fn test_server_state_fixture_preserves_configuration_and_exact_budget_capacities() {
        let server_app_state =
            server_app_state::make_test_server_app_state::make_test_server_app_state();
        let bulk_budget = server_runtime_core::bulk_item_resource_budget_provider::BulkItemResourceBudgetProvider::bulk_item_resource_budget(&server_app_state);
        let bulk_reservation = bulk_budget.reserve(
            server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(8usize),
        );
        let bulk_capacity_matches = bulk_reservation.is_ok()
            && bulk_budget
                .reserve(server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(1usize))
                .is_err_and(|error| error == server_runtime_core::resource_budget_reserve_error::ResourceBudgetReserveError::Exhausted);
        drop(bulk_reservation);
        let response_budget = server_runtime_core::idempotency_response_resource_budget_provider::IdempotencyResponseResourceBudgetProvider::idempotency_response_resource_budget(&server_app_state);
        let response_reservation = response_budget.reserve(
            server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(4_096usize),
        );
        let response_capacity_matches = response_reservation.is_ok()
            && response_budget
                .reserve(server_runtime_core::resource_budget_amount::ResourceBudgetAmount::from(1usize))
                .is_err_and(|error| error == server_runtime_core::resource_budget_reserve_error::ResourceBudgetReserveError::Exhausted);
        drop(response_reservation);
        let sqlx_pg_pool_ref =
            app_state::sqlx_pg_pool_provider::SqlxPgPoolProvider::sqlx_pg_pool(&server_app_state);
        let pool_initially_open = !sqlx_pg_pool_ref.as_ref().is_closed();
        sqlx_pg_pool_ref.as_ref().close().await;
        assert!(pool_initially_open);
        assert!(sqlx_pg_pool_ref.as_ref().is_closed());
        assert!(bulk_capacity_matches);
        assert!(response_capacity_matches);
        assert!(
            !config_lib::domain_types::EnableApiGitCommitCheckProvider::enable_api_git_commit_check(
                &server_app_state
            )
        );
        assert!(
            !config_lib::admin_cookie_secure::AdminCookieSecureProvider::admin_cookie_secure(
                &server_app_state
            )
        );
        assert_eq!(
            config_lib::domain_types::SourcePlaceTypeProvider::source_place_type(&server_app_state),
            &config_lib::source_place_type::SourcePlaceType::Github
        );
        assert_eq!(
            config_lib::chrono_timezone::ChronoTimezoneProvider::chrono_timezone(&server_app_state)
                .local_minus_utc(),
            10_800i32
        );
        assert_eq!(
            config_lib::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytesProvider::maximum_size_of_http_body_in_bytes(&server_app_state),
            &1_024usize
        );
        assert_eq!(
            config_lib::admin_token_audience::AdminTokenAudienceProvider::admin_token_audience(
                &server_app_state
            ),
            constants_str::TEST_AUDIENCE
        );
        assert_eq!(
            config_lib::admin_token_issuer::AdminTokenIssuerProvider::admin_token_issuer(
                &server_app_state
            ),
            constants_str::TEST_ISSUER
        );
        assert_eq!(
            config_lib::admin_jwt_secret::AdminJwtSecretProvider::admin_jwt_secret(
                &server_app_state
            )
            .len()
            .get(),
            1usize
        );
        assert_eq!(
            AsRef::<str>::as_ref(&server_app_state),
            AsRef::<str>::as_ref(&git_info::project_git_info_value::project_git_info_value())
        );
    }
}
