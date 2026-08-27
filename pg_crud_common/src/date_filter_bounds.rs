#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Default)]
pub struct DateFilterBounds<'value_lt> {
    pub(crate) created_at_from: Option<crate::domain_types::ChronoUtcDateTimeRef<'value_lt>>,
    pub(crate) created_at_to: Option<crate::domain_types::ChronoUtcDateTimeRef<'value_lt>>,
    pub(crate) updated_at_from: Option<crate::domain_types::ChronoUtcDateTimeRef<'value_lt>>,
    pub(crate) updated_at_to: Option<crate::domain_types::ChronoUtcDateTimeRef<'value_lt>>,
}

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
