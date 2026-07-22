#![allow(
    clippy::single_call_fn,
    reason = "the bind emitter boundary is intentionally isolated from descriptor and contract emitters"
)]
#[derive(Clone, Copy)]
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
        Self::from(1usize)
    }
}
pub(super) fn bind_count_matches(
    spec: crate::model::FilterSpec,
    placeholders: FilterPlaceholderCount,
) -> crate::model::FilterSpecValid {
    spec.bind_count_matches(placeholders)
}
pub(super) fn text_search_token_stream(
    spec: crate::model::FilterSpec,
) -> macros_helpers::proc_macro2_tokens::ProcMacro2GeneratedRustTokenStream {
    if !bind_count_matches(spec, FilterPlaceholderCount::one()).get() {
        return quote::quote! {compile_error!("text search bind count must match one placeholder");}.into();
    }
    let sql_operator = crate::sql::filter_sql_operator(spec);
    let sql_suffix = crate::sql::filter_sql_suffix(spec);
    quote::quote! {
        impl<'query_lt> pg_crud_common::PgTypeWhereFilter<'query_lt> for PgTypeWhereTextSearch {
            fn query_bind(self, mut query: pg_crud_common::SqlxPostgresQuery<'query_lt>) -> Result<pg_crud_common::SqlxPostgresQuery<'query_lt>, pg_crud_common::SqlxPostgresQueryBindError> {
                let pattern = self.pattern().map_err(|error| match pg_crud_common::SqlxPostgresQueryBindError::try_from(error.to_string()) {
                    Ok(value) => value,
                    Err(conversion_error) => pg_crud_common::SqlxPostgresQueryBindError::from(conversion_error),
                })?;
                if let Err(error) = query.as_mut().try_bind(String::from(pattern)) {
                    return Err(match pg_crud_common::SqlxPostgresQueryBindError::try_from(error.to_string()) {
                        Ok(value) => value,
                        Err(conversion_error) => pg_crud_common::SqlxPostgresQueryBindError::from(conversion_error),
                    });
                }
                Ok(query)
            }
            fn query_part(&self, increment: &mut dyn pg_crud_common::QueryPartIncrementMut, column: pg_crud_common::SqlColumnRef<'_>, add_operator: pg_crud_common::AddOperator) -> Result<pg_crud_common::QueryPartFragment, pg_crud_common::QueryPartError> {
                let parameter = increment.checked_add_one().ok_or_else(|| pg_crud_common::QueryPartError::CheckedAdd { location: location_macros::location!() })?;
                let fragment = format!("{}{} {} ${parameter} {}", self.operator.to_query_part(add_operator), column, #sql_operator, #sql_suffix);
                pg_crud_common::QueryPartFragment::try_from(fragment).map_err(pg_crud_common::QueryPartError::from)
            }
        }
    }
    .into()
}
