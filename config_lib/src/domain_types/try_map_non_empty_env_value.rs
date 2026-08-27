use super::StdEnvVarOk;

pub(super) fn try_map_non_empty_env_value<T, Error>(
    v: StdEnvVarOk,
    mk_error: impl FnOnce(&'static str) -> Error,
    map_ok: impl FnOnce(String) -> T,
) -> Result<T, Error> {
    if v.0.is_empty() {
        return Err(mk_error(constants_str::CONFIG_ENV_VALUE_IS_EMPTY_MSG));
    }
    Ok(map_ok(v.0))
}
