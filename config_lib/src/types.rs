const TRACING_LEVEL_PARSE_PAIRS: [(&str, TracingLevel); 5] = [
    ("trace", TracingLevel::Trace),
    ("debug", TracingLevel::Debug),
    ("info", TracingLevel::Info),
    ("warn", TracingLevel::Warn),
    ("er", TracingLevel::Er),
];
const SRC_PLACE_TYPE_PARSE_PAIRS: [(&str, SrcPlaceType); 2] =
    [("github", SrcPlaceType::Github), ("src", SrcPlaceType::Src)];
const SRC_PLACE_TYPE_ENV_VAR: &str = "SRC_PLACE_TYPE";
const SRC_PLACE_TYPE_PARSE_CTX: &str = "<SrcPlaceType as std::str::FromStr>::from_str(&v)";
const SRC_PLACE_TYPE_FIX_MSG: &str =
    "You can set environment variable SRC_PLACE_TYPE to be eq \"src\" or \"github\"";
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
)]
pub enum TracingLevel {
    Trace,
    Debug,
    Info,
    Warn,
    #[default]
    Er,
}
impl TracingLevel {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Er => "er",
        }
    }
}
impl std::str::FromStr for TracingLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::str_from_enum_macros::impl_from_str_for_enum_helper(s, &TRACING_LEVEL_PARSE_PAIRS)
    }
}
impl std::fmt::Display for TracingLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (*self).as_str())
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
    serde::Serialize,
    serde::Deserialize,
    optml::Optml,
)]
pub enum SrcPlaceType {
    #[default]
    Github,
    Src,
}
impl std::str::FromStr for SrcPlaceType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::str_from_enum_macros::impl_from_str_for_enum_helper(s, &SRC_PLACE_TYPE_PARSE_PAIRS)
    }
}
impl SrcPlaceType {
    #[must_use]
    pub fn from_env_or_dflt() -> Self {
        let dflt = Self::default();
        if let Err(er) = dotenv::dotenv() {
            eprintln!("dotenv::dotenv() failed in SrcPlaceType::from_env_or_dflt: {er}");
        }
        let parsed = Self::parse_src_place_type_from_env_var(std::env::var(SRC_PLACE_TYPE_ENV_VAR));
        match parsed {
            Ok(v) => v,
            Err(msg) => {
                eprintln!("using dflt SrcPlaceType::{dflt:#?} ({msg}) {SRC_PLACE_TYPE_FIX_MSG}");
                dflt
            }
        }
    }
    #[allow(clippy::single_call_fn)] // helper keeps env-read error context centralized and deterministic for tests
    fn parse_src_place_type_from_env_var(
        v: Result<String, std::env::VarError>,
    ) -> Result<Self, String> {
        parse_from_env_var_from_str(v, SRC_PLACE_TYPE_ENV_VAR, SRC_PLACE_TYPE_PARSE_CTX)
    }
}
#[allow(clippy::single_call_fn)] // helper centralizes env var context mapping for string parsers and is reused by enum parsing
fn parse_from_env_var_with<T>(
    env_v: Result<String, std::env::VarError>,
    env_var_name: &str,
    parse: impl FnOnce(&str) -> Result<T, String>,
) -> Result<T, String> {
    let raw_v = env_v.map_err(|er| format!("std::env::var(\"{env_var_name}\"): {er}"))?;
    parse(&raw_v)
}
#[allow(clippy::single_call_fn)] // helper centralizes std::str::FromStr context formatting and keeps per-type parsing helpers minimal
fn parse_from_str_with_ctx<T>(v: &str, parse_ctx: &'static str) -> Result<T, String>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    T::from_str(v).map_err(|er| format!("{parse_ctx}: {er}"))
}
#[allow(clippy::single_call_fn)] // helper composes env var read + std::str::FromStr context mapping for reuse across enum env parsers
fn parse_from_env_var_from_str<T>(
    env_v: Result<String, std::env::VarError>,
    env_var_name: &str,
    parse_ctx: &'static str,
) -> Result<T, String>
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
    #[allow(clippy::single_call_fn)] // shared helper keeps parse-pair assertions centralized across enum parser tests
    fn assert_from_str_matches_pairs<T>(pairs: &[(&str, T)])
    where
        T: Copy + Eq + std::fmt::Debug + std::str::FromStr<Err = String>,
    {
        for (name, value) in pairs {
            assert_eq!(T::from_str(name), Ok(*value));
        }
    }
    #[allow(clippy::single_call_fn)] // shared helper keeps tracing-level parse/display roundtrip assertions reusable across tests
    fn assert_tracing_level_roundtrip(name: &str, level: super::TracingLevel) {
        assert_eq!(
            <super::TracingLevel as std::str::FromStr>::from_str(name),
            Ok(level)
        );
        assert_eq!(level.to_string(), name);
    }
    #[allow(clippy::single_call_fn)] // shared helper keeps parse+display roundtrip checks reusable for pair-based enum tests
    fn assert_parse_display_roundtrip_pairs<T>(pairs: &[(&str, T)])
    where
        T: Copy + Eq + std::fmt::Debug + std::fmt::Display + std::str::FromStr<Err = String>,
    {
        for (name, value) in pairs {
            assert_eq!(T::from_str(name), Ok(*value));
            assert_eq!(value.to_string(), *name);
        }
    }
    #[test]
    fn tracing_level_display_is_stable() {
        assert_parse_display_roundtrip_pairs(&super::TRACING_LEVEL_PARSE_PAIRS);
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
        for (name, level) in super::TRACING_LEVEL_PARSE_PAIRS {
            assert_tracing_level_roundtrip(name, level);
        }
    }
    #[test]
    fn src_place_type_from_str_roundtrip_is_stable_for_all_variants() {
        assert_from_str_matches_pairs(&super::SRC_PLACE_TYPE_PARSE_PAIRS);
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
            "GiThUb",
            super::SRC_PLACE_TYPE_PARSE_CTX,
        );
        assert_eq!(parsed, Ok(super::SrcPlaceType::Github));
    }
    #[test]
    fn parse_src_place_type_env_value_wraps_parse_context() {
        let er = super::parse_from_str_with_ctx::<super::SrcPlaceType>(
            "bad",
            super::SRC_PLACE_TYPE_PARSE_CTX,
        )
        .expect_err("8c9f2a17");
        assert!(er.contains("<SrcPlaceType as std::str::FromStr>::from_str(&v):"));
        assert!(er.contains("Unknown value: bad"));
    }
    #[test]
    fn parse_from_env_var_with_wraps_missing_var_context() {
        let parsed = super::parse_from_env_var_with(
            Err(std::env::VarError::NotPresent),
            super::SRC_PLACE_TYPE_ENV_VAR,
            |_v| Ok(()),
        );
        let er = parsed.expect_err("d2f3b74a");
        assert!(er.contains("std::env::var(\"SRC_PLACE_TYPE\")"));
    }
    #[test]
    fn parse_from_env_var_with_passes_value_into_parse_callback() {
        let parsed = super::parse_from_env_var_with(
            Ok(String::from("src")),
            super::SRC_PLACE_TYPE_ENV_VAR,
            |v| Ok(v.to_owned()),
        );
        assert_eq!(parsed, Ok(String::from("src")));
    }
    #[test]
    fn parse_from_env_var_from_str_parses_bool_when_input_is_valid() {
        let parsed = super::parse_from_env_var_from_str::<bool>(
            Ok(String::from("true")),
            super::SRC_PLACE_TYPE_ENV_VAR,
            "bool parse",
        );
        assert_eq!(parsed, Ok(true));
    }
    #[test]
    fn parse_from_env_var_from_str_wraps_context_when_parse_fails() {
        let er = super::parse_from_env_var_from_str::<bool>(
            Ok(String::from("x")),
            super::SRC_PLACE_TYPE_ENV_VAR,
            "bool parse",
        )
        .expect_err("7e4b3f19");
        assert!(er.contains("bool parse:"));
    }
    #[test]
    fn parse_src_place_type_from_env_var_wraps_missing_var_context() {
        let er = super::SrcPlaceType::parse_src_place_type_from_env_var(Err(
            std::env::VarError::NotPresent,
        ))
        .expect_err("5a83f2be");
        assert!(er.contains("std::env::var(\"SRC_PLACE_TYPE\")"));
    }
    #[test]
    fn parse_src_place_type_from_env_var_parses_ok_value() {
        let parsed =
            super::SrcPlaceType::parse_src_place_type_from_env_var(Ok(String::from("src")));
        assert_eq!(parsed, Ok(super::SrcPlaceType::Src));
    }
    #[test]
    fn parse_from_str_with_ctx_parses_value_when_input_is_valid() {
        let parsed = super::parse_from_str_with_ctx::<bool>("true", "bool parse");
        assert_eq!(parsed, Ok(true));
    }
    #[test]
    fn parse_from_str_with_ctx_wraps_context_when_parsing_fails() {
        let er = super::parse_from_str_with_ctx::<bool>("x", "bool parse").expect_err("13fe8a6d");
        assert!(er.contains("bool parse:"));
    }
}
