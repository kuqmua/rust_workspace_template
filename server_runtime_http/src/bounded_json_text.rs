#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct BoundedJsonText(String);

impl BoundedJsonText {
    pub fn compact(&self) -> Result<Self, crate::bounded_json_read_error::BoundedJsonReadError> {
        let value =
            serde_json::from_str::<serde_json::Value>(self.0.as_str()).map_err(|error| {
                crate::bounded_json_read_error::BoundedJsonReadError::SerdeJson(
                    crate::serde_json_error::SerdeJsonError::from(error),
                )
            })?;
        let text = serde_json::to_string(&value).map_err(|error| {
            crate::bounded_json_read_error::BoundedJsonReadError::SerdeJson(
                crate::serde_json_error::SerdeJsonError::from(error),
            )
        })?;
        Self::try_from(text)
    }

    pub fn pretty(&self) -> Result<Self, crate::bounded_json_read_error::BoundedJsonReadError> {
        let value =
            serde_json::from_str::<serde_json::Value>(self.0.as_str()).map_err(|error| {
                crate::bounded_json_read_error::BoundedJsonReadError::SerdeJson(
                    crate::serde_json_error::SerdeJsonError::from(error),
                )
            })?;
        let text = serde_json::to_string_pretty(&value).map_err(|error| {
            crate::bounded_json_read_error::BoundedJsonReadError::SerdeJson(
                crate::serde_json_error::SerdeJsonError::from(error),
            )
        })?;
        Self::try_from(text)
    }
}

impl TryFrom<String> for BoundedJsonText {
    type Error = crate::bounded_json_read_error::BoundedJsonReadError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_16_777_216 {
            return Err(crate::bounded_json_read_error::BoundedJsonReadError::Read(
                crate::bounded_read_error::BoundedReadError::ExceedsMaximum {
                    maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
                        constants_usize::VALUE_16_777_216,
                    ),
                },
            ));
        }
        let _validated_value =
            serde_json::from_str::<serde_json::Value>(value.as_str()).map_err(|error| {
                crate::bounded_json_read_error::BoundedJsonReadError::SerdeJson(
                    crate::serde_json_error::SerdeJsonError::from(error),
                )
            })?;
        Ok(Self(value))
    }
}
