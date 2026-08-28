use crate::domain_types::{
    SNAKE_IDENT_MAX_LEN, SnakeIdentifierifierLen, SnakeIdentifierifierTryFromStringError,
};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct SnakeIdentifier(String);
impl TryFrom<String> for SnakeIdentifier {
    type Error = SnakeIdentifierifierTryFromStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > SNAKE_IDENT_MAX_LEN {
            return Err(SnakeIdentifierifierTryFromStringError(
                SnakeIdentifierifierLen::from(value.len()),
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
