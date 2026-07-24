#[allow(
    clippy::arbitrary_source_item_ordering,
    reason = "fields are ordered by decreasing alignment as enforced by optml"
)]
#[derive(Debug, try_from_env::TryFromEnv, optml::Optml)]
#[config(env_example)]
pub struct Config {
    #[config(secret)]
    #[config(getter)]
    #[config(
        example = "postgres://notification_service:change-me@127.0.0.1:5432/notification_service"
    )]
    notification_database_url: config_lib::DatabaseUrl,
    #[config(getter)]
    #[config(example = "30")]
    request_timeout_seconds: config_lib::RequestTimeoutSeconds,
    #[config(getter)]
    #[config(example = "127.0.0.1:8081")]
    notification_service_socket_address: config_lib::ServiceSocketAddress,
    #[config(getter)]
    #[config(example = "10")]
    pg_pool_max_connections: config_lib::PgPoolMaxConnections,
    #[config(getter)]
    #[config(example = "text")]
    tracing_format: config_lib::types::TracingFormat,
}

#[cfg(test)]
mod tests {
    #[test]
    fn derive_generates_typed_getters() {
        fn is_typed_getter<Value>(_getter: fn(&super::Config) -> &Value) -> bool {
            true
        }

        assert!(
            !str_constants::UPDATE_CONFIG_PROJECTIONS.is_empty(),
            "4b913df2"
        );
        assert!(is_typed_getter::<config_lib::DatabaseUrl>(
            super::Config::notification_database_url
        ));
        assert!(is_typed_getter::<config_lib::ServiceSocketAddress>(
            super::Config::notification_service_socket_address
        ));
        assert!(is_typed_getter::<config_lib::types::TracingFormat>(
            super::Config::tracing_format
        ));
        assert!(is_typed_getter::<config_lib::PgPoolMaxConnections>(
            super::Config::pg_pool_max_connections
        ));
        assert!(is_typed_getter::<config_lib::RequestTimeoutSeconds>(
            super::Config::request_timeout_seconds
        ));
    }
}
