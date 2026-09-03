#[proc_macro_derive(DebugRedacted)]
pub fn debug_redacted(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::debug_redacted(token_stream.into()).into()
}
