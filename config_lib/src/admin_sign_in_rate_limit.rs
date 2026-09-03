#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_generate_accessor_traits_for_struct_fields_generate_accessor_trait::GenerateAccessorTrait,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct AdminSignInRateLimit(std::num::NonZeroU64);

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for AdminSignInRateLimit {
    type Error = crate::try_from_std_env_var_ok_admin_positive_u64_error::TryFromStdEnvVarOkAdminPositiveU64Error;

    fn try_from_std_env_var_ok(
        std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    ) -> Result<Self, Self::Error> {
        super::parse_admin_positive_u64::parse_admin_positive_u64(&std_env_var_ok).map(Self)
    }
}
