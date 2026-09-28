proc_macro_config_lib_impl_try_from_non_empty_string::impl_try_from_non_empty_string!(
    CorsAllowOrigin,
    TryFromStdEnvVarOkCorsAllowOriginError
);
proc_macro_config_lib_impl_try_from_non_empty_string::impl_try_from_non_empty_string!(
    TrustedProxyRangesText,
    TryFromStdEnvVarOkTrustedProxyRangesTextError
);
proc_macro_config_lib_impl_try_from_secret_url::impl_try_from_secret_url!(
    DatabaseUrl,
    TryFromStdEnvVarOkDatabaseUrlError
);
proc_macro_config_lib_impl_try_from_parse::impl_try_from_parse!(
    EnableApiGitCommitCheck,
    TryFromStdEnvVarOkEnableApiGitCommitCheckError,
    bool,
    BoolParsing,
    bool_parsing,
    std::str::ParseBoolError,
    Clone,
    Copy
);
proc_macro_config_lib_impl_try_from_secret_url::impl_try_from_secret_url!(
    MongoUrl,
    TryFromStdEnvVarOkMongoUrlError
);

proc_macro_config_lib_impl_try_from_secret_url::impl_try_from_secret_url!(
    RedisUrl,
    TryFromStdEnvVarOkRedisUrlError
);
proc_macro_config_lib_impl_try_from_parse::impl_try_from_parse!(
    ServiceSocketAddress,
    TryFromStdEnvVarOkServiceSocketAddressError,
    std::net::SocketAddr,
    StdNetSocketAddr,
    std_net_socket_addr,
    std::net::AddrParseError,
    Clone,
    Copy
);
proc_macro_config_lib_impl_try_from_parse_string_error::impl_try_from_parse_string_error!(
    SourcePlaceType,
    TryFromStdEnvVarOkSourcePlaceTypeError,
    crate::source_place_type::SourcePlaceType,
    AppStateSourcePlaceTypeParsing,
    app_state_source_place_type_parsing
);
proc_macro_config_lib_impl_try_from_non_empty_string::impl_try_from_non_empty_string!(
    StartingCheckLink,
    TryFromStdEnvVarOkStartingCheckLinkError
);
impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for crate::tracing_format::TracingFormat {
    type Error =
        crate::try_from_std_env_var_ok_tracing_format_error::TryFromStdEnvVarOkTracingFormatError;
    fn try_from_std_env_var_ok(
        std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    ) -> Result<Self, Self::Error> {
        if std_env_var_ok.eq_ignore_ascii_case(constants_str::JSON) {
            Ok(Self::Json)
        } else if std_env_var_ok.eq_ignore_ascii_case(constants_str::TRACING_FORMAT_TEXT) {
            Ok(Self::Text)
        } else {
            Err(Self::Error::Unknown)
        }
    }
}
proc_macro_config_lib_impl_try_from_parse_string_error::impl_try_from_parse_string_error!(
    TracingLevel,
    TryFromStdEnvVarOkTracingLevelError,
    crate::tracing_level::TracingLevel,
    AppStateTracingLevelParsing,
    app_state_tracing_type_parsing
);
