#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

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
    type Error = crate::admin_ssr_text_try_from_string_error::AdminSsrTextTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        (value.len() <= constants_usize::VALUE_16_777_216)
            .then_some(Self(value))
            .ok_or(crate::admin_ssr_text_try_from_string_error::AdminSsrTextTryFromStringError)
    }
}
impl From<crate::admin_ssr_text_try_from_string_error::AdminSsrTextTryFromStringError>
    for AdminSsrText
{
    fn from(
        value: crate::admin_ssr_text_try_from_string_error::AdminSsrTextTryFromStringError,
    ) -> Self {
        Self(value.to_string())
    }
}
