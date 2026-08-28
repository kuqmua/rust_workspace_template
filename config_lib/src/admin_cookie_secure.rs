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
pub struct AdminCookieSecure(bool);
impl crate::TryFromStdEnvVarOk for AdminCookieSecure {
    type Error = crate::ParseBoolError;
    fn try_from_std_env_var_ok(v: crate::StdEnvVarOk) -> Result<Self, Self::Error> {
        v.0.parse::<bool>()
            .map(Self)
            .map_err(crate::ParseBoolError::from)
    }
}
