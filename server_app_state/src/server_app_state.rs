#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub struct ServerAppState<'lt> {
    bulk_item_budget: server_runtime_core::resource_budget::ResourceBudget,
    config: server_config::server_config::ServerConfig,
    idempotency_response_budget: server_runtime_core::resource_budget::ResourceBudget,
    pg_pool: app_state::sqlx_pg_pool::SqlxPgPool,
    project_git_info: git_info::project_git_info::ProjectGitInfo<'lt>,
}
impl<'lt> ServerAppState<'lt> {
    const fn cfg_ref(&self) -> &server_config::server_config::ServerConfig {
        &self.config
    }

    #[must_use]
    pub const fn new(
        bulk_item_budget: server_runtime_core::resource_budget::ResourceBudget,
        config: server_config::server_config::ServerConfig,
        idempotency_response_budget: server_runtime_core::resource_budget::ResourceBudget,
        pg_pool: app_state::sqlx_pg_pool::SqlxPgPool,
        project_git_info: git_info::project_git_info::ProjectGitInfo<'lt>,
    ) -> Self {
        Self {
            bulk_item_budget,
            config,
            idempotency_response_budget,
            pg_pool,
            project_git_info,
        }
    }
}
impl common_routes::common_routes_parameters::CommonRoutesParameters for ServerAppState<'_> {}
impl pg_table::combination_of_app_state_logic_traits::CombinationOfAppStateLogicTraits
    for ServerAppState<'_>
{
}
impl server_runtime_core::bulk_item_resource_budget_provider::BulkItemResourceBudgetProvider
    for ServerAppState<'_>
{
    fn bulk_item_resource_budget(&self) -> &server_runtime_core::resource_budget::ResourceBudget {
        &self.bulk_item_budget
    }
}
impl server_runtime_core::idempotency_response_resource_budget_provider::IdempotencyResponseResourceBudgetProvider
    for ServerAppState<'_>
{
    fn idempotency_response_resource_budget(
        &self,
    ) -> &server_runtime_core::resource_budget::ResourceBudget {
        &self.idempotency_response_budget
    }
}
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::EnableApiGitCommitCheckProvider,
    enable_api_git_commit_check,
    bool
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::SrcPlaceTypeProvider,
    src_place_type,
    config_lib::src_place_type::SrcPlaceType
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::chrono_timezone::ChronoTimezoneProvider,
    chrono_timezone,
    chrono::FixedOffset
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::maximum_size_of_http_body_in_bytes::MaximumSizeOfHttpBodyInBytesProvider,
    maximum_size_of_http_body_in_bytes,
    usize
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::admin_cookie_secure::AdminCookieSecureProvider,
    admin_cookie_secure,
    bool
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::admin_jwt_secret::AdminJwtSecretProvider,
    admin_jwt_secret,
    bounded_types::bounded_vec::BoundedVec<config_lib::secrecy_secret_box_string::SecrecySecretBoxString, 1, 8>
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::admin_token_audience::AdminTokenAudienceProvider,
    admin_token_audience,
    bounded_types::bounded_string::BoundedString<0usize, 256, false>
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::admin_token_issuer::AdminTokenIssuerProvider,
    admin_token_issuer,
    bounded_types::bounded_string::BoundedString<0usize, 256, false>
);
impl app_state::sqlx_pg_pool_provider::SqlxPgPoolProvider for ServerAppState<'_> {
    fn sqlx_pg_pool(&self) -> app_state::sqlx_pg_pool_ref::SqlxPgPoolRef<'_> {
        app_state::sqlx_pg_pool_ref::SqlxPgPoolRef::from(self.pg_pool.as_ref())
    }
}
impl AsRef<str> for ServerAppState<'_> {
    fn as_ref(&self) -> &str {
        self.project_git_info.as_ref()
    }
}
