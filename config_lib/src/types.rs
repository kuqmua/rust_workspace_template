const SRC_PLACE_TYPE_ENV_VAR: &str = "SRC_PLACE_TYPE";
const SRC_PLACE_TYPE_PARSE_CTX: &str = "<SrcPlaceType as std::str::FromStr>::from_str(&v)";
const SRC_PLACE_TYPE_FIX_MSG: &str =
    "You can set environment variable SRC_PLACE_TYPE to be eq \"src\" or \"github\"";
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TracingLevelName(&'static str);
impl std::fmt::Display for TracingLevelName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
#[derive(Debug)]
struct StdEnvVarResult(Result<String, std::env::VarError>);
#[derive(Debug, Clone, Copy)]
struct EnvVarNameRef<'name_lt>(&'name_lt str);
#[derive(Debug, Clone, Copy)]
struct EnvVarValueRef<'value_lt>(&'value_lt str);
#[derive(Debug, Clone, Copy)]
struct ParseCtxRef(&'static str);
#[derive(Debug, Clone, PartialEq, Eq)]
struct EnvParseEr(String);
impl From<super::ConfigLibStringWrapperTryFromStringEr> for EnvParseEr {
    fn from(value: super::ConfigLibStringWrapperTryFromStringEr) -> Self {
        Self(value.to_string())
    }
}
impl TryFrom<String> for EnvParseEr {
    type Error = super::ConfigLibStringWrapperTryFromStringEr;
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
impl std::fmt::Display for EnvParseEr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl AsRef<str> for EnvParseEr {
    fn as_ref(&self) -> &str {
        self.0.as_str()
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
    Er,
}
impl TracingLevel {
    const fn as_str(self) -> TracingLevelName {
        TracingLevelName(match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Er => "er",
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
    pub fn from_env_or_dflt() -> Self {
        let dflt = Self::default();
        if let Err(er) = dotenv::dotenv() {
            eprintln!("dotenv::dotenv() failed in SrcPlaceType::from_env_or_dflt: {er}");
        }
        let parsed = Self::parse_src_place_type_from_env_var(StdEnvVarResult(std::env::var(
            SRC_PLACE_TYPE_ENV_VAR,
        )));
        match parsed {
            Ok(v) => v,
            Err(msg) => {
                eprintln!("using dflt SrcPlaceType::{dflt:#?} ({msg}) {SRC_PLACE_TYPE_FIX_MSG}");
                dflt
            }
        }
    }
    #[allow(clippy::single_call_fn)] // helper keeps env-read error context centralized and deterministic for tests
    fn parse_src_place_type_from_env_var(v: StdEnvVarResult) -> Result<Self, EnvParseEr> {
        parse_from_env_var_from_str(
            v,
            EnvVarNameRef(SRC_PLACE_TYPE_ENV_VAR),
            ParseCtxRef(SRC_PLACE_TYPE_PARSE_CTX),
        )
    }
}
#[allow(clippy::single_call_fn)] // helper centralizes env var context mapping for string parsers and is reused by enum parsing
fn parse_from_env_var_with<T>(
    env_v: StdEnvVarResult,
    env_var_name: EnvVarNameRef<'_>,
    parse: impl FnOnce(EnvVarValueRef<'_>) -> Result<T, EnvParseEr>,
) -> Result<T, EnvParseEr> {
    let raw_v = env_v.0.map_err(|er| {
        EnvParseEr::try_from(format!("std::env::var(\"{}\"): {er}", env_var_name.0))
            .unwrap_or_else(EnvParseEr::from)
    })?;
    parse(EnvVarValueRef(&raw_v))
}
#[allow(clippy::single_call_fn)] // helper centralizes std::str::FromStr context formatting and keeps per-type parsing helpers minimal
fn parse_from_str_with_ctx<T>(
    v: EnvVarValueRef<'_>,
    parse_ctx: ParseCtxRef,
) -> Result<T, EnvParseEr>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    T::from_str(v.0).map_err(|er| {
        EnvParseEr::try_from(format!("{}: {er}", parse_ctx.0)).unwrap_or_else(EnvParseEr::from)
    })
}
#[allow(clippy::single_call_fn)] // helper composes env var read + std::str::FromStr context mapping for reuse across enum env parsers
fn parse_from_env_var_from_str<T>(
    env_v: StdEnvVarResult,
    env_var_name: EnvVarNameRef<'_>,
    parse_ctx: ParseCtxRef,
) -> Result<T, EnvParseEr>
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
        let _er =
            <super::TracingLevel as std::str::FromStr>::from_str("bad").expect_err("9f8d72a1");
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
        let _er =
            <super::SrcPlaceType as std::str::FromStr>::from_str("bad").expect_err("8d6f70bb");
    }
    #[test]
    fn src_place_type_default_is_github() {
        assert_eq!(super::SrcPlaceType::default(), super::SrcPlaceType::Github);
    }
    #[test]
    fn src_place_type_parse_error_contains_expected_context() {
        let er =
            <super::SrcPlaceType as std::str::FromStr>::from_str("unknown").expect_err("f2cc7d6b");
        assert!(er.contains("Unknown value"));
        assert!(er.contains("Allowed values:"));
    }
    #[test]
    fn parse_src_place_type_env_value_parses_case_insensitively() {
        let parsed = super::parse_from_str_with_ctx::<super::SrcPlaceType>(
            super::EnvVarValueRef("GiThUb"),
            super::ParseCtxRef(super::SRC_PLACE_TYPE_PARSE_CTX),
        );
        assert_eq!(parsed, Ok(super::SrcPlaceType::Github));
    }
    #[test]
    fn parse_src_place_type_env_value_wraps_parse_context() {
        let er = super::parse_from_str_with_ctx::<super::SrcPlaceType>(
            super::EnvVarValueRef("bad"),
            super::ParseCtxRef(super::SRC_PLACE_TYPE_PARSE_CTX),
        )
        .expect_err("8c9f2a17");
        assert!(
            er.as_ref()
                .contains("<SrcPlaceType as std::str::FromStr>::from_str(&v):")
        );
        assert!(er.as_ref().contains("Unknown value: bad"));
    }
    #[test]
    fn parse_from_env_var_with_wraps_missing_var_context() {
        let parsed = super::parse_from_env_var_with(
            super::StdEnvVarResult(Err(std::env::VarError::NotPresent)),
            super::EnvVarNameRef(super::SRC_PLACE_TYPE_ENV_VAR),
            |_v| Ok(()),
        );
        let er = parsed.expect_err("d2f3b74a");
        assert!(er.as_ref().contains("std::env::var(\"SRC_PLACE_TYPE\")"));
    }
    #[test]
    fn parse_from_env_var_with_passes_value_into_parse_callback() {
        let parsed = super::parse_from_env_var_with(
            super::StdEnvVarResult(Ok(String::from("src"))),
            super::EnvVarNameRef(super::SRC_PLACE_TYPE_ENV_VAR),
            |v| Ok(v.0.to_owned()),
        );
        assert_eq!(parsed, Ok(String::from("src")));
    }
    #[test]
    fn parse_from_env_var_from_str_parses_bool_when_input_is_valid() {
        let parsed = super::parse_from_env_var_from_str::<bool>(
            super::StdEnvVarResult(Ok(String::from("true"))),
            super::EnvVarNameRef(super::SRC_PLACE_TYPE_ENV_VAR),
            super::ParseCtxRef("bool parse"),
        );
        assert_eq!(parsed, Ok(true));
    }
    #[test]
    fn parse_from_env_var_from_str_wraps_context_when_parse_fails() {
        let er = super::parse_from_env_var_from_str::<bool>(
            super::StdEnvVarResult(Ok(String::from("x"))),
            super::EnvVarNameRef(super::SRC_PLACE_TYPE_ENV_VAR),
            super::ParseCtxRef("bool parse"),
        )
        .expect_err("7e4b3f19");
        assert!(er.as_ref().contains("bool parse:"));
    }
    #[test]
    fn parse_src_place_type_from_env_var_wraps_missing_var_context() {
        let er = super::SrcPlaceType::parse_src_place_type_from_env_var(super::StdEnvVarResult(
            Err(std::env::VarError::NotPresent),
        ))
        .expect_err("5a83f2be");
        assert!(er.as_ref().contains("std::env::var(\"SRC_PLACE_TYPE\")"));
    }
    #[test]
    fn parse_src_place_type_from_env_var_parses_ok_value() {
        let parsed = super::SrcPlaceType::parse_src_place_type_from_env_var(
            super::StdEnvVarResult(Ok(String::from("src"))),
        );
        assert_eq!(parsed, Ok(super::SrcPlaceType::Src));
    }
    #[test]
    fn parse_from_str_with_ctx_parses_value_when_input_is_valid() {
        let parsed = super::parse_from_str_with_ctx::<bool>(
            super::EnvVarValueRef("true"),
            super::ParseCtxRef("bool parse"),
        );
        assert_eq!(parsed, Ok(true));
    }
    #[test]
    fn parse_from_str_with_ctx_wraps_context_when_parsing_fails() {
        let er = super::parse_from_str_with_ctx::<bool>(
            super::EnvVarValueRef("x"),
            super::ParseCtxRef("bool parse"),
        )
        .expect_err("13fe8a6d");
        assert!(er.as_ref().contains("bool parse:"));
    }
}
