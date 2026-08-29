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
    fragment: crate::query_part_fragment::QueryPartFragment,
    values: crate::chrono_utc_date_times::ChronoUtcDateTimes,
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
        crate::query_part_fragment::QueryPartFragment,
        crate::chrono_utc_date_times::ChronoUtcDateTimes,
    ) {
        (self.fragment, self.values)
    }
}
