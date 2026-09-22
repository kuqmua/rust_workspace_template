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
pub struct AdminReadRuleOrder {
    column: crate::admin_read_rule_column::AdminReadRuleColumn,
    order: crate::admin_sort_direction::AdminSortDirection,
}
