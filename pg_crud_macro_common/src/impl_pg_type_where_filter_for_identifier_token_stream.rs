#![allow(
    clippy::wildcard_imports,
    reason = "split owner modules import the private facade vocabulary used by the moved generator"
)]
use super::super::*;

pub fn impl_pg_type_where_filter_for_identifier_token_stream(
    impl_generic_token_stream: &dyn quote::ToTokens,
    identifier_token_stream: &dyn quote::ToTokens,
    identifier_generic_token_stream: &dyn quote::ToTokens,
    increment_parameter_undrscr: &IncrementParameterUndrscr,
    column_parameter_undrscr: &ColumnParameterUndrscr,
    add_operator_undrscr: &AddOperatorUndrscr,
    query_part_token_stream: &dyn quote::ToTokens,
    is_query_bind_mut: &IsQueryBindMut,
    query_bind_token_stream: &dyn quote::ToTokens,
    import: &Import,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    let names = NamesCtx::new();
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(non_snake_case)]
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
        impl #impl_generic_token_stream #import ::#PgTypeWhereFilterUpperCamelCase<'lt> for #identifier_token_stream #identifier_generic_token_stream {
            fn #QueryPartSnakeCase(
                &self,
                #increment_parameter_undrscr: &mut dyn #import::QueryPartIncrementMut,
                #column_parameter_undrscr: #import::SqlColumnRef<'_>,
                #add_operator_undrscr: #import::AddOperator
            ) -> Result<#import::QueryPartFragment, #import::#QueryPartErrorUpperCamelCase> {
                #query_part_token_stream
            }
            fn #QueryBindSnakeCase(self, #is_query_bind_mut query: #import::SqlxPostgresQuery<'lt>) -> Result<
                #import::SqlxPostgresQuery<'lt>,
                #import::SqlxPostgresQueryBindError
            > {
                #query_bind_token_stream
            }
        }
    }
    .into()
}
