#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_debug_redacted::DebugRedacted,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner::IntoInner,
    serde::Deserialize,
)]
#[serde(try_from = "String")]
#[derive(proc_macro_getters::Getters)]
pub struct RuntimeAdminPassword(server_admin_core::secrecy_admin_string::SecrecyAdminString);

impl utoipa::PartialSchema for RuntimeAdminPassword {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::Type::String)
            .min_length(Some(
                server_admin_contract::identity::ADMIN_PASSWORD_MIN_CHARS,
            ))
            .max_length(Some(
                server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS,
            ))
            .write_only(Some(true))
            .build()
            .into()
    }
}
impl utoipa::ToSchema for RuntimeAdminPassword {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(constants_str::ADMINPASSWORD)
    }
}
impl TryFrom<String> for RuntimeAdminPassword {
    type Error = crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let len = value.chars().count();
        if !(server_admin_contract::identity::ADMIN_PASSWORD_MIN_CHARS
            ..=server_admin_contract::identity::ADMIN_PASSWORD_MAX_CHARS)
            .contains(&len)
        {
            return Err(crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError::InvalidLength);
        }
        server_admin_core::secrecy_admin_string::SecrecyAdminString::try_from(value)
            .map(Self::from)
            .map_err(|error| match error {
                server_admin_core::std_admin_string::StdAdminStringTryFromStringError::InvalidBounds { .. }
                | server_admin_core::std_admin_string::StdAdminStringTryFromStringError::TooShort { .. }
                | server_admin_core::std_admin_string::StdAdminStringTryFromStringError::TooLong { .. }
                | server_admin_core::std_admin_string::StdAdminStringTryFromStringError::ContainsNul
                | server_admin_core::std_admin_string::StdAdminStringTryFromStringError::InvalidValue => {
                    crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError::InvalidLength
                }
            })
    }
}
impl RuntimeAdminPassword {
    #[must_use]
    pub fn new(
        secrecy_admin_string: server_admin_core::secrecy_admin_string::SecrecyAdminString,
    ) -> Self {
        Self::from(secrecy_admin_string)
    }
}
