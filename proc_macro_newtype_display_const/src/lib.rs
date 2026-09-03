#[proc_macro_derive(DisplayConst, attributes(display_const))]
pub fn display_const(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::display_const(token_stream.into()).into()
}
