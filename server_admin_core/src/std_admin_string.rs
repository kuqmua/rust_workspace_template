#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    utoipa::ToSchema,
    newtype::BoundedStringWrapper,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::IntoInner,
)]
#[bounded_string(max = 8192, description = "administrator internal text")]
pub struct StdAdminString(bounded_types::bounded_string::BoundedString<0usize, 8192, false>);
impl secrecy::zeroize::Zeroize for StdAdminString {
    fn zeroize(&mut self) {
        let mut value = std::mem::take(&mut self.0).into_string();
        secrecy::zeroize::Zeroize::zeroize(&mut value);
    }
}
impl From<crate::admin_resource_text::AdminResourceText> for StdAdminString {
    fn from(resource: crate::admin_resource_text::AdminResourceText) -> Self {
        let value = match resource {
            crate::admin_resource_text::AdminResourceText::PositiveI64(value) => {
                value.get().to_string()
            }
            crate::admin_resource_text::AdminResourceText::SystemSettings => {
                constants_str::VALUE_1.to_owned()
            }
            crate::admin_resource_text::AdminResourceText::Uuid(value) => value.get().to_string(),
        };
        Self::try_from(value).unwrap_or_else(Self::from)
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
