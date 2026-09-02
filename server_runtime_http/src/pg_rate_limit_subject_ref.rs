#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub struct PgRateLimitSubjectRef<'value_lt>(&'value_lt str);

impl<'value_lt> PgRateLimitSubjectRef<'value_lt> {
    #[must_use]
    pub(crate) const fn get(self) -> &'value_lt str {
        self.0
    }
}

impl<'value_lt> TryFrom<&'value_lt str> for PgRateLimitSubjectRef<'value_lt> {
    type Error = crate::pg_rate_limit_validation_error::PgRateLimitValidationError;

    fn try_from(value: &'value_lt str) -> Result<Self, Self::Error> {
        crate::pg_rate_limit_scope_ref::PgRateLimitScopeRef::try_from(value)
            .map(|scope| Self(scope.get()))
    }
}
