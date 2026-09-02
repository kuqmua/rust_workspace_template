#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_generate_accessor_traits_for_struct_fields::GenerateAccessorTrait,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
pub struct AdminSessionLimit(std::num::NonZeroUsize);

impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for AdminSessionLimit {
    type Error = crate::try_from_std_env_var_ok_admin_positive_u64_error::TryFromStdEnvVarOkAdminPositiveU64Error;

    fn try_from_std_env_var_ok(
        std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    ) -> Result<Self, Self::Error> {
        let value = super::parse_admin_positive_u64::parse_admin_positive_u64(&std_env_var_ok)?;
        usize::try_from(value.get())
            .ok()
            .and_then(std::num::NonZeroUsize::new)
            .map(Self)
            .ok_or(crate::try_from_std_env_var_ok_admin_positive_u64_error::TryFromStdEnvVarOkAdminPositiveU64Error::IsZero)
    }
}
