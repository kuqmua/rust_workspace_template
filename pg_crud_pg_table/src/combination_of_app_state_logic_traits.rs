pub trait CombinationOfAppStateLogicTraits:
    config_lib::domain_types::EnableApiGitCommitCheckProvider
    + config_lib::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytesProvider
    + config_lib::domain_types::SrcPlaceTypeProvider
    + config_lib::chrono_timezone::ChronoTimezoneProvider
    + app_state::sqlx_pg_pool_provider::SqlxPgPoolProvider
    + server_runtime_core::bulk_item_resource_budget_provider::BulkItemResourceBudgetProvider
    + server_runtime_core::idempotency_response_resource_budget_provider::IdempotencyResponseResourceBudgetProvider
    + Send
    + Sync
{
}
