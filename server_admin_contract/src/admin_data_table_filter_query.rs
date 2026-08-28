use super::{AdminFilterField, AdminFilterValue};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    serde::Deserialize,
    serde::Serialize,
    utoipa::IntoParams,
    utoipa::ToSchema,
)]
#[into_params(parameter_in = Query)]
pub struct AdminDataTableFilterQuery {
    #[serde(default)]
    #[param(value_type = Option<String>, max_length = 63)]
    filter_field: Option<AdminFilterField>,
    #[serde(default)]
    #[param(value_type = Option<String>, max_length = 4096)]
    filter_value: Option<AdminFilterValue>,
    #[serde(default)]
    #[param(value_type = Option<String>, max_length = 4096)]
    filter_end: Option<AdminFilterValue>,
    #[serde(default)]
    #[param(inline)]
    filter_operation: Option<frontend_contract::domain_types::FilterOperation>,
}
impl AdminDataTableFilterQuery {
    #[must_use]
    pub const fn new(
        filter_field: Option<AdminFilterField>,
        filter_operation: Option<frontend_contract::domain_types::FilterOperation>,
        filter_value: Option<AdminFilterValue>,
        filter_end: Option<AdminFilterValue>,
    ) -> Self {
        Self {
            filter_field,
            filter_value,
            filter_end,
            filter_operation,
        }
    }
    #[must_use]
    pub const fn field(&self) -> Option<&AdminFilterField> {
        self.filter_field.as_ref()
    }
    #[must_use]
    pub const fn operation(&self) -> Option<frontend_contract::domain_types::FilterOperation> {
        self.filter_operation
    }
    #[must_use]
    pub const fn value(&self) -> Option<&AdminFilterValue> {
        self.filter_value.as_ref()
    }
    #[must_use]
    pub const fn end(&self) -> Option<&AdminFilterValue> {
        self.filter_end.as_ref()
    }
}
