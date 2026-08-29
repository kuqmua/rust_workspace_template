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
pub struct AdminRefreshTokenTtlSeconds(std::num::NonZeroU64);

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for AdminRefreshTokenTtlSeconds {
    type Error = crate::try_from_std_env_var_ok_admin_positive_u64_error::TryFromStdEnvVarOkAdminPositiveU64Error;

    fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
        super::parse_admin_positive_u64::parse_admin_positive_u64(&v).map(Self)
    }
}
