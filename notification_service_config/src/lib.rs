#[derive(Debug, try_from_env::TryFromEnv, optml::Optml)]
pub struct Config {
    #[config(secret)]
    notification_database_url: config_lib::DatabaseUrl,
    notification_service_socket_address: config_lib::ServiceSocketAddress,
    tracing_format: config_lib::types::TracingFormat,
}
impl Config {
    #[must_use]
    pub const fn notification_database_url(&self) -> &config_lib::DatabaseUrl {
        &self.notification_database_url
    }
    #[must_use]
    pub const fn notification_service_socket_address(&self) -> &config_lib::ServiceSocketAddress {
        &self.notification_service_socket_address
    }
    #[must_use]
    pub const fn tracing_format(&self) -> config_lib::types::TracingFormat {
        self.tracing_format
    }
}
