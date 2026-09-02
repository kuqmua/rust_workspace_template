#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
pub struct HttpGzipEnabled(bool);
impl crate::try_from_std_env_var_ok::TryFromStdEnvVarOk for HttpGzipEnabled {
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
#[cfg(test)]
mod tests {
    #[test]
    fn test_boolean_flags_share_strict_boolean_parsing() {
        let enabled =
            <super::HttpGzipEnabled as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::TRUE)).expect(constants_str::DIAGNOSTIC_EA35FB71),
            )
            .expect(constants_str::DIAGNOSTIC_864D1F90);
        assert!(enabled.0);
        let invalid =
            <crate::production_mode::ProductionMode as crate::try_from_std_env_var_ok::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                crate::std_env_var_ok::StdEnvVarOk::try_from(String::from(constants_str::VALUE_1)).expect(constants_str::DIAGNOSTIC_AB9EC621),
            );
        assert!(matches!(invalid, Err(_error)));
    }
}
