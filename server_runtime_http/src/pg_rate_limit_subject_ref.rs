#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_get_inner::GetInner,
)]
#[accessor(pub(crate))]
pub struct PgRateLimitSubjectRef<'value_lt>(&'value_lt str);

impl<'value_lt> TryFrom<&'value_lt str> for PgRateLimitSubjectRef<'value_lt> {
    type Error = crate::pg_rate_limit_validation_error::PgRateLimitValidationError;

    fn try_from(value: &'value_lt str) -> Result<Self, Self::Error> {
        crate::pg_rate_limit_scope_ref::PgRateLimitScopeRef::try_from(value)
            .map(|scope| Self(scope.get()))
    }
}
