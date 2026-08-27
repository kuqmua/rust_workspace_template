#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct ChronoUtcDateTimeRef<'value_lt>(pub(crate) &'value_lt chrono::DateTime<chrono::Utc>);
