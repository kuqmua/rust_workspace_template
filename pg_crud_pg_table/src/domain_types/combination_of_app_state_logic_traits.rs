pub trait CombinationOfAppStateLogicTraits:
    config_lib::domain_types::EnableApiGitCommitCheckProvider
    + config_lib::domain_types::MaximumSizeOfHttpBodyInBytesProvider
    + config_lib::domain_types::SrcPlaceTypeProvider
    + config_lib::domain_types::ChronoTimezoneProvider
    + app_state::domain_types::SqlxPgPoolProvider
    + server_runtime_http::domain_types::BulkItemResourceBudgetProvider
    + server_runtime_http::domain_types::IdempotencyResponseResourceBudgetProvider
    + Send
    + Sync
{
}
