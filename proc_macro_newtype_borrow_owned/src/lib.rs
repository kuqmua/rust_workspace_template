#[proc_macro_derive(BorrowOwned)]
pub fn borrow_owned(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::borrow_owned(token_stream.into()).into()
}
