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
pub struct AdminCookieSecure(bool);
impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for AdminCookieSecure {
    type Error = crate::parse_bool_error::ParseBoolError;
    fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
        v.parse::<bool>()
            .map(Self)
            .map_err(crate::parse_bool_error::ParseBoolError::from)
    }
}
