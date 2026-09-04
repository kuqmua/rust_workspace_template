#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_new::New,
)]
pub struct AdminDataColumn {
    #[getters(skip)]
    #[constructor(order = 0)]
    filters: crate::admin_data_filters::AdminDataFilters,
    #[constructor(order = 2)]
    label: crate::admin_text::AdminText,
    #[constructor(order = 3)]
    name: crate::admin_text::AdminText,
    #[getters(copy)]
    #[constructor(order = 1)]
    input_kind: frontend_contract::input_kind::InputKind,
}
impl AdminDataColumn {
    #[must_use]
    pub const fn filters(&self) -> &[crate::admin_data_filter::AdminDataFilter] {
        self.filters.as_slice()
    }
}
