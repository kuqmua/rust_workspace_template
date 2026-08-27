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
pub struct AdminSessionLimit(super::super::ConfigNonZeroUsize);

impl super::super::TryFromStdEnvVarOk for AdminSessionLimit {
    type Error = super::TryFromStdEnvVarOkAdminPositiveU64Error;

    fn try_from_std_env_var_ok(v: super::super::StdEnvVarOk) -> Result<Self, Self::Error> {
        let value = super::parse_admin_positive_u64::parse_admin_positive_u64(&v)?;
        usize::try_from(value.0.get())
            .ok()
            .and_then(std::num::NonZeroUsize::new)
            .map(super::super::ConfigNonZeroUsize::from)
            .map(Self)
            .ok_or(super::TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)
    }
}
