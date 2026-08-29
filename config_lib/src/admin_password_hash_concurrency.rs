#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct AdminPasswordHashConcurrency(std::num::NonZeroUsize);

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for AdminPasswordHashConcurrency {
    type Error = crate::try_from_std_env_var_ok_admin_password_hash_concurrency_error::TryFromStdEnvVarOkAdminPasswordHashConcurrencyError;

    fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
        let parsed =
            v.0.parse::<usize>()
                .map_err(|admin_positive_usize_parsing| Self::Error::Parse {
                    admin_positive_usize_parsing: crate::parse_int_error::ParseIntError::from(
                        admin_positive_usize_parsing,
                    ),
                })?;
        std::num::NonZeroUsize::new(parsed)
            .map(Self)
            .ok_or(Self::Error::IsZero)
    }
}
