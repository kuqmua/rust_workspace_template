#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Eq, PartialEq, newtype::DisplayConst,
)]
#[display_const(constants_str::catalog::REDACTED_ALT_3)]
pub struct BoundedSecretText(pub(super) String);

impl TryFrom<String> for BoundedSecretText {
    type Error = crate::bounded_secret_text_error::BoundedSecretTextError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let _validated = crate::secret_text_ref::SecretTextRef::try_from(value.as_str())?;
        Ok(Self(value))
    }
}

impl std::fmt::Debug for BoundedSecretText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::catalog::REDACTED_ALT_3)
    }
}
