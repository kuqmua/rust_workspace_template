pub(super) fn parse_admin_positive_u64(
    v: &crate::std_env_var_ok::StdEnvVarOk,
) -> Result<std::num::NonZeroU64, crate::try_from_std_env_var_ok_admin_positive_u64_error::TryFromStdEnvVarOkAdminPositiveU64Error>{
    let parsed = v.parse::<u64>().map_err(|admin_positive_u64_parsing| {
        crate::try_from_std_env_var_ok_admin_positive_u64_error::TryFromStdEnvVarOkAdminPositiveU64Error::Parse {
            admin_positive_u64_parsing: crate::config_parse_int_error::ConfigParseIntError::from(admin_positive_u64_parsing),
        }
    })?;
    std::num::NonZeroU64::new(parsed).ok_or(crate::try_from_std_env_var_ok_admin_positive_u64_error::TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)
}
