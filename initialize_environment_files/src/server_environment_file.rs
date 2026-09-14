#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub(crate) struct ServerEnvironmentFile {
    admin_access_token_ttl_seconds: crate::configuration_field::ConfigurationField,
    admin_cookie_secure: crate::configuration_field::ConfigurationField,
    admin_jwt_secret: crate::configuration_field::ConfigurationField,
    admin_login_failure_limit: crate::configuration_field::ConfigurationField,
    admin_password_hash_concurrency: crate::configuration_field::ConfigurationField,
    admin_refresh_token_ttl_seconds: crate::configuration_field::ConfigurationField,
    admin_session_limit: crate::configuration_field::ConfigurationField,
    admin_sign_in_rate_limit: crate::configuration_field::ConfigurationField,
    admin_swagger_enabled: crate::configuration_field::ConfigurationField,
    admin_token_audience: crate::configuration_field::ConfigurationField,
    admin_token_issuer: crate::configuration_field::ConfigurationField,
    content_security_policy: crate::configuration_field::ConfigurationField,
    cors_allow_origin: crate::configuration_field::ConfigurationField,
    database_url: crate::configuration_field::ConfigurationField,
    enable_api_git_commit_check: crate::configuration_field::ConfigurationField,
    http_gzip_enabled: crate::configuration_field::ConfigurationField,
    maximum_size_of_http_body_in_bytes: crate::configuration_field::ConfigurationField,
    pg_pool_acquire_timeout_seconds: crate::configuration_field::ConfigurationField,
    pg_pool_idle_timeout_seconds: crate::configuration_field::ConfigurationField,
    pg_pool_max_connections: crate::configuration_field::ConfigurationField,
    pg_pool_max_lifetime_seconds: crate::configuration_field::ConfigurationField,
    pg_pool_min_connections: crate::configuration_field::ConfigurationField,
    production_mode: crate::configuration_field::ConfigurationField,
    request_timeout_seconds: crate::configuration_field::ConfigurationField,
    service_socket_address: crate::configuration_field::ConfigurationField,
    source_place_type: crate::configuration_field::ConfigurationField,
    svc_mode: crate::configuration_field::ConfigurationField,
    timezone: crate::configuration_field::ConfigurationField,
    tracing_format: crate::configuration_field::ConfigurationField,
    tracing_level: crate::configuration_field::ConfigurationField,
    trusted_proxy_ranges_text: crate::configuration_field::ConfigurationField,
}

impl ServerEnvironmentFile {
    pub(crate) const fn content(&self) -> &'static [u8] {
        match self.get_admin_access_token_ttl_seconds() {
            crate::configuration_field::ConfigurationField::Declared => {
                crate::server_environment_content::SERVER_ENVIRONMENT_CONTENT
            }
        }
    }
}
