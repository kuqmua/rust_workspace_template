#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    generate_accessor::Getters,
)]
pub struct DateSqlFilter {
    fragment: crate::domain_types::QueryPartFragment,
    values: crate::domain_types::ChronoUtcDateTimes,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl DateSqlFilter {
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        crate::domain_types::QueryPartFragment,
        crate::domain_types::ChronoUtcDateTimes,
    ) {
        (self.fragment, self.values)
    }
    #[allow(
        clippy::single_call_fn,
        reason = "constructor keeps private field initialization inside the domain type"
    )]
    pub(crate) const fn new(
        fragment: crate::domain_types::QueryPartFragment,
        values: crate::domain_types::ChronoUtcDateTimes,
    ) -> Self {
        Self { fragment, values }
    }
}
