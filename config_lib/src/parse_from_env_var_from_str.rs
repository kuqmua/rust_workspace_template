use crate::{
    EnvParseError, EnvVarResultVarError, ParseCtxRef, ParseEnvVarNameRef, parse_from_env_var_with,
    parse_from_str_with_ctx,
};

#[cfg(test)]
pub(super) fn parse_from_env_var_from_str<T>(
    env_v: EnvVarResultVarError,
    env_var_name: ParseEnvVarNameRef<'static>,
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
