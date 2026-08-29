#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct SnakeIdentifier(String);
impl TryFrom<String> for SnakeIdentifier {
    type Error =
        crate::snake_identifierifier_try_from_string_error::SnakeIdentifierifierTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::snake_ident_max_len::SNAKE_IDENT_MAX_LEN {
            return Err(crate::snake_identifierifier_try_from_string_error::SnakeIdentifierifierTryFromStringError(
                crate::snake_identifierifier_len::SnakeIdentifierifierLen::from(value.len()),
            ));
        }
        Ok(Self(value))
    }
}
impl AsRef<str> for SnakeIdentifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Display for SnakeIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl quote::ToTokens for SnakeIdentifier {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
