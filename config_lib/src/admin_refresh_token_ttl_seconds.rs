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

impl crate::TryFromStdEnvVarOk for AdminRefreshTokenTtlSeconds {
    type Error = super::TryFromStdEnvVarOkAdminPositiveU64Error;

    fn try_from_std_env_var_ok(v: crate::StdEnvVarOk) -> Result<Self, Self::Error> {
        super::parse_admin_positive_u64::parse_admin_positive_u64(&v).map(Self)
    }
}
