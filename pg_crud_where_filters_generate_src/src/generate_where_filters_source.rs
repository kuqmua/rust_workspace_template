#[must_use]
pub fn generate_where_filters_source(
    proc_macro2_generate_where_filters_input: crate::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput<'_>,
) -> crate::proc_macro2_generate_where_filters_token_stream::ProcMacro2GenerateWhereFiltersTokenStream
{
    match crate::parse_generate_where_filters::parse_generate_where_filters(
        proc_macro2_generate_where_filters_input,
    )
    .and_then(crate::build_generate_where_filters::build_generate_where_filters)
    .and_then(crate::validate_generate_where_filters::validate_generate_where_filters)
    {
        Ok(validated) => crate::emit_generate_where_filters::emit_generate_where_filters(validated),
        Err(error) => {
            let message = error.to_string();
            crate::proc_macro2_generate_where_filters_token_stream::ProcMacro2GenerateWhereFiltersTokenStream::from(
                quote::quote! { compile_error!(#message); },
            )
        }
    }
}
