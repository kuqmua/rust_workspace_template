use crate::env_parse_error::EnvParseError;
use crate::env_var_error::EnvVarError;
use crate::env_var_result_var_error::EnvVarResultVarError;
#[cfg(test)]
use crate::env_var_value_ref::EnvVarValueRef;
use crate::parse_ctx_ref::ParseCtxRef;
use crate::parse_env_var_name_ref::ParseEnvVarNameRef;
#[cfg(test)]
use crate::parse_from_env_var_from_str::parse_from_env_var_from_str;
#[cfg(test)]
use crate::parse_from_env_var_with::parse_from_env_var_with;
#[cfg(test)]
use crate::parse_from_str_with_ctx::parse_from_str_with_ctx;
pub use crate::src_place_type::*;
pub use crate::svc_mode::SvcMode;
pub use crate::tracing_format::TracingFormat;
pub use crate::tracing_level::*;
use crate::tracing_level_name::TracingLevelName;
use crate::{ConfigLibStringWrapperTryFromStringError, EnvVarName, StdEnvVarOk};

#[cfg(test)]
mod tests {
    fn env_result(value: Result<String, std::env::VarError>) -> super::EnvVarResultVarError {
        super::EnvVarResultVarError::try_from(value)
            .expect("a4aa0c6f env_result invariant must hold")
    }
    #[test]
    fn environment_result_rejects_values_above_shared_limit() {
        let value = constants_str::TEST_JWT_SECRET_CHARACTER_A
            .repeat(crate::CONFIG_LIB_STRING_WRAPPER_MAX_LEN.saturating_add(constants_usize::ONE));
        let Err(_error) = super::EnvVarResultVarError::try_from(Ok(value)) else {
            panic!("3ee39bcb");
        };
    }
    fn assert_parse_display_roundtrip_variants<T>()
    where
        T: Copy
            + Eq
            + std::fmt::Debug
            + std::fmt::Display
            + std::str::FromStr<Err = String>
            + strum::IntoEnumIterator,
    {
        assert!(
            T::iter().all(|value| {
                let name = value.to_string();
                T::from_str(&name) == Ok(value)
            }),
            "7d39b6f2"
        );
    }
    #[test]
    fn tracing_level_display_is_stable() {
        assert_parse_display_roundtrip_variants::<super::TracingLevel>();
    }
    #[test]
    fn tracing_level_from_str_is_case_insensitive() {
        assert_eq!(
            <super::TracingLevel as std::str::FromStr>::from_str("TRACE"),
            Ok(super::TracingLevel::Trace)
        );
        assert_eq!(
            <super::TracingLevel as std::str::FromStr>::from_str("Warn"),
            Ok(super::TracingLevel::Warn)
        );
        let _error = <super::TracingLevel as std::str::FromStr>::from_str(constants_str::BAD)
            .expect_err(constants_str::VALUE_9F8D72A1);
    }
    #[test]
    fn tracing_level_roundtrip_is_stable_for_all_variants() {
        assert_parse_display_roundtrip_variants::<super::TracingLevel>();
    }
    #[test]
    fn src_place_type_from_str_roundtrip_is_stable_for_all_variants() {
        assert_parse_display_roundtrip_variants::<super::SrcPlaceType>();
    }
    #[test]
    fn src_place_type_from_str_accepts_src_value() {
        assert_eq!(
            <super::SrcPlaceType as std::str::FromStr>::from_str("src"),
            Ok(super::SrcPlaceType::Src)
        );
    }
    #[test]
    fn src_place_type_from_str_rejects_unknown_value() {
        let _error = <super::SrcPlaceType as std::str::FromStr>::from_str(constants_str::BAD)
            .expect_err(constants_str::VALUE_8D6F70BB);
    }
    #[test]
    fn src_place_type_default_is_github() {
        assert_eq!(super::SrcPlaceType::default(), super::SrcPlaceType::Github);
    }
    #[test]
    fn src_place_type_parse_error_contains_expected_context() {
        let error =
            <super::SrcPlaceType as std::str::FromStr>::from_str(constants_str::UNKNOWN_ALT)
                .expect_err(constants_str::F2CC7D6B);
        assert!(error.contains("Unknown value"));
        assert!(error.contains("Allowed values:"));
    }
    #[test]
    fn parse_src_place_type_env_value_parses_case_insensitively() {
        let parsed = super::parse_from_str_with_ctx::<super::SrcPlaceType>(
            super::EnvVarValueRef::from(constants_str::GITHUB),
            super::ParseCtxRef::from(constants_str::CONFIG_SRC_PLACE_TYPE_PARSE_CTX),
        );
        assert_eq!(parsed, Ok(super::SrcPlaceType::Github));
    }
    #[test]
    fn parse_src_place_type_env_value_wraps_parse_context() {
        let error = super::parse_from_str_with_ctx::<super::SrcPlaceType>(
            super::EnvVarValueRef::from(constants_str::BAD),
            super::ParseCtxRef::from(constants_str::CONFIG_SRC_PLACE_TYPE_PARSE_CTX),
        )
        .expect_err(constants_str::VALUE_8C9F2A17);
        let error_text = error.to_string();
        assert!(error_text.contains("<SrcPlaceType as std::str::FromStr>::from_str(&v):"));
        assert!(error_text.contains("Unknown value: bad"));
    }
    #[test]
    fn parse_from_env_var_with_wraps_missing_var_context() {
        let parsed = super::parse_from_env_var_with(
            env_result(Err(std::env::VarError::NotPresent)),
            super::ParseEnvVarNameRef::from(constants_str::ENV_NAMES_SRC_PLACE_TYPE),
            |_v| Ok(()),
        );
        let error = parsed.expect_err(constants_str::D2F3B74A);
        assert!(
            error
                .to_string()
                .contains("std::env::var(\"SRC_PLACE_TYPE\")")
        );
        assert_eq!(
            std::error::Error::source(&error)
                .expect(
                    "e6fbfe6b parse_from_env_var_with_wraps_missing_var_context invariant must hold"
                )
                .to_string(),
            "environment variable not found"
        );
    }
    #[test]
    fn parse_from_env_var_with_passes_value_into_parse_callback() {
        let parsed = super::parse_from_env_var_with(
            env_result(Ok(String::from(constants_str::SRC_ALT))),
            super::ParseEnvVarNameRef::from(constants_str::ENV_NAMES_SRC_PLACE_TYPE),
            |v| Ok(v.0.to_owned()),
        );
        assert_eq!(parsed, Ok(String::from("src")));
    }
    #[test]
    fn parse_from_env_var_from_str_parses_bool_when_input_is_valid() {
        let parsed = super::parse_from_env_var_from_str::<bool>(
            env_result(Ok(String::from(constants_str::TRUE))),
            super::ParseEnvVarNameRef::from(constants_str::ENV_NAMES_SRC_PLACE_TYPE),
            super::ParseCtxRef::from(constants_str::BOOL_PARSE),
        );
        assert_eq!(parsed, Ok(true));
    }
    #[test]
    fn parse_from_env_var_from_str_wraps_context_when_parse_fails() {
        let error = super::parse_from_env_var_from_str::<bool>(
            env_result(Ok(String::from(constants_str::X))),
            super::ParseEnvVarNameRef::from(constants_str::ENV_NAMES_SRC_PLACE_TYPE),
            super::ParseCtxRef::from(constants_str::BOOL_PARSE),
        )
        .expect_err(constants_str::VALUE_7E4B3F19);
        assert!(error.to_string().contains("bool parse:"));
    }
    #[test]
    fn parse_src_place_type_from_env_var_wraps_missing_var_context() {
        let error = super::SrcPlaceType::parse_src_place_type_from_env_var(env_result(Err(
            std::env::VarError::NotPresent,
        )))
        .expect_err(constants_str::VALUE_5A83F2BE);
        assert!(
            error
                .to_string()
                .contains("std::env::var(\"SRC_PLACE_TYPE\")")
        );
    }
    #[test]
    fn parse_src_place_type_from_env_var_parses_ok_value() {
        let parsed = super::SrcPlaceType::parse_src_place_type_from_env_var(env_result(Ok(
            String::from(constants_str::SRC_ALT),
        )));
        assert_eq!(parsed, Ok(super::SrcPlaceType::Src));
    }
    #[test]
    fn parse_from_str_with_ctx_parses_value_when_input_is_valid() {
        let parsed = super::parse_from_str_with_ctx::<bool>(
            super::EnvVarValueRef::from(constants_str::TRUE),
            super::ParseCtxRef::from(constants_str::BOOL_PARSE),
        );
        assert_eq!(parsed, Ok(true));
    }
    #[test]
    fn parse_from_str_with_ctx_wraps_context_when_parsing_fails() {
        let error = super::parse_from_str_with_ctx::<bool>(
            super::EnvVarValueRef::from(constants_str::X),
            super::ParseCtxRef::from(constants_str::BOOL_PARSE),
        )
        .expect_err(constants_str::VALUE_13FE8A6D);
        assert!(error.to_string().contains("bool parse:"));
    }
}
