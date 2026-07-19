#[derive(Debug, try_from_env::TryFromEnv, optml::Optml)]
pub struct Config {
    #[config(secret)]
    #[config(getter)]
    notification_database_url: config_lib::DatabaseUrl,
    #[config(getter)]
    notification_service_socket_address: config_lib::ServiceSocketAddress,
    #[config(getter)]
    tracing_format: config_lib::types::TracingFormat,
}

#[cfg(test)]
mod tests {
    #[test]
    fn derive_generates_typed_getters() {
        fn is_typed_getter<Value>(_getter: fn(&super::Config) -> &Value) -> bool {
            true
        }
        assert!(is_typed_getter::<config_lib::DatabaseUrl>(
            super::Config::notification_database_url
        ));
        assert!(is_typed_getter::<config_lib::ServiceSocketAddress>(
            super::Config::notification_service_socket_address
        ));
        assert!(is_typed_getter::<config_lib::types::TracingFormat>(
            super::Config::tracing_format
        ));
    }
}
