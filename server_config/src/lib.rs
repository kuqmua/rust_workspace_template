#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    Debug, config_lib::TryFromEnv, config_lib::GenGetterTraitsForStructFields, optml::Optml,
)]
pub struct Config {
    //todo mb auto gen .env and docker-compose environment variables. and mb write in directly into files
    pub cors_allow_origin: String,
    pub database_url: secrecy::SecretBox<String>,
    pub maximum_size_of_http_body_in_bytes: usize,
    pub service_socket_address: std::net::SocketAddr,
    pub pg_pool_max_connections: u32,
    pub timezone: chrono::FixedOffset,
    pub src_place_type: config_lib::types::SrcPlaceType,
    pub tracing_level: config_lib::types::TracingLevel,
    pub enable_api_git_commit_check: bool,
}
#[cfg(test)]
mod tests {
    #[test]
    fn generated_getters_return_expected_refs_and_values() {
        let cfg = super::Config {
            cors_allow_origin: "*".to_owned(),
            database_url: secrecy::SecretBox::new(Box::new("postgres://db".to_owned())),
            maximum_size_of_http_body_in_bytes: 16_384,
            service_socket_address: "127.0.0.1:8080".parse().expect("e7a3d5c1"),
            pg_pool_max_connections: 8,
            timezone: chrono::FixedOffset::east_opt(3i32 * 3_600i32).expect("93cbf4a2"),
            src_place_type: config_lib::types::SrcPlaceType::Github,
            tracing_level: config_lib::types::TracingLevel::Info,
            enable_api_git_commit_check: true,
        };
        assert_eq!(
            config_lib::GetCorsAllowOrigin::get_cors_allow_origin(&cfg),
            "*"
        );
        assert_eq!(
            secrecy::ExposeSecret::expose_secret(config_lib::GetDatabaseUrl::get_database_url(
                &cfg
            )),
            "postgres://db"
        );
        assert_eq!(
            config_lib::GetMaximumSizeOfHttpBodyInBytes::get_maximum_size_of_http_body_in_bytes(
                &cfg
            ),
            &16_384
        );
        assert_eq!(
            config_lib::GetServiceSocketAddress::get_service_socket_address(&cfg).port(),
            8080
        );
        assert_eq!(
            config_lib::GetPgPoolMaxConnections::get_pg_pool_max_connections(&cfg),
            &8
        );
        assert_eq!(
            config_lib::GetTimezone::get_timezone(&cfg).local_minus_utc(),
            3i32 * 3_600i32
        );
        assert_eq!(
            config_lib::GetSrcPlaceType::get_src_place_type(&cfg),
            &config_lib::types::SrcPlaceType::Github
        );
        assert_eq!(
            config_lib::GetTracingLevel::get_tracing_level(&cfg),
            &config_lib::types::TracingLevel::Info
        );
        assert!(config_lib::GetEnableApiGitCommitCheck::get_enable_api_git_commit_check(&cfg));
    }
}
