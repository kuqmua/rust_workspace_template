#[must_use]
pub fn generate_where_filters_source(
    input_token_stream: crate::source::ProcMacro2GenerateWhereFiltersInput<'_>,
) -> crate::source::ProcMacro2GenerateWhereFiltersTokenStream {
    match crate::source::parse_generate_where_filters(input_token_stream)
        .and_then(crate::source::build_generate_where_filters)
        .and_then(crate::source::validate_generate_where_filters)
    {
        Ok(validated) => crate::source::emit_generate_where_filters(validated),
        Err(error) => {
            let message = error.to_string();
            crate::source::ProcMacro2GenerateWhereFiltersTokenStream::from(
                quote::quote! { compile_error!(#message); },
            )
        }
    }
}
