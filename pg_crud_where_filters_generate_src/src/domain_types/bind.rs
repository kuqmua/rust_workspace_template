#![allow(
    clippy::single_call_fn,
    reason = "the bind emitter boundary is intentionally isolated from descriptor and contract emitters"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the sibling descriptor validates bind count without exposing a primitive boundary"
)]
#[derive(newtype::FromInner)]
pub(super) struct FilterPlaceholderCount(usize);
impl FilterPlaceholderCount {
    pub(super) const fn get(self) -> usize {
        self.0
    }
    pub(super) fn one() -> Self {
        Self::from(constants_usize::ONE)
    }
}
pub(super) fn bind_count_matches(
    spec: crate::domain_types::spec::FilterSpec,
    placeholders: FilterPlaceholderCount,
) -> crate::domain_types::spec::FilterSpecValid {
    spec.bind_count_matches(placeholders)
}
pub(super) fn text_search_token_stream(
    spec: crate::domain_types::spec::FilterSpec,
) -> macro_helpers::domain_types::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    if !bind_count_matches(spec, FilterPlaceholderCount::one()).get() {
        return quote::quote! {compile_error!("text search bind count must match one placeholder");}.into();
    }
    let sql_operator = crate::domain_types::sql::filter_sql_operator(spec);
    let sql_suffix = crate::domain_types::sql::filter_sql_suffix(spec);
    quote::quote! {
        impl<'query_lt> pg_crud_common::domain_types::PgTypeWhereFilter<'query_lt> for PgTypeWhereTextSearch {
            fn query_bind(self, mut query: pg_crud_common::domain_types::SqlxPostgresQuery<'query_lt>) -> Result<pg_crud_common::domain_types::SqlxPostgresQuery<'query_lt>, pg_crud_common::domain_types::SqlxPostgresQueryBindError> {
                let pattern = self.pattern().map_err(pg_crud_common::domain_types::mk_query_bind_err)?;
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
