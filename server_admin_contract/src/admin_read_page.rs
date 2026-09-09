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
pub struct AdminReadPage {
    offset: crate::admin_page_offset::AdminPageOffset,
    limit: crate::admin_page_limit::AdminPageLimit,
}
