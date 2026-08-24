#[proc_macro]
pub fn generate_where_filters(
    input_token_stream: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate_where_filters_src::generate_where_filters(
        generate_where_filters_src::ProcMacro2GenerateWhereFiltersInput::from(
            &input_token_stream.into(),
        ),
    )
    .to_string()
    .parse::<proc_macro::TokenStream>()
    .expect("6716175c generate_where_filters invariant must hold")
}
