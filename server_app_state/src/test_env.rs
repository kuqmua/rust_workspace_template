#[cfg(feature = "test-utils")]
pub(super) fn test_env<T>(std_env_var_ok: config_lib::std_env_var_ok::StdEnvVarOk) -> T
where
    T: config_lib::try_from_std_env_var_ok::TryFromStdEnvVarOk,
    T::Error: std::fmt::Debug,
{
    T::try_from_std_env_var_ok(std_env_var_ok).expect(constants_str::DIAGNOSTIC_3F1C7BB7)
}
