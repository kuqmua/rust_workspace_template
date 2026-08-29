#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub(crate) struct RuntimeTestUrl(pub(super) String);

impl TryFrom<String> for RuntimeTestUrl {
    type Error = crate::service_base_url_error::ServiceBaseUrlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_8_192 {
            Err(crate::service_base_url_error::ServiceBaseUrlError::Length)
        } else {
            Ok(Self(value))
        }
    }
}
