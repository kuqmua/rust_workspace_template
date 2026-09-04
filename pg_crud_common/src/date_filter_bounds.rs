#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    proc_macro_getters::Getters,
)]
#[getters(get_mut)]
#[derive(proc_macro_new::New)]
pub struct DateFilterBounds<'value_lt> {
    created_at_from: Option<crate::chrono_utc_date_time_ref::ChronoUtcDateTimeRef<'value_lt>>,
    created_at_to: Option<crate::chrono_utc_date_time_ref::ChronoUtcDateTimeRef<'value_lt>>,
    updated_at_from: Option<crate::chrono_utc_date_time_ref::ChronoUtcDateTimeRef<'value_lt>>,
    updated_at_to: Option<crate::chrono_utc_date_time_ref::ChronoUtcDateTimeRef<'value_lt>>,
}
