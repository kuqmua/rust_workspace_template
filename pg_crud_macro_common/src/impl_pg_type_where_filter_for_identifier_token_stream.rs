#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]

pub fn impl_pg_type_where_filter_for_identifier_token_stream(
    impl_generic_token_stream: &dyn quote::ToTokens,
    identifier_token_stream: &dyn quote::ToTokens,
    identifier_generic_token_stream: &dyn quote::ToTokens,
    increment_parameter_undrscr: &crate::emission_types::IncrementParameterUndrscr,
    column_parameter_undrscr: &crate::emission_types::ColumnParameterUndrscr,
    add_operator_undrscr: &crate::emission_types::AddOperatorUndrscr,
    query_part_token_stream: &dyn quote::ToTokens,
    is_query_bind_mut: &crate::emission_types::IsQueryBindMut,
    query_bind_token_stream: &dyn quote::ToTokens,
    import: &crate::import::Import,
) -> macro_helpers::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream {
    let names = crate::names_context::NamesContext::new();

    #[allow(non_snake_case, reason = "lint suppression is required here")]
    let (
        AllowClippyArbitrarySrcItemOrdering,
        PgTypeWhereFilterUpperCamelCase,
        QueryBindSnakeCase,
        QueryPartErrorUpperCamelCase,
        QueryPartSnakeCase,
    ) = (
        names.get_allow_clippy_arbitrary_src_item_ordering(),
        names.get_pg_type_where_filter_upper_camel_case(),
        names.get_query_bind_snake_case(),
        names.get_query_part_error_upper_camel_case(),
        names.get_query_part_snake_case(),
    );
    quote::quote! {
        #AllowClippyArbitrarySrcItemOrdering
        impl #impl_generic_token_stream #import::pg_type_where_filter::#PgTypeWhereFilterUpperCamelCase<'lt> for #identifier_token_stream #identifier_generic_token_stream {
            fn #QueryPartSnakeCase(
                &self,
                #increment_parameter_undrscr: &mut dyn #import::query_part_increment_mut::QueryPartIncrementMut,
                #column_parameter_undrscr: #import::sql_column_ref::SqlColumnRef<'_>,
                #add_operator_undrscr: #import::add_operator::AddOperator
            ) -> Result<#import::query_part_fragment::QueryPartFragment, #import::query_part_error::#QueryPartErrorUpperCamelCase> {
                #query_part_token_stream
            }
            fn #QueryBindSnakeCase(self, #is_query_bind_mut query: #import::sqlx_postgres_query::SqlxPostgresQuery<'lt>) -> Result<
                #import::sqlx_postgres_query::SqlxPostgresQuery<'lt>,
                #import::sqlx_postgres_query_bind_error::SqlxPostgresQueryBindError
            > {
                #query_bind_token_stream
            }
        }
    }
    .into()
}
