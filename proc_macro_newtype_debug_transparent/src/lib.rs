#[proc_macro_derive(DebugTransparent)]
pub fn debug_transparent(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::debug_transparent(token_stream.into()).into()
}
