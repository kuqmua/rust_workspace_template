#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_foundation_foundation_to_tokens::ToTokens,
)]
pub(crate) struct SnakeIdentifier(String);
impl TryFrom<String> for SnakeIdentifier {
    type Error =
        crate::snake_identifierifier_try_from_string_error::SnakeIdentifierifierTryFromStringError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        if string.len() > crate::snake_ident_max_len::SNAKE_IDENT_MAX_LEN {
            return Err(
                crate::snake_identifierifier_try_from_string_error::SnakeIdentifierifierTryFromStringError::from(
                    crate::snake_identifierifier_len::SnakeIdentifierifierLen::from(string.len()),
                ),
            );
        }
        Ok(Self(string))
    }
}
impl AsRef<str> for SnakeIdentifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Display for SnakeIdentifier {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}
