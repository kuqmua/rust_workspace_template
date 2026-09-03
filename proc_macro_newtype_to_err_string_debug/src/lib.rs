#[proc_macro_derive(ToErrStringDebug)]
pub fn to_err_string_debug(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::to_err_string_debug(token_stream.into()).into()
}
