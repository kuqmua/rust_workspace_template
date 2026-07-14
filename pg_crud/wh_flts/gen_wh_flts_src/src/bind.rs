#![allow(
    clippy::single_call_fn,
    reason = "the bind emitter boundary is intentionally isolated from descriptor and contract emitters"
)]
#[derive(Clone, Copy)]
#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the sibling descriptor validates bind count without exposing a primitive boundary"
)]
pub(super) struct FilterPlaceholderCount(pub(super) usize);
impl FilterPlaceholderCount {
    pub(super) const fn one() -> Self {
        Self(1usize)
    }
}
pub(super) const fn bind_count_matches(
    spec: crate::model::FilterSpec,
    placeholders: FilterPlaceholderCount,
) -> crate::model::FilterSpecValid {
    spec.bind_count_matches(placeholders)
}
pub(super) fn text_search_ts(
    spec: crate::model::FilterSpec,
) -> macros_helpers::generated_rust_ts::GeneratedRustTs {
    if !bind_count_matches(spec, FilterPlaceholderCount::one()).get() {
        return quote::quote! {compile_error!("text search bind count must match one placeholder");}.into();
    }
    let sql_operator = crate::sql::filter_sql_operator(spec);
    let sql_suffix = crate::sql::filter_sql_suffix(spec);
    quote::quote! {
        impl<'query_lt> pg_crud_cmn::PgTypeWhFlt<'query_lt> for PgTypeWhTextSearch {
            fn qb(self, mut query: pg_crud_cmn::SqlxPostgresQuery<'query_lt>) -> Result<pg_crud_cmn::SqlxPostgresQuery<'query_lt>, pg_crud_cmn::SqlxPostgresQueryBindEr> {
                let pattern = self.pattern().map_err(|error| match pg_crud_cmn::SqlxPostgresQueryBindEr::try_from(error.to_string()) {
                    Ok(value) => value,
                    Err(conversion_error) => pg_crud_cmn::SqlxPostgresQueryBindEr::from(conversion_error),
                })?;
                if let Err(error) = query.as_mut().try_bind(String::from(pattern)) {
                    return Err(match pg_crud_cmn::SqlxPostgresQueryBindEr::try_from(error.to_string()) {
                        Ok(value) => value,
                        Err(conversion_error) => pg_crud_cmn::SqlxPostgresQueryBindEr::from(conversion_error),
                    });
                }
                Ok(query)
            }
            fn qp(&self, incr: &mut dyn pg_crud_cmn::QpIncrMut, col: pg_crud_cmn::SqlColRef<'_>, add_oprtr: pg_crud_cmn::AddOprtr) -> Result<pg_crud_cmn::QpFragment, pg_crud_cmn::QpEr> {
                let parameter = incr.checked_add_one().ok_or_else(|| pg_crud_cmn::QpEr::CheckedAdd { loc: loc_macros::loc!() })?;
                let fragment = format!("{}{} {} ${parameter} {}", self.oprtr.to_qp(add_oprtr), col, #sql_operator, #sql_suffix);
                pg_crud_cmn::QpFragment::try_from(fragment).map_err(pg_crud_cmn::QpEr::from)
            }
        }
    }
    .into()
}
