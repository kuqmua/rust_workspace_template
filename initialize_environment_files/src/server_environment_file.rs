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
    pub(crate) fn content(&self) -> crate::std_byte_vector::StdByteVector {
        let mut std_byte_vector = crate::std_byte_vector::StdByteVector::default();
        self.get_admin_access_token_ttl_seconds()
            .append_to(&mut std_byte_vector);
        self.get_admin_cookie_secure()
            .append_to(&mut std_byte_vector);
        self.get_admin_jwt_secret().append_to(&mut std_byte_vector);
        self.get_admin_login_failure_limit()
            .append_to(&mut std_byte_vector);
        self.get_admin_password_hash_concurrency()
            .append_to(&mut std_byte_vector);
        self.get_admin_refresh_token_ttl_seconds()
            .append_to(&mut std_byte_vector);
        self.get_admin_session_limit()
            .append_to(&mut std_byte_vector);
        self.get_admin_sign_in_rate_limit()
            .append_to(&mut std_byte_vector);
        self.get_admin_swagger_enabled()
            .append_to(&mut std_byte_vector);
        self.get_admin_token_audience()
            .append_to(&mut std_byte_vector);
        self.get_admin_token_issuer()
            .append_to(&mut std_byte_vector);
        self.get_content_security_policy()
            .append_to(&mut std_byte_vector);
        self.get_cors_allow_origin().append_to(&mut std_byte_vector);
        self.get_database_url().append_to(&mut std_byte_vector);
        self.get_enable_api_git_commit_check()
            .append_to(&mut std_byte_vector);
        self.get_http_gzip_enabled().append_to(&mut std_byte_vector);
        self.get_maximum_size_of_http_body_in_bytes()
            .append_to(&mut std_byte_vector);
        self.get_pg_pool_acquire_timeout_seconds()
            .append_to(&mut std_byte_vector);
        self.get_pg_pool_idle_timeout_seconds()
            .append_to(&mut std_byte_vector);
        self.get_pg_pool_max_connections()
            .append_to(&mut std_byte_vector);
        self.get_pg_pool_max_lifetime_seconds()
            .append_to(&mut std_byte_vector);
        self.get_pg_pool_min_connections()
            .append_to(&mut std_byte_vector);
        self.get_production_mode().append_to(&mut std_byte_vector);
        self.get_request_timeout_seconds()
            .append_to(&mut std_byte_vector);
        self.get_service_socket_address()
            .append_to(&mut std_byte_vector);
        self.get_source_place_type().append_to(&mut std_byte_vector);
        self.get_svc_mode().append_to(&mut std_byte_vector);
        self.get_timezone().append_to(&mut std_byte_vector);
        self.get_tracing_format().append_to(&mut std_byte_vector);
        self.get_tracing_level().append_to(&mut std_byte_vector);
        self.get_trusted_proxy_ranges_text()
            .append_to(&mut std_byte_vector);
        std_byte_vector
    }
}
