#[proc_macro]
pub fn generate_pg_types(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_tokens = token_stream.into();
    generate_pg_types_src::generate_pg_types_tokens::generate_pg_types_tokens(
        macro_helpers::proc_macro2_token_stream_ref::ProcMacro2TokenStreamRef::from(&input_tokens),
    )
    .to_string()
    .parse::<proc_macro::TokenStream>()
    .expect(constants_str::DIAGNOSTIC_122809BA)
}
