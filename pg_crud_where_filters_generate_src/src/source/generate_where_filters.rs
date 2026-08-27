#[must_use]
pub fn generate_where_filters(
    input_token_stream: super::ProcMacro2GenerateWhereFiltersInput<'_>,
) -> super::ProcMacro2GenerateWhereFiltersTokenStream {
    match super::parse_generate_where_filters(input_token_stream)
        .and_then(super::build_generate_where_filters)
        .and_then(super::validate_generate_where_filters)
    {
        Ok(validated) => super::emit_generate_where_filters(validated),
        Err(error) => {
            let message = error.to_string();
            super::ProcMacro2GenerateWhereFiltersTokenStream::from(
                quote::quote! { compile_error!(#message); },
            )
        }
    }
}
