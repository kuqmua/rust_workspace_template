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
pub struct ProductionMode(bool);
impl crate::TryFromStdEnvVarOk for ProductionMode {
    type Error = crate::ParseBoolError;
    fn try_from_std_env_var_ok(v: crate::StdEnvVarOk) -> Result<Self, Self::Error> {
        v.0.parse::<bool>()
            .map(Self)
            .map_err(crate::ParseBoolError::from)
    }
}
