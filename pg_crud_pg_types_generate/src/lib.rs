#[proc_macro]
pub fn generate_pg_types(input_token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_tokens = input_token_stream.into();
    generate_pg_types_src::domain_types::source::generate_pg_types(
        macro_helpers::domain_types::ts_writer::ProcMacro2TokenStreamRef::from(&input_tokens),
    )
    .to_string()
    .parse::<proc_macro::TokenStream>()
    .expect("122809ba generate_pg_types invariant must hold")
}
