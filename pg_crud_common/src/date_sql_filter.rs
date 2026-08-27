#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct DateSqlFilter {
    pub(crate) fragment: crate::domain_types::QueryPartFragment,
    pub(crate) values: crate::domain_types::ChronoUtcDateTimes,
}
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
