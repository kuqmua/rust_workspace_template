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
    type Error = super::try_from_std_env_var_ok_admin_cookie_secure_error::TryFromStdEnvVarOkAdminCookieSecureError;
    fn try_from_std_env_var_ok(v: crate::StdEnvVarOk) -> Result<Self, Self::Error> {
        v.0.parse::<bool>().map(Self).map_err(|admin_bool_parsing| {
            super::try_from_std_env_var_ok_admin_cookie_secure_error::TryFromStdEnvVarOkAdminCookieSecureError::from(super::admin_bool_parsing_error::AdminBoolParsingError::from(
                crate::ParseBoolError::from(admin_bool_parsing),
            ))
        })
    }
}
