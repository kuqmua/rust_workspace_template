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
pub struct HttpGzipEnabled(bool);
impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for HttpGzipEnabled {
    type Error = crate::parse_bool_error::ParseBoolError;
    fn try_from_std_env_var_ok(v: crate::std_env_var_ok::StdEnvVarOk) -> Result<Self, Self::Error> {
        v.parse::<bool>()
            .map(Self)
            .map_err(crate::parse_bool_error::ParseBoolError::from)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn boolean_flags_share_strict_boolean_parsing() {
        let enabled =
            <super::HttpGzipEnabled as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::catalog::TRUE)).expect(
                    "ea35fb71 boolean_flags_share_strict_boolean_parsing invariant must hold",
                ),
            )
            .expect("864d1f90 boolean_flags_share_strict_boolean_parsing invariant must hold");
        assert!(enabled.0);
        let invalid =
            <crate::production_mode::ProductionMode as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::catalog::VALUE_1)).expect(
                    "ab9ec621 boolean_flags_share_strict_boolean_parsing invariant must hold",
                ),
            );
        assert!(matches!(invalid, Err(_error)));
    }
}
