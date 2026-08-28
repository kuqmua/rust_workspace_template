#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
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
    operation: frontend_contract::domain_types::FilterOperation,
    value_shape: frontend_contract::domain_types::FilterValueShape,
}
impl From<frontend_contract::domain_types::FilterOperation> for AdminDataFilter {
    fn from(value: frontend_contract::domain_types::FilterOperation) -> Self {
        Self {
            operation: value,
            value_shape: value.value_shape(),
        }
    }
}
impl AdminDataFilter {
    #[must_use]
    pub const fn operation(&self) -> frontend_contract::domain_types::FilterOperation {
        self.operation
    }
    #[must_use]
    pub const fn value_shape(&self) -> frontend_contract::domain_types::FilterValueShape {
        self.value_shape
    }
    #[must_use]
    pub fn requires_value(&self) -> crate::domain_types::AdminBool {
        crate::domain_types::AdminBool::from(!matches!(
            self.value_shape,
            frontend_contract::domain_types::FilterValueShape::None
        ))
    }
    #[must_use]
    pub fn requires_end(&self) -> crate::domain_types::AdminBool {
        crate::domain_types::AdminBool::from(matches!(
            self.value_shape,
            frontend_contract::domain_types::FilterValueShape::Range
        ))
    }
}
