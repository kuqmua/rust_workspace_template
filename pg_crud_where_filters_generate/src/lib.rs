#[proc_macro]
pub fn generate_where_filters(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate_where_filters_src::domain_types::source::generate_where_filters_source(
        generate_where_filters_src::domain_types::source::ProcMacro2GenerateWhereFiltersInput::from(
            &input_token_stream.into(),
        ),
    )
    .to_string()
    .parse::<proc_macro::TokenStream>()
    .expect("6716175c generate_where_filters invariant must hold")
}
