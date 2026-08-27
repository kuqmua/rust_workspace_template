#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::AdminSsrTextTryFromStringError;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::Display,
    newtype::IntoInnerFrom,
)]
pub struct AdminSsrText(pub(super) String);
impl TryFrom<String> for AdminSsrText {
    type Error = AdminSsrTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        (value.len() <= constants_usize::VALUE_16_777_216)
            .then_some(Self(value))
            .ok_or(AdminSsrTextTryFromStringError)
    }
}
impl From<AdminSsrTextTryFromStringError> for AdminSsrText {
    fn from(value: AdminSsrTextTryFromStringError) -> Self {
        Self(value.to_string())
    }
}
