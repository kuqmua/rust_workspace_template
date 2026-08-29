#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::BoundedString,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::IntoInner,
)]
#[bounded_string(max = 8192, description = "administrator internal text")]
pub struct StdAdminString(String);
impl secrecy::zeroize::Zeroize for StdAdminString {
    fn zeroize(&mut self) {
        secrecy::zeroize::Zeroize::zeroize(&mut self.0);
    }
}
impl From<crate::admin_resource_text::AdminResourceText> for StdAdminString {
    fn from(resource: crate::admin_resource_text::AdminResourceText) -> Self {
        Self(match resource {
            crate::admin_resource_text::AdminResourceText::PositiveI64(value) => {
                value.get().to_string()
            }
            crate::admin_resource_text::AdminResourceText::SystemSettings => {
                constants_str::catalog::VALUE_1.to_owned()
            }
            crate::admin_resource_text::AdminResourceText::Uuid(value) => value.get().to_string(),
        })
    }
}
impl StdAdminString {
    #[must_use]
    pub fn from_positive_i64(
        value: server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64,
    ) -> Self {
        Self::from(crate::admin_resource_text::AdminResourceText::PositiveI64(
            value,
        ))
    }

    #[must_use]
    pub fn from_uuid(value: crate::uuid_admin_value::UuidAdminValue) -> Self {
        Self::from(crate::admin_resource_text::AdminResourceText::Uuid(value))
    }

    #[must_use]
    pub fn system_settings_resource() -> Self {
        Self::from(crate::admin_resource_text::AdminResourceText::SystemSettings)
    }
}
