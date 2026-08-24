#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::Display,
    newtype::FromInner,
)]
struct TracingLevelName(&'static str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
struct StdEnvVarResult(Result<String, std::env::VarError>);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    PartialEq,
    Eq,
    thiserror::Error,
    newtype::FromInner,
)]
#[error(transparent)]
struct StdEnvVarError(std::env::VarError);
impl TryFrom<Result<String, std::env::VarError>> for StdEnvVarResult {
    type Error = super::ConfigLibStringWrapperTryFromStringError;
    fn try_from(value: Result<String, std::env::VarError>) -> Result<Self, Self::Error> {
        match value {
            Ok(raw_value) => {
                let bounded = super::StdEnvVarOk::try_from(raw_value)?;
                Ok(Self(Ok(bounded.0)))
            }
            Err(error) => Ok(Self(Err(error))),
        }
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
struct EnvVarNameRef<'name_lt>(&'name_lt str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
struct EnvVarValueRef<'value_lt>(&'value_lt str);
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::Display,
    newtype::FromInner,
)]
struct ParseCtxRef(&'static str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, PartialEq, Eq, thiserror::Error)]
enum EnvParseError {
    #[error("environment variable value exceeds the size limit")]
    ValueTooLong {
        #[source]
        source: super::ConfigLibStringWrapperTryFromStringError,
    },
    #[error("std::env::var(\"{name}\")")]
    Read {
        name: super::EnvVarName,
        #[source]
        source: StdEnvVarError,
    },
    #[error("{context}: {detail}")]
    Parse {
        context: ParseCtxRef,
        detail: to_err_string::ErrorText,
    },
}
impl From<super::ConfigLibStringWrapperTryFromStringError> for EnvParseError {
    fn from(value: super::ConfigLibStringWrapperTryFromStringError) -> Self {
        Self::ValueTooLong { source: value }
    }
}
#[allow(clippy::arbitrary_source_item_ordering)]
#[derive(
    std::fmt::Debug,
    Default,
    Clone,
    Copy,
    strum_macros::EnumIter,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::EnumFromStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum TracingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    #[default]
    Error,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Default, Clone, Copy, PartialEq, Eq,
)]
pub enum TracingFormat {
    Json,
    #[default]
    Text,
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Default, Clone, Copy, PartialEq, Eq,
)]
pub enum SvcMode {
    Migrate,
    #[default]
    Serve,
}
impl TracingLevel {
    fn as_str(self) -> TracingLevelName {
        TracingLevelName::from(match self {
            Self::Trace => constants_str::CONFIG_TRACING_TRACE,
            Self::Debug => constants_str::CONFIG_TRACING_DEBUG,
            Self::Info => constants_str::CONFIG_TRACING_INFO,
            Self::Warn => constants_str::CONFIG_TRACING_WARN,
            Self::Error => constants_str::CONFIG_TRACING_ERROR,
        })
    }
}
impl std::fmt::Display for TracingLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (*self).as_str().0)
    }
}
#[derive(
    std::fmt::Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum_macros::Display,
    strum_macros::EnumIter,
    serde::Serialize,
    serde::Deserialize,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::EnumFromStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum SrcPlaceType {
    #[default]
    Github,
    Src,
}
impl SrcPlaceType {
    #[must_use]
    pub fn from_env_or_default() -> Self {
        let default = Self::default();
        if let Err(error) = dotenv::dotenv() {
            eprintln!("dotenv::dotenv() failed in SrcPlaceType::from_env_or_default: {error}");
        }
        let parsed =
            StdEnvVarResult::try_from(std::env::var(constants_str::ENV_NAMES_SRC_PLACE_TYPE))
                .map_err(EnvParseError::from)
                .and_then(Self::parse_src_place_type_from_env_var);
        match parsed {
            Ok(v) => v,
            Err(message) => {
                eprintln!(
                    "using default SrcPlaceType::{default:#?} ({message}) {}",
                    constants_str::CONFIG_SRC_PLACE_TYPE_FIX_MSG
                );
                default
            }
        }
    }
    #[allow(clippy::single_call_fn)] // helper keeps env-read error context centralized and deterministic for tests
    fn parse_src_place_type_from_env_var(v: StdEnvVarResult) -> Result<Self, EnvParseError> {
        parse_from_env_var_from_str(
            v,
            EnvVarNameRef::from(constants_str::ENV_NAMES_SRC_PLACE_TYPE),
            ParseCtxRef::from(constants_str::CONFIG_SRC_PLACE_TYPE_PARSE_CTX),
        )
    }
}
#[allow(clippy::single_call_fn)] // helper centralizes env var context mapping for string parsers and is reused by enum parsing
fn parse_from_env_var_with<T>(
    env_v: StdEnvVarResult,
    env_var_name: EnvVarNameRef<'static>,
    parse: impl FnOnce(EnvVarValueRef<'_>) -> Result<T, EnvParseError>,
) -> Result<T, EnvParseError> {
    let raw_v = env_v.0.map_err(|source| EnvParseError::Read {
        name: super::EnvVarName::try_from(env_var_name.0.to_owned())
            .unwrap_or_else(super::EnvVarName::from),
        source: StdEnvVarError::from(source),
    })?;
    parse(EnvVarValueRef::from(raw_v.as_str()))
}
#[allow(clippy::single_call_fn)] // helper centralizes std::str::FromStr context formatting and keeps per-type parsing helpers minimal
fn parse_from_str_with_ctx<T>(
    v: EnvVarValueRef<'_>,
    parse_ctx: ParseCtxRef,
) -> Result<T, EnvParseError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    T::from_str(v.0).map_err(|error| EnvParseError::Parse {
        context: parse_ctx,
        detail: to_err_string::ErrorText::try_from(error.to_string())
            .unwrap_or_else(to_err_string::ErrorText::from),
    })
}
#[allow(clippy::single_call_fn)] // helper composes env var read + std::str::FromStr context mapping for reuse across enum env parsers
fn parse_from_env_var_from_str<T>(
    env_v: StdEnvVarResult,
    env_var_name: EnvVarNameRef<'static>,
    parse_ctx: ParseCtxRef,
) -> Result<T, EnvParseError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    parse_from_env_var_with(env_v, env_var_name, |v| {
        parse_from_str_with_ctx(v, parse_ctx)
    })
}
#[cfg(test)]
mod tests {
    fn env_result(value: Result<String, std::env::VarError>) -> super::StdEnvVarResult {
        super::StdEnvVarResult::try_from(value).expect("a4aa0c6f env_result invariant must hold")
    }
    #[test]
    fn environment_result_rejects_values_above_shared_limit() {
        let value = constants_str::TEST_JWT_SECRET_CHARACTER_A.repeat(
            super::super::CONFIG_LIB_STRING_WRAPPER_MAX_LEN.saturating_add(constants_usize::ONE),
        );
        let Err(_error) = super::StdEnvVarResult::try_from(Ok(value)) else {
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
            super::EnvVarNameRef::from(constants_str::ENV_NAMES_SRC_PLACE_TYPE),
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
            super::EnvVarNameRef::from(constants_str::ENV_NAMES_SRC_PLACE_TYPE),
            |v| Ok(v.0.to_owned()),
        );
        assert_eq!(parsed, Ok(String::from("src")));
    }
    #[test]
    fn parse_from_env_var_from_str_parses_bool_when_input_is_valid() {
        let parsed = super::parse_from_env_var_from_str::<bool>(
            env_result(Ok(String::from(constants_str::TRUE))),
            super::EnvVarNameRef::from(constants_str::ENV_NAMES_SRC_PLACE_TYPE),
            super::ParseCtxRef::from(constants_str::BOOL_PARSE),
        );
        assert_eq!(parsed, Ok(true));
    }
    #[test]
    fn parse_from_env_var_from_str_wraps_context_when_parse_fails() {
        let error = super::parse_from_env_var_from_str::<bool>(
            env_result(Ok(String::from(constants_str::X))),
            super::EnvVarNameRef::from(constants_str::ENV_NAMES_SRC_PLACE_TYPE),
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
