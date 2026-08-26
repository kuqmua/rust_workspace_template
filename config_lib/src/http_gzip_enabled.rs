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
impl super::super::TryFromStdEnvVarOk for HttpGzipEnabled {
    type Error = super::try_from_std_env_var_ok_admin_cookie_secure_error::TryFromStdEnvVarOkAdminCookieSecureError;
    fn try_from_std_env_var_ok(v: super::super::StdEnvVarOk) -> Result<Self, Self::Error> {
        v.0.parse::<bool>().map(Self).map_err(|admin_bool_parsing| {
            super::try_from_std_env_var_ok_admin_cookie_secure_error::TryFromStdEnvVarOkAdminCookieSecureError::from(super::admin_bool_parsing_error::AdminBoolParsingError::from(
                super::super::ParseBoolError::from(admin_bool_parsing),
            ))
        })
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn boolean_flags_share_strict_boolean_parsing() {
        let enabled =
            <super::HttpGzipEnabled as super::super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                super::super::super::StdEnvVarOk::try_from(String::from(constants_str::TRUE)).expect(
                    "ea35fb71 boolean_flags_share_strict_boolean_parsing invariant must hold",
                ),
            )
            .expect("864d1f90 boolean_flags_share_strict_boolean_parsing invariant must hold");
        assert!(enabled.0);
        let invalid =
            <super::super::production_mode::ProductionMode as super::super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                super::super::super::StdEnvVarOk::try_from(String::from(constants_str::VALUE_1)).expect(
                    "ab9ec621 boolean_flags_share_strict_boolean_parsing invariant must hold",
                ),
            );
        assert!(matches!(invalid, Err(_error)));
    }
}
