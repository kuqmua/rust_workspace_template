#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_newtype_bounded_string_wrapper::BoundedStringWrapper,
    proc_macro_newtype_as_ref_owned::AsRefOwned,
    proc_macro_newtype_deref_inner::DerefInner,
    proc_macro_newtype_into_inner::IntoInner,
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
    fn from(value: crate::admin_resource_text::AdminResourceText) -> Self {
        let text = match value {
            crate::admin_resource_text::AdminResourceText::PositiveI64(positive_i64) => {
                positive_i64.get().to_string()
            }
            crate::admin_resource_text::AdminResourceText::SystemSettings => {
                constants_str::VALUE_1.to_owned()
            }
            crate::admin_resource_text::AdminResourceText::Uuid(uuid) => uuid.get().to_string(),
        };
        Self::try_from(text).unwrap_or_else(Self::from)
    }
}
impl StdAdminString {
    #[must_use]
    pub fn from_positive_i64(
        positive_non_zero_i64: server_admin_contract::positive_non_zero_i64::PositiveNonZeroI64,
    ) -> Self {
        Self::from(crate::admin_resource_text::AdminResourceText::PositiveI64(
            positive_non_zero_i64,
        ))
    }

    #[must_use]
    pub fn from_uuid(uuid_admin_value: crate::uuid_admin_value::UuidAdminValue) -> Self {
        Self::from(crate::admin_resource_text::AdminResourceText::Uuid(
            uuid_admin_value,
        ))
    }

    #[must_use]
    pub fn system_settings_resource() -> Self {
        Self::from(crate::admin_resource_text::AdminResourceText::SystemSettings)
    }
}
