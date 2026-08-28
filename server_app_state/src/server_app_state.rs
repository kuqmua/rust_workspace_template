#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub struct ServerAppState<'lt> {
    pub bulk_item_budget: server_runtime_core::ResourceBudget,
    pub config: server_config::config::Config,
    pub idempotency_response_budget: server_runtime_core::ResourceBudget,
    pub pg_pool: app_state::SqlxPgPool,
    pub project_git_info: git_info::ProjectGitInfo<'lt>,
}
impl ServerAppState<'_> {
    const fn cfg_ref(&self) -> &server_config::config::Config {
        &self.config
    }
}
impl common_routes::CommonRoutesParameters for ServerAppState<'_> {}
impl pg_table::CombinationOfAppStateLogicTraits for ServerAppState<'_> {}
impl server_runtime_core::BulkItemResourceBudgetProvider for ServerAppState<'_> {
    fn bulk_item_resource_budget(&self) -> &server_runtime_core::ResourceBudget {
        &self.bulk_item_budget
    }
}
impl server_runtime_core::IdempotencyResponseResourceBudgetProvider for ServerAppState<'_> {
    fn idempotency_response_resource_budget(&self) -> &server_runtime_core::ResourceBudget {
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
    config_lib::domain_types::types::SrcPlaceType
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::ChronoTimezoneProvider,
    chrono_timezone,
    chrono::FixedOffset
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::MaximumSizeOfHttpBodyInBytesProvider,
    maximum_size_of_http_body_in_bytes,
    usize
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminCookieSecureProvider,
    admin_cookie_secure,
    bool
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminJwtSecretProvider,
    admin_jwt_secret,
    bounded_types::BoundedVec<config_lib::domain_types::SecrecySecretBoxString, 1, 8>
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminTokenAudienceProvider,
    admin_token_audience,
    String
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminTokenIssuerProvider,
    admin_token_issuer,
    String
);
impl app_state::SqlxPgPoolProvider for ServerAppState<'_> {
    fn sqlx_pg_pool(&self) -> app_state::SqlxPgPoolRef<'_> {
        app_state::SqlxPgPoolRef::from(self.pg_pool.as_ref())
    }
}
impl AsRef<str> for ServerAppState<'_> {
    fn as_ref(&self) -> &str {
        self.project_git_info.as_ref()
    }
}
