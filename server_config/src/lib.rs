#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    cors_allow_origin: config_lib::CorsAllowOrigin,
    database_url: config_lib::DatabaseUrl,
    enable_api_git_commit_check: config_lib::EnableApiGitCommitCheck,
    maximum_size_of_http_body_in_bytes: config_lib::MaximumSizeOfHttpBodyInBytes,
    pg_pool_max_connections: config_lib::PgPoolMaxConnections,
    service_socket_address: config_lib::ServiceSocketAddress,
    src_place_type: config_lib::SrcPlaceType,
    timezone: config_lib::Timezone,
    tracing_level: config_lib::TracingLevel,
}

impl Config {
    #[must_use]
    pub fn default_config() -> Self {
        Self {
            cors_allow_origin:
                <config_lib::CorsAllowOrigin as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                config_lib::EnvVarName::CorsAllowOrigin,
            ),
            database_url:
                <config_lib::DatabaseUrl as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                config_lib::EnvVarName::DatabaseUrl,
            ),
            enable_api_git_commit_check:
                <config_lib::EnableApiGitCommitCheck as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                    config_lib::EnvVarName::EnableApiGitCommitCheck,
                ),
            maximum_size_of_http_body_in_bytes:
                <config_lib::MaximumSizeOfHttpBodyInBytes as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                    config_lib::EnvVarName::MaximumSizeOfHttpBodyInBytes,
                ),
            pg_pool_max_connections:
                <config_lib::PgPoolMaxConnections as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                config_lib::EnvVarName::PgPoolMaxConnections,
            ),
            service_socket_address:
                <config_lib::ServiceSocketAddress as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                config_lib::EnvVarName::ServiceSocketAddress,
            ),
            src_place_type:
                <config_lib::SrcPlaceType as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                config_lib::EnvVarName::SrcPlaceType,
            ),
            timezone:
                <config_lib::Timezone as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                    config_lib::EnvVarName::Timezone,
                ),
            tracing_level:
                <config_lib::TracingLevel as config_lib::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                config_lib::EnvVarName::TracingLevel,
            ),
        }
    }
}

impl app_state::GetCorsAllowOrigin for Config {
    fn get_cors_allow_origin(&self) -> &config_lib::CorsAllowOrigin {
        &self.cors_allow_origin
    }
}

impl app_state::GetDatabaseUrl for Config {
    fn get_database_url(&self) -> &config_lib::DatabaseUrl {
        &self.database_url
    }
}

impl app_state::GetEnableApiGitCommitCheck for Config {
    fn get_enable_api_git_commit_check(&self) -> &config_lib::EnableApiGitCommitCheck {
        &self.enable_api_git_commit_check
    }
}

impl app_state::GetMaximumSizeOfHttpBodyInBytes for Config {
    fn get_maximum_size_of_http_body_in_bytes(&self) -> &config_lib::MaximumSizeOfHttpBodyInBytes {
        &self.maximum_size_of_http_body_in_bytes
    }
}

impl app_state::GetPgPoolMaxConnections for Config {
    fn get_pg_pool_max_connections(&self) -> &config_lib::PgPoolMaxConnections {
        &self.pg_pool_max_connections
    }
}

impl app_state::GetServiceSocketAddress for Config {
    fn get_service_socket_address(&self) -> &config_lib::ServiceSocketAddress {
        &self.service_socket_address
    }
}

impl app_state::GetSrcPlaceType for Config {
    fn get_src_place_type(&self) -> &config_lib::SrcPlaceType {
        &self.src_place_type
    }
}

impl app_state::GetTimezone for Config {
    fn get_timezone(&self) -> &config_lib::Timezone {
        &self.timezone
    }
}

impl app_state::GetTracingLevel for Config {
    fn get_tracing_level(&self) -> &config_lib::TracingLevel {
        &self.tracing_level
    }
}
