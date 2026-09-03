#[proc_macro]
pub fn impl_to_err_string_as_ref_str(
    token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_to_err_string_shared::impl_to_err_string_as_ref_str(token_stream.into()).into()
}
