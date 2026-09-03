#[proc_macro]
pub fn assert_parse_err_matches(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_config_lib_shared::assert_parse_err_matches(token_stream.into()).into()
}
