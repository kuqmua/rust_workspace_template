#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
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
    fn from(value: frontend_contract::filter_operation::FilterOperation) -> Self {
        Self {
            operation: value,
            value_shape: value.value_shape(),
        }
    }
}
impl<'de> serde::Deserialize<'de> for AdminDataFilter {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, serde::Deserialize)]
        struct AdminDataFilterWire {
            operation: frontend_contract::filter_operation::FilterOperation,
            value_shape: frontend_contract::filter_value_shape::FilterValueShape,
        }
        let wire = <AdminDataFilterWire as serde::Deserialize>::deserialize(deserializer)?;
        if wire.operation.value_shape() != wire.value_shape {
            return Err(serde::de::Error::custom(
                constants_str::INVALID_FILTER_SPECIFICATION,
            ));
        }
        Ok(Self::from(wire.operation))
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
