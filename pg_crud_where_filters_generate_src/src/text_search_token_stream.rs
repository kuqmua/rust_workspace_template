#[allow(
    clippy::single_call_fn,
    reason = "the bind emitter boundary is intentionally isolated from descriptor and contract emitters"
)]
pub(super) fn text_search_token_stream(
    spec: crate::spec::FilterSpec,
) -> macro_helpers::domain_types::proc_macro2_generated_rust_token_stream::ProcMacro2GeneratedRustTokenStream{
    if !crate::bind_count_matches::bind_count_matches(
        spec,
        crate::filter_placeholder_count::FilterPlaceholderCount::one(),
    )
    .get()
    {
        return quote::quote! {compile_error!("text search bind count must match one placeholder");}.into();
    }
    let sql_operator = crate::filter_sql_operator_value::filter_sql_operator_value(spec);
    let sql_suffix = crate::filter_sql_suffix_value::filter_sql_suffix_value(spec);
    quote::quote! {
        impl<'query_lt> pg_crud_common::domain_types::PgTypeWhereFilter<'query_lt> for PgTypeWhereTextSearch {
            fn query_bind(self, mut query: pg_crud_common::domain_types::SqlxPostgresQuery<'query_lt>) -> Result<pg_crud_common::domain_types::SqlxPostgresQuery<'query_lt>, pg_crud_common::domain_types::SqlxPostgresQueryBindError> {
                let pattern = self.pattern().map_err(pg_crud_common::domain_types::make_query_bind_error)?;
                if let Err(error) = query.as_mut().try_bind(String::from(pattern)) {
                    return Err(pg_crud_common::domain_types::SqlxPostgresQueryBindError::from(error));
                }
                Ok(query)
            }
            fn query_part(&self, increment: &mut dyn pg_crud_common::domain_types::QueryPartIncrementMut, column: pg_crud_common::domain_types::SqlColumnRef<'_>, add_operator: pg_crud_common::domain_types::AddOperator) -> Result<pg_crud_common::domain_types::QueryPartFragment, pg_crud_common::domain_types::QueryPartError> {
                let parameter = increment.checked_add_one().ok_or_else(|| pg_crud_common::domain_types::QueryPartError::CheckedAdd { location: location_macros::location!() })?;
                let fragment = format!("{}{} {} ${parameter} {}", self.operator.to_query_part(add_operator), column, #sql_operator, #sql_suffix);
                pg_crud_common::domain_types::QueryPartFragment::try_from(fragment).map_err(pg_crud_common::domain_types::QueryPartError::from)
            }
        }
    }
    .into()
}
