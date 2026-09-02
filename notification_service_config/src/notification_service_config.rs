#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "fields are ordered by decreasing alignment as enforced by optimal_memory_layout"
)]
#[derive(
    Debug,
    proc_macro_try_from_env::TryFromEnv,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
)]
#[config(env_example)]
pub struct NotificationServiceConfig {
    #[config(secret)]
    #[config(accessor)]
    #[config(
        example = "postgres://notification_service:change-me@127.0.0.1:5432/notification_service"
    )]
    notification_database_url: config_lib::domain_types::DatabaseUrl,
    #[config(accessor)]
    #[config(example = "30")]
    request_timeout_seconds: config_lib::request_timeout_seconds::RequestTimeoutSeconds,
    #[config(accessor)]
    #[config(example = "127.0.0.1:8081")]
    notification_service_socket_address: config_lib::domain_types::ServiceSocketAddress,
    #[config(accessor)]
    #[config(example = "10")]
    pg_pool_max_connections: config_lib::pg_pool_max_connections::PgPoolMaxConnections,
    #[config(accessor)]
    #[config(example = "text")]
    tracing_format: config_lib::tracing_format::TracingFormat,
    #[config(accessor)]
    #[config(example = "serve")]
    svc_mode: config_lib::svc_mode::SvcMode,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_derive_generates_typed_accessors() {
        fn is_typed_accessor<Value>(
            _accessor: fn(&super::NotificationServiceConfig) -> &Value,
        ) -> bool {
            true
        }

        assert!(
            !constants_str::UPDATE_CONFIG_PROJECTIONS.is_empty(),
            "4b913df2"
        );
        assert!(is_typed_accessor::<config_lib::domain_types::DatabaseUrl>(
            super::NotificationServiceConfig::notification_database_url
        ));
        assert!(is_typed_accessor::<
            config_lib::domain_types::ServiceSocketAddress,
        >(
            super::NotificationServiceConfig::notification_service_socket_address
        ));
        assert!(
            is_typed_accessor::<config_lib::tracing_format::TracingFormat>(
                super::NotificationServiceConfig::tracing_format
            )
        );
        assert!(is_typed_accessor::<
            config_lib::pg_pool_max_connections::PgPoolMaxConnections,
        >(
            super::NotificationServiceConfig::pg_pool_max_connections
        ));
        assert!(is_typed_accessor::<
            config_lib::request_timeout_seconds::RequestTimeoutSeconds,
        >(
            super::NotificationServiceConfig::request_timeout_seconds
        ));
    }
}
