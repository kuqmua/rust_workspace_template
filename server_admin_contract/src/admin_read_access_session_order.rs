#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_new::New,
    proc_macro_getters::Getters,
)]
pub struct AdminReadAccessSessionOrder {
    column: crate::admin_read_access_session_column::AdminReadAccessSessionColumn,
    order: crate::admin_sort_direction::AdminSortDirection,
}
