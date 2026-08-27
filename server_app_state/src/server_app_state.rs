#[derive(Debug, optimal_memory_layout::OptimalMemoryLayout)]
pub struct ServerAppState<'lt> {
    pub bulk_item_budget: server_runtime_core::domain_types::ResourceBudget,
    pub config: server_config::domain_types::Config,
    pub idempotency_response_budget: server_runtime_core::domain_types::ResourceBudget,
    pub pg_pool: app_state::domain_types::SqlxPgPool,
    pub project_git_info: git_info::domain_types::ProjectGitInfo<'lt>,
}
impl ServerAppState<'_> {
    const fn cfg_ref(&self) -> &server_config::domain_types::Config {
        &self.config
    }
}
impl common_routes::domain_types::CommonRoutesParameters for ServerAppState<'_> {}
impl pg_table::domain_types::CombinationOfAppStateLogicTraits for ServerAppState<'_> {}
impl server_runtime_core::domain_types::BulkItemResourceBudgetProvider for ServerAppState<'_> {
    fn bulk_item_resource_budget(&self) -> &server_runtime_core::domain_types::ResourceBudget {
        &self.bulk_item_budget
    }
}
impl server_runtime_core::domain_types::IdempotencyResponseResourceBudgetProvider
    for ServerAppState<'_>
{
    fn idempotency_response_resource_budget(
        &self,
    ) -> &server_runtime_core::domain_types::ResourceBudget {
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
    config_lib::domain_types::AdminAccessTokenTtlSecondsProvider,
    admin_access_token_ttl_seconds,
    config_lib::domain_types::ConfigNonZeroU64
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminCookieSecureProvider,
    admin_cookie_secure,
    bool
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminJwtSecretProvider,
    admin_jwt_secret,
    bounded_types::domain_types::vector::BoundedVec<config_lib::domain_types::SecrecySecretBoxString, 1, 8>
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminPasswordHashConcurrencyProvider,
    admin_password_hash_concurrency,
    config_lib::domain_types::ConfigNonZeroUsize
);
server_app_state_macros::impl_cfg_accessor!(
    config_lib::domain_types::AdminRefreshTokenTtlSecondsProvider,
    admin_refresh_token_ttl_seconds,
    config_lib::domain_types::ConfigNonZeroU64
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
impl app_state::domain_types::SqlxPgPoolProvider for ServerAppState<'_> {
    fn sqlx_pg_pool(&self) -> app_state::domain_types::SqlxPgPoolRef<'_> {
        app_state::domain_types::SqlxPgPoolRef::from(self.pg_pool.as_ref())
    }
}
impl AsRef<str> for ServerAppState<'_> {
    fn as_ref(&self) -> &str {
        self.project_git_info.as_ref()
    }
}
