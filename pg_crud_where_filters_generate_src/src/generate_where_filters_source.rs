#[must_use]
pub fn generate_where_filters_source(
    proc_macro2_generate_where_filters_input: crate::proc_macro2_generate_where_filters_input::ProcMacro2GenerateWhereFiltersInput<'_>,
) -> crate::proc_macro2_generate_where_filters_token_stream::ProcMacro2GenerateWhereFiltersTokenStream
{
    macro_helpers::generate_validated_tokens::generate_validated_tokens(
        proc_macro2_generate_where_filters_input,
        crate::parse_generate_where_filters::parse_generate_where_filters,
        crate::build_generate_where_filters::build_generate_where_filters,
        crate::validate_generate_where_filters::validate_generate_where_filters,
        crate::emit_generate_where_filters::emit_generate_where_filters,
        |error| {
            let message = error.to_string();
            crate::proc_macro2_generate_where_filters_token_stream::ProcMacro2GenerateWhereFiltersTokenStream::from(
                quote::quote! { compile_error!(#message); },
            )
        },
    )
}
