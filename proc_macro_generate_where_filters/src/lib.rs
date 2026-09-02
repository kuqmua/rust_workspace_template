#[proc_macro]
pub fn generate_where_filters(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate_where_filters_src::generate_where_filters_source::generate_where_filters_source(
        generate_where_filters_src::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput::from(
            &input_token_stream.into(),
        ),
    )
    .to_string()
    .parse::<proc_macro::TokenStream>()
    .expect(constants_str::DIAGNOSTIC_6716175C)
}
