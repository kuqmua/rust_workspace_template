use super::{
    EnvParseError, EnvVarNameRef, EnvVarResultVarError, ParseCtxRef, parse_from_env_var_with,
    parse_from_str_with_ctx,
};

#[allow(clippy::single_call_fn)] // helper composes env var read + std::str::FromStr context mapping for reuse across enum env parsers
pub(super) fn parse_from_env_var_from_str<T>(
    env_v: EnvVarResultVarError,
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
