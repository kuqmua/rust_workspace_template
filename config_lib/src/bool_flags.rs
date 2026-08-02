#[derive(
    optml::Optml,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct AdminCookieSecure(bool);
#[derive(
    optml::Optml,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    generate_getter_traits_for_struct_fields::GenerateGetterTrait,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct AdminSwaggerEnabled(bool);
#[derive(
    optml::Optml, Debug, Clone, Copy, PartialEq, Eq, newtype::DerefInner, newtype::FromInner,
)]
pub struct HttpGzipEnabled(bool);
#[derive(
    optml::Optml, Debug, Clone, Copy, PartialEq, Eq, newtype::DerefInner, newtype::FromInner,
)]
pub struct ProductionMode(bool);
#[derive(optml::Optml, newtype::DebugTransparent, newtype::FromInner)]
pub struct AdminBoolParsingError(super::StdParseBoolError);
#[derive(optml::Optml, Debug, thiserror::Error)]
#[error("{0:?}")]
#[derive(newtype::FromInner)]
pub struct TryFromStdEnvVarOkAdminCookieSecureError(AdminBoolParsingError);
impl super::TryFromStdEnvVarOk for AdminCookieSecure {
    type Error = TryFromStdEnvVarOkAdminCookieSecureError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        v.0.parse::<bool>().map(Self).map_err(|admin_bool_parsing| {
            TryFromStdEnvVarOkAdminCookieSecureError::from(AdminBoolParsingError::from(
                super::StdParseBoolError::from(admin_bool_parsing),
            ))
        })
    }
}
impl super::TryFromStdEnvVarOk for AdminSwaggerEnabled {
    type Error = TryFromStdEnvVarOkAdminCookieSecureError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        v.0.parse::<bool>().map(Self).map_err(|admin_bool_parsing| {
            TryFromStdEnvVarOkAdminCookieSecureError::from(AdminBoolParsingError::from(
                super::StdParseBoolError::from(admin_bool_parsing),
            ))
        })
    }
}
impl super::TryFromStdEnvVarOk for HttpGzipEnabled {
    type Error = TryFromStdEnvVarOkAdminCookieSecureError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        v.0.parse::<bool>().map(Self).map_err(|admin_bool_parsing| {
            TryFromStdEnvVarOkAdminCookieSecureError::from(AdminBoolParsingError::from(
                super::StdParseBoolError::from(admin_bool_parsing),
            ))
        })
    }
}
impl super::TryFromStdEnvVarOk for ProductionMode {
    type Error = TryFromStdEnvVarOkAdminCookieSecureError;
    fn try_from_std_env_var_ok(v: super::StdEnvVarOk) -> Result<Self, Self::Error> {
        v.0.parse::<bool>().map(Self).map_err(|admin_bool_parsing| {
            TryFromStdEnvVarOkAdminCookieSecureError::from(AdminBoolParsingError::from(
                super::StdParseBoolError::from(admin_bool_parsing),
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn boolean_flags_share_strict_boolean_parsing() {
        let enabled =
            <super::HttpGzipEnabled as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                super::super::StdEnvVarOk::try_from(String::from("true")).expect("ea35fb71"),
            )
            .expect("864d1f90");
        assert!(enabled.0);
        let invalid =
            <super::ProductionMode as super::super::TryFromStdEnvVarOk>::try_from_std_env_var_ok(
                super::super::StdEnvVarOk::try_from(String::from("1")).expect("ab9ec621"),
            );
        assert!(matches!(
            invalid,
            Err(super::TryFromStdEnvVarOkAdminCookieSecureError(_))
        ));
    }
}
