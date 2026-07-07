#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(Debug, config_lib::TryFromEnv, optml::Optml)]
pub struct Config {
    //todo mb auto gen .env and docker-compose environment variables. and mb write in directly into files
    pub cors_allow_origin: config_lib::CorsAllowOrigin,
    pub database_url: config_lib::DatabaseUrl,
    pub maximum_size_of_http_body_in_bytes: config_lib::MaximumSizeOfHttpBodyInBytes,
    pub service_socket_address: config_lib::ServiceSocketAddress,
    pub pg_pool_max_connections: config_lib::PgPoolMaxConnections,
    pub timezone: config_lib::Timezone,
    pub src_place_type: config_lib::SrcPlaceType,
    pub tracing_level: config_lib::TracingLevel,
    pub enable_api_git_commit_check: config_lib::EnableApiGitCommitCheck,
}
impl app_state::GetCorsAllowOrigin for Config {
    fn get_cors_allow_origin(&self) -> &String {
        &self.cors_allow_origin.0
    }
}
impl app_state::GetDatabaseUrl for Config {
    fn get_database_url(&self) -> &secrecy::SecretBox<String> {
        &self.database_url.0
    }
}
impl app_state::GetMaximumSizeOfHttpBodyInBytes for Config {
    fn get_maximum_size_of_http_body_in_bytes(&self) -> &usize {
        &self.maximum_size_of_http_body_in_bytes.0
    }
}
impl app_state::GetServiceSocketAddress for Config {
    fn get_service_socket_address(&self) -> &std::net::SocketAddr {
        &self.service_socket_address.0
    }
}
impl app_state::GetPgPoolMaxConnections for Config {
    fn get_pg_pool_max_connections(&self) -> &u32 {
        &self.pg_pool_max_connections.0
    }
}
impl app_state::GetTimezone for Config {
    fn get_timezone(&self) -> &chrono::FixedOffset {
        &self.timezone.0
    }
}
impl app_state::GetSrcPlaceType for Config {
    fn get_src_place_type(&self) -> &config_lib::types::SrcPlaceType {
        &self.src_place_type.0
    }
}
impl app_state::GetTracingLevel for Config {
    fn get_tracing_level(&self) -> &config_lib::types::TracingLevel {
        &self.tracing_level.0
    }
}
impl app_state::GetEnableApiGitCommitCheck for Config {
    fn get_enable_api_git_commit_check(&self) -> &bool {
        &self.enable_api_git_commit_check.0
    }
}
impl app_state::GetCorsAllowOrigin for &Config {
    fn get_cors_allow_origin(&self) -> &String {
        Config::get_cors_allow_origin(self)
    }
}
impl app_state::GetDatabaseUrl for &Config {
    fn get_database_url(&self) -> &secrecy::SecretBox<String> {
        Config::get_database_url(self)
    }
}
impl app_state::GetMaximumSizeOfHttpBodyInBytes for &Config {
    fn get_maximum_size_of_http_body_in_bytes(&self) -> &usize {
        Config::get_maximum_size_of_http_body_in_bytes(self)
    }
}
impl app_state::GetServiceSocketAddress for &Config {
    fn get_service_socket_address(&self) -> &std::net::SocketAddr {
        Config::get_service_socket_address(self)
    }
}
impl app_state::GetPgPoolMaxConnections for &Config {
    fn get_pg_pool_max_connections(&self) -> &u32 {
        Config::get_pg_pool_max_connections(self)
    }
}
impl app_state::GetTimezone for &Config {
    fn get_timezone(&self) -> &chrono::FixedOffset {
        Config::get_timezone(self)
    }
}
impl app_state::GetSrcPlaceType for &Config {
    fn get_src_place_type(&self) -> &config_lib::types::SrcPlaceType {
        Config::get_src_place_type(self)
    }
}
impl app_state::GetTracingLevel for &Config {
    fn get_tracing_level(&self) -> &config_lib::types::TracingLevel {
        Config::get_tracing_level(self)
    }
}
impl app_state::GetEnableApiGitCommitCheck for &Config {
    fn get_enable_api_git_commit_check(&self) -> &bool {
        Config::get_enable_api_git_commit_check(self)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn generated_getters_return_expected_refs_and_values() {
        let cfg =
            super::Config {
                cors_allow_origin: config_lib::CorsAllowOrigin("*".to_owned()),
                database_url: config_lib::DatabaseUrl(secrecy::SecretBox::new(Box::new(
                    "postgres://db".to_owned(),
                ))),
                maximum_size_of_http_body_in_bytes:
                    config_lib::MaximumSizeOfHttpBodyInBytes::try_from(16_384).expect("0d9e4b7a"),
                service_socket_address: config_lib::ServiceSocketAddress(
                    "127.0.0.1:8080".parse().expect("e7a3d5c1"),
                ),
                pg_pool_max_connections: config_lib::PgPoolMaxConnections::try_from(8)
                    .expect("39a84c10"),
                timezone: config_lib::Timezone(
                    chrono::FixedOffset::east_opt(3i32 * 3_600i32).expect("93cbf4a2"),
                ),
                src_place_type: config_lib::SrcPlaceType(config_lib::types::SrcPlaceType::Github),
                tracing_level: config_lib::TracingLevel(config_lib::types::TracingLevel::Info),
                enable_api_git_commit_check: config_lib::EnableApiGitCommitCheck(true),
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
