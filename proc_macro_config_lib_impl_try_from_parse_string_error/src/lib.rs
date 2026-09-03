#[proc_macro]
pub fn impl_try_from_parse_string_error(
    token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_config_lib_shared::impl_try_from_parse_string_error(token_stream.into()).into()
}
