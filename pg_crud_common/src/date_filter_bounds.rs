#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    generate_accessor::Getters,
)]
#[getters(get_mut)]
pub struct DateFilterBounds<'value_lt> {
    created_at_from: Option<crate::domain_types::ChronoUtcDateTimeRef<'value_lt>>,
    created_at_to: Option<crate::domain_types::ChronoUtcDateTimeRef<'value_lt>>,
    updated_at_from: Option<crate::domain_types::ChronoUtcDateTimeRef<'value_lt>>,
    updated_at_to: Option<crate::domain_types::ChronoUtcDateTimeRef<'value_lt>>,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl<'value_lt> DateFilterBounds<'value_lt> {
    #[must_use]
    pub const fn new(
        created_at_from: Option<crate::domain_types::ChronoUtcDateTimeRef<'value_lt>>,
        created_at_to: Option<crate::domain_types::ChronoUtcDateTimeRef<'value_lt>>,
        updated_at_from: Option<crate::domain_types::ChronoUtcDateTimeRef<'value_lt>>,
        updated_at_to: Option<crate::domain_types::ChronoUtcDateTimeRef<'value_lt>>,
    ) -> Self {
        Self {
            created_at_from,
            created_at_to,
            updated_at_from,
            updated_at_to,
        }
    }
}
