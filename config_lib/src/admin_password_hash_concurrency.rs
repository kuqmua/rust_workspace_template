#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct AdminPasswordHashConcurrency(std::num::NonZeroUsize);

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for AdminPasswordHashConcurrency {
    type Error = crate::try_from_std_env_var_ok_admin_password_hash_concurrency_error::TryFromStdEnvVarOkAdminPasswordHashConcurrencyError;

    fn try_from_std_env_var_ok(
        std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    ) -> Result<Self, Self::Error> {
        let parsed = std_env_var_ok
            .parse::<usize>()
            .map_err(|admin_positive_usize_parsing| Self::Error::Parse {
                admin_positive_usize_parsing:
                    crate::config_parse_int_error::ConfigParseIntError::from(
                        admin_positive_usize_parsing,
                    ),
            })?;
        std::num::NonZeroUsize::new(parsed)
            .map(Self)
            .ok_or(Self::Error::IsZero)
    }
}
