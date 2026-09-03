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
pub struct AdminCookieSecure(bool);
impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for AdminCookieSecure {
    type Error = crate::parse_bool_error::ParseBoolError;
    fn try_from_std_env_var_ok(
        std_env_var_ok: crate::std_env_var_ok::StdEnvVarOk,
    ) -> Result<Self, Self::Error> {
        std_env_var_ok
            .parse::<bool>()
            .map(Self)
            .map_err(crate::parse_bool_error::ParseBoolError::from)
    }
}
