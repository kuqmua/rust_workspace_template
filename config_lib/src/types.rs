#[derive(Debug, Clone, Copy, PartialEq, Eq, newtype::Display)]
struct TracingLevelName(&'static str);
#[derive(Debug)]
struct StdEnvVarResult(Result<String, std::env::VarError>);
#[derive(Debug, Clone, Copy)]
struct EnvVarNameRef<'name_lt>(&'name_lt str);
#[derive(Debug, Clone, Copy)]
struct EnvVarValueRef<'value_lt>(&'value_lt str);
#[derive(Debug, Clone, Copy)]
struct ParseCtxRef(&'static str);
#[derive(Debug, Clone, PartialEq, Eq, newtype::AsRefStr, newtype::Display)]
struct EnvParseError(String);
impl From<super::ConfigLibStringWrapperTryFromStringError> for EnvParseError {
    fn from(value: super::ConfigLibStringWrapperTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for EnvParseError {
    type Error = super::ConfigLibStringWrapperTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > super::CONFIG_LIB_STRING_WRAPPER_MAX_LEN {
            return Err(Self::Error::TooLong {
                len: value.len(),
                max: super::CONFIG_LIB_STRING_WRAPPER_MAX_LEN,
            });
        }
        Ok(Self(value))
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
    optml::Optml,
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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TracingFormat {
    Json,
    #[default]
    Text,
}
impl TracingLevel {
    const fn as_str(self) -> TracingLevelName {
        TracingLevelName(match self {
            Self::Trace => str_constants::CONFIG_TRACING_TRACE,
            Self::Debug => str_constants::CONFIG_TRACING_DEBUG,
            Self::Info => str_constants::CONFIG_TRACING_INFO,
            Self::Warn => str_constants::CONFIG_TRACING_WARN,
            Self::Error => str_constants::CONFIG_TRACING_ERROR,
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
    optml::Optml,
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
        let parsed = Self::parse_src_place_type_from_env_var(StdEnvVarResult(std::env::var(
            str_constants::ENV_NAMES_SRC_PLACE_TYPE,
        )));
        match parsed {
            Ok(v) => v,
            Err(message) => {
                eprintln!(
                    "using default SrcPlaceType::{default:#?} ({message}) {}",
                    str_constants::CONFIG_SRC_PLACE_TYPE_FIX_MSG
                );
                default
            }
        }
    }
    #[allow(clippy::single_call_fn)] // helper keeps env-read error context centralized and deterministic for tests
    fn parse_src_place_type_from_env_var(v: StdEnvVarResult) -> Result<Self, EnvParseError> {
        parse_from_env_var_from_str(
            v,
            EnvVarNameRef(str_constants::ENV_NAMES_SRC_PLACE_TYPE),
            ParseCtxRef(str_constants::CONFIG_SRC_PLACE_TYPE_PARSE_CTX),
        )
    }
}
#[allow(clippy::single_call_fn)] // helper centralizes env var context mapping for string parsers and is reused by enum parsing
fn parse_from_env_var_with<T>(
    env_v: StdEnvVarResult,
    env_var_name: EnvVarNameRef<'_>,
    parse: impl FnOnce(EnvVarValueRef<'_>) -> Result<T, EnvParseError>,
) -> Result<T, EnvParseError> {
    let raw_v = env_v.0.map_err(|error| {
        EnvParseError::try_from(format!("std::env::var(\"{}\"): {error}", env_var_name.0))
            .unwrap_or_else(EnvParseError::from)
    })?;
    parse(EnvVarValueRef(&raw_v))
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
    T::from_str(v.0).map_err(|error| {
        EnvParseError::try_from(format!("{}: {error}", parse_ctx.0))
            .unwrap_or_else(EnvParseError::from)
    })
}
#[allow(clippy::single_call_fn)] // helper composes env var read + std::str::FromStr context mapping for reuse across enum env parsers
fn parse_from_env_var_from_str<T>(
    env_v: StdEnvVarResult,
    env_var_name: EnvVarNameRef<'_>,
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
    #[allow(clippy::single_call_fn)] // shared helper keeps variant parse/display assertions centralized across enum parser tests
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
        let _error = <super::TracingLevel as std::str::FromStr>::from_str(str_constants::BAD)
            .expect_err(str_constants::VALUE_9F8D72A1);
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
        let _error = <super::SrcPlaceType as std::str::FromStr>::from_str(str_constants::BAD)
            .expect_err(str_constants::VALUE_8D6F70BB);
    }
    #[test]
    fn src_place_type_default_is_github() {
        assert_eq!(super::SrcPlaceType::default(), super::SrcPlaceType::Github);
    }
    #[test]
    fn src_place_type_parse_error_contains_expected_context() {
        let error =
            <super::SrcPlaceType as std::str::FromStr>::from_str(str_constants::UNKNOWN_ALT)
                .expect_err(str_constants::F2CC7D6B);
        assert!(error.contains("Unknown value"));
        assert!(error.contains("Allowed values:"));
    }
    #[test]
    fn parse_src_place_type_env_value_parses_case_insensitively() {
        let parsed = super::parse_from_str_with_ctx::<super::SrcPlaceType>(
            super::EnvVarValueRef(str_constants::GITHUB),
            super::ParseCtxRef(str_constants::CONFIG_SRC_PLACE_TYPE_PARSE_CTX),
        );
        assert_eq!(parsed, Ok(super::SrcPlaceType::Github));
    }
    #[test]
    fn parse_src_place_type_env_value_wraps_parse_context() {
        let error = super::parse_from_str_with_ctx::<super::SrcPlaceType>(
            super::EnvVarValueRef(str_constants::BAD),
            super::ParseCtxRef(str_constants::CONFIG_SRC_PLACE_TYPE_PARSE_CTX),
        )
        .expect_err(str_constants::VALUE_8C9F2A17);
        assert!(
            error
                .as_ref()
                .contains("<SrcPlaceType as std::str::FromStr>::from_str(&v):")
        );
        assert!(error.as_ref().contains("Unknown value: bad"));
    }
    #[test]
    fn parse_from_env_var_with_wraps_missing_var_context() {
        let parsed = super::parse_from_env_var_with(
            super::StdEnvVarResult(Err(std::env::VarError::NotPresent)),
            super::EnvVarNameRef(str_constants::ENV_NAMES_SRC_PLACE_TYPE),
            |_v| Ok(()),
        );
        let error = parsed.expect_err(str_constants::D2F3B74A);
        assert!(error.as_ref().contains("std::env::var(\"SRC_PLACE_TYPE\")"));
    }
    #[test]
    fn parse_from_env_var_with_passes_value_into_parse_callback() {
        let parsed = super::parse_from_env_var_with(
            super::StdEnvVarResult(Ok(String::from(str_constants::SRC_ALT))),
            super::EnvVarNameRef(str_constants::ENV_NAMES_SRC_PLACE_TYPE),
            |v| Ok(v.0.to_owned()),
        );
        assert_eq!(parsed, Ok(String::from("src")));
    }
    #[test]
    fn parse_from_env_var_from_str_parses_bool_when_input_is_valid() {
        let parsed = super::parse_from_env_var_from_str::<bool>(
            super::StdEnvVarResult(Ok(String::from(str_constants::TRUE))),
            super::EnvVarNameRef(str_constants::ENV_NAMES_SRC_PLACE_TYPE),
            super::ParseCtxRef(str_constants::BOOL_PARSE),
        );
        assert_eq!(parsed, Ok(true));
    }
    #[test]
    fn parse_from_env_var_from_str_wraps_context_when_parse_fails() {
        let error = super::parse_from_env_var_from_str::<bool>(
            super::StdEnvVarResult(Ok(String::from(str_constants::X))),
            super::EnvVarNameRef(str_constants::ENV_NAMES_SRC_PLACE_TYPE),
            super::ParseCtxRef(str_constants::BOOL_PARSE),
        )
        .expect_err(str_constants::VALUE_7E4B3F19);
        assert!(error.as_ref().contains("bool parse:"));
    }
    #[test]
    fn parse_src_place_type_from_env_var_wraps_missing_var_context() {
        let error = super::SrcPlaceType::parse_src_place_type_from_env_var(super::StdEnvVarResult(
            Err(std::env::VarError::NotPresent),
        ))
        .expect_err(str_constants::VALUE_5A83F2BE);
        assert!(error.as_ref().contains("std::env::var(\"SRC_PLACE_TYPE\")"));
    }
    #[test]
    fn parse_src_place_type_from_env_var_parses_ok_value() {
        let parsed = super::SrcPlaceType::parse_src_place_type_from_env_var(
            super::StdEnvVarResult(Ok(String::from(str_constants::SRC_ALT))),
        );
        assert_eq!(parsed, Ok(super::SrcPlaceType::Src));
    }
    #[test]
    fn parse_from_str_with_ctx_parses_value_when_input_is_valid() {
        let parsed = super::parse_from_str_with_ctx::<bool>(
            super::EnvVarValueRef(str_constants::TRUE),
            super::ParseCtxRef(str_constants::BOOL_PARSE),
        );
        assert_eq!(parsed, Ok(true));
    }
    #[test]
    fn parse_from_str_with_ctx_wraps_context_when_parsing_fails() {
        let error = super::parse_from_str_with_ctx::<bool>(
            super::EnvVarValueRef(str_constants::X),
            super::ParseCtxRef(str_constants::BOOL_PARSE),
        )
        .expect_err(str_constants::VALUE_13FE8A6D);
        assert!(error.as_ref().contains("bool parse:"));
    }
}
