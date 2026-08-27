#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct AdminAccessTokenTtlSeconds(pub(super) super::super::ConfigNonZeroU64);

impl super::super::TryFromStdEnvVarOk for AdminAccessTokenTtlSeconds {
    type Error = super::TryFromStdEnvVarOkAdminPositiveU64Error;

    fn try_from_std_env_var_ok(v: super::super::StdEnvVarOk) -> Result<Self, Self::Error> {
        super::parse_admin_positive_u64::parse_admin_positive_u64(&v).map(Self)
    }
}
