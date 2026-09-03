#[proc_macro_derive(BorrowPath)]
pub fn borrow_path(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::borrow_path(token_stream.into()).into()
}
