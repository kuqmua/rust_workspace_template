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
    generate_constructor::New,
)]
#[constructor(pub(crate))]
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
}
