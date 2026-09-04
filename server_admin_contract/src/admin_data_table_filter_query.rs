#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    serde::Deserialize,
    serde::Serialize,
    utoipa::IntoParams,
    utoipa::ToSchema,
)]
#[into_params(parameter_in = Query)]
#[derive(proc_macro_new::New)]
pub struct AdminDataTableFilterQuery {
    #[serde(default)]
    #[param(value_type = Option<String>, max_length = 63)]
    #[constructor(order = 0)]
    filter_field: Option<crate::admin_filter_field::AdminFilterField>,
    #[serde(default)]
    #[param(value_type = Option<String>, max_length = 4096)]
    #[constructor(order = 2)]
    filter_value: Option<crate::admin_filter_value::AdminFilterValue>,
    #[serde(default)]
    #[param(value_type = Option<String>, max_length = 4096)]
    #[constructor(order = 3)]
    filter_end: Option<crate::admin_filter_value::AdminFilterValue>,
    #[serde(default)]
    #[param(inline)]
    #[constructor(order = 1)]
    filter_operation: Option<frontend_contract::filter_operation::FilterOperation>,
}
impl AdminDataTableFilterQuery {
    #[must_use]
    pub const fn field(&self) -> Option<&crate::admin_filter_field::AdminFilterField> {
        self.filter_field.as_ref()
    }
    #[must_use]
    pub const fn operation(&self) -> Option<frontend_contract::filter_operation::FilterOperation> {
        self.filter_operation
    }
    #[must_use]
    pub const fn value(&self) -> Option<&crate::admin_filter_value::AdminFilterValue> {
        self.filter_value.as_ref()
    }
    #[must_use]
    pub const fn end(&self) -> Option<&crate::admin_filter_value::AdminFilterValue> {
        self.filter_end.as_ref()
    }
}
