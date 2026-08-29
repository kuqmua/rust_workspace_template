#[cfg(test)]
pub(super) fn parse_from_env_var_from_str<T>(
    env_v: crate::env_var_result_var_error::EnvVarResultVarError,
    env_var_name: crate::parse_env_var_name_ref::ParseEnvVarNameRef<'static>,
    parse_ctx: crate::parse_ctx_ref::ParseCtxRef,
) -> Result<T, crate::env_parse_error::EnvParseError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    crate::parse_from_env_var_with_tests::parse_from_env_var_with(env_v, env_var_name, |v| {
        crate::parse_from_str_with_ctx_tests::parse_from_str_with_ctx(v, parse_ctx)
    })
}
