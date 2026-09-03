#[proc_macro_derive(ToErrString)]
pub fn to_err_string(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::to_err_string(token_stream.into()).into()
}
