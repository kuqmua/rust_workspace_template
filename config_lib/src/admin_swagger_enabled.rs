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
pub struct AdminSwaggerEnabled(bool);
impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for AdminSwaggerEnabled {
    type Error = crate::parse_bool_error::ParseBoolError;
    fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
        v.0.parse::<bool>()
            .map(Self)
            .map_err(crate::parse_bool_error::ParseBoolError::from)
    }
}
