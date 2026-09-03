#[proc_macro]
pub fn generate_upper_camel_case_and_snake_case_str_and_token_stream(
    token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_naming_shared::generate_upper_camel_case_and_snake_case_str_and_token_stream(
        token_stream.into(),
    )
    .into()
}
