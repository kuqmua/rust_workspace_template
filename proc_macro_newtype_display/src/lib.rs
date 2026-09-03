#[proc_macro_derive(Display)]
pub fn display(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::display(token_stream.into()).into()
}
