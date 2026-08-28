use crate::domain_types::{AdminResourceText, UuidAdminValue};

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
impl From<AdminResourceText> for StdAdminString {
    fn from(resource: AdminResourceText) -> Self {
        Self(match resource {
            AdminResourceText::PositiveI64(value) => value.get().to_string(),
            AdminResourceText::SystemSettings => constants_str::VALUE_1.to_owned(),
            AdminResourceText::Uuid(value) => value.get().to_string(),
        })
    }
}
impl StdAdminString {
    #[must_use]
    pub fn from_positive_i64(
        value: server_admin_contract::domain_types::PositiveNonZeroI64,
    ) -> Self {
        Self::from(AdminResourceText::PositiveI64(value))
    }

    #[must_use]
    pub fn from_uuid(value: UuidAdminValue) -> Self {
        Self::from(AdminResourceText::Uuid(value))
    }

    #[must_use]
    pub fn system_settings_resource() -> Self {
        Self::from(AdminResourceText::SystemSettings)
    }
}
