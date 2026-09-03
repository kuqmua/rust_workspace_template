#[proc_macro_derive(DebugDisplay)]
pub fn debug_display(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::debug_display(token_stream.into()).into()
}
