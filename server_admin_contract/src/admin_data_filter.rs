#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct AdminDataFilter {
    #[getters(copy)]
    operation: frontend_contract::filter_operation::FilterOperation,
    #[getters(copy)]
    value_shape: frontend_contract::filter_value_shape::FilterValueShape,
}
impl From<frontend_contract::filter_operation::FilterOperation> for AdminDataFilter {
    fn from(filter_operation: frontend_contract::filter_operation::FilterOperation) -> Self {
        Self {
            operation: filter_operation,
            value_shape: filter_operation.value_shape(),
        }
    }
}
impl AdminDataFilter {
    #[must_use]
    pub fn requires_value(&self) -> crate::admin_bool::AdminBool {
        crate::admin_bool::AdminBool::from(!matches!(
            self.value_shape,
            frontend_contract::filter_value_shape::FilterValueShape::None
        ))
    }
    #[must_use]
    pub fn requires_end(&self) -> crate::admin_bool::AdminBool {
        crate::admin_bool::AdminBool::from(matches!(
            self.value_shape,
            frontend_contract::filter_value_shape::FilterValueShape::Range
        ))
    }
}
