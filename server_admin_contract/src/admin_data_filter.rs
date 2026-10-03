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

#[cfg(test)]
mod tests {
    #[test]
    fn test_data_filter_shapes_enforce_input_requirements_and_wire_consistency() {
        let shapes = [
            frontend_contract::filter_value_shape::FilterValueShape::EncodedText,
            frontend_contract::filter_value_shape::FilterValueShape::List,
            frontend_contract::filter_value_shape::FilterValueShape::None,
            frontend_contract::filter_value_shape::FilterValueShape::Range,
            frontend_contract::filter_value_shape::FilterValueShape::Regex,
            frontend_contract::filter_value_shape::FilterValueShape::Scalar,
        ];
        let cases = [
            (frontend_contract::filter_operation::FilterOperation::EqToEncodedStringRepresentation, frontend_contract::filter_value_shape::FilterValueShape::EncodedText, true, false),
            (frontend_contract::filter_operation::FilterOperation::In, frontend_contract::filter_value_shape::FilterValueShape::List, true, false),
            (frontend_contract::filter_operation::FilterOperation::CurrentTimestamp, frontend_contract::filter_value_shape::FilterValueShape::None, false, false),
            (frontend_contract::filter_operation::FilterOperation::Between, frontend_contract::filter_value_shape::FilterValueShape::Range, true, true),
            (frontend_contract::filter_operation::FilterOperation::Regex, frontend_contract::filter_value_shape::FilterValueShape::Regex, true, false),
            (frontend_contract::filter_operation::FilterOperation::Eq, frontend_contract::filter_value_shape::FilterValueShape::Scalar, true, false),
        ];
        assert!(cases.into_iter().all(|(operation, expected_shape, requires_value, requires_end)| {
            let filter = super::AdminDataFilter::from(operation);
            filter.operation() == operation
                && filter.value_shape() == expected_shape
                && filter.requires_value() == crate::admin_bool::AdminBool::from(requires_value)
                && filter.requires_end() == crate::admin_bool::AdminBool::from(requires_end)
                && shapes.into_iter().all(|value_shape| {
                    let wire = serde_json::json!({(stringify!(operation)): operation, (stringify!(value_shape)): value_shape});
                    let result = serde_json::from_value::<super::AdminDataFilter>(wire.clone());
                    if value_shape == expected_shape {
                        result.is_ok_and(|parsed| parsed == filter && serde_json::to_value(parsed).is_ok_and(|serialized| serialized == wire))
                    } else {
                        result.is_err_and(|error| error.is_data() && error.to_string().contains(constants_str::INVALID_FILTER_SPECIFICATION))
                    }
                })
        }));
        assert!([serde_json::json!({}), serde_json::json!({(stringify!(operation)): frontend_contract::filter_operation::FilterOperation::Eq}), serde_json::json!({(stringify!(value_shape)): frontend_contract::filter_value_shape::FilterValueShape::Scalar}), serde_json::json!([])].into_iter().all(|wire| serde_json::from_value::<super::AdminDataFilter>(wire).is_err()));
    }
}
