#[proc_macro_derive(ToTokens)]
pub fn to_tokens(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::to_tokens(token_stream.into()).into()
}
