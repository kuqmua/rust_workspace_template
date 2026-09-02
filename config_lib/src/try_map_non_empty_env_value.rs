pub(super) fn try_map_non_empty_env_value<T, Error>(
    std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    make_error: impl FnOnce(&'static str) -> Error,
    map_ok: impl FnOnce(String) -> T,
) -> Result<T, Error> {
    if std_env_var_ok.is_empty() {
        return Err(make_error(constants_str::CONFIG_ENV_VALUE_IS_EMPTY_MSG));
    }
    Ok(map_ok(String::from(std_env_var_ok)))
}
