#[proc_macro_derive(BorrowStr)]
pub fn borrow_str(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::borrow_str(token_stream.into()).into()
}
