#[proc_macro_derive(GetInner, attributes(accessor, borrow))]
pub fn get_inner(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::get_inner(token_stream.into()).into()
}
