#[proc_macro_derive(ToErrStringAsRefStr)]
pub fn to_err_string_as_ref_str(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_newtype_shared::to_err_string_as_ref_str(token_stream.into()).into()
}
