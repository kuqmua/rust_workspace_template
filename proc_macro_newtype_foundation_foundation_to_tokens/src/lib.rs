#[proc_macro_derive(ToTokens)]
pub fn foundation_to_tokens(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_foundation_shared::foundation_to_tokens(token_stream.into()).into()
}
