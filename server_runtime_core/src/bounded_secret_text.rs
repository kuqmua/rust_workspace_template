#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Eq,
    PartialEq,
    proc_macro_newtype_display_const::DisplayConst,
)]
#[display_const(constants_str::REDACTED_ALT_3)]
pub struct BoundedSecretText(String);

impl BoundedSecretText {
    pub(super) const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for BoundedSecretText {
    type Error = crate::bounded_secret_text_error::BoundedSecretTextError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        let _validated = crate::secret_text_ref::SecretTextRef::try_from(string.as_str())?;
        Ok(Self(string))
    }
}

impl std::fmt::Debug for BoundedSecretText {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(constants_str::REDACTED_ALT_3)
    }
}
