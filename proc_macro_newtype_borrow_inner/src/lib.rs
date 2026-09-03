#[proc_macro_derive(BorrowInner)]
pub fn borrow_inner(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::borrow_inner(token_stream.into()).into()
}
