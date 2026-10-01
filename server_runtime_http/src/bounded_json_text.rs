#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct BoundedJsonText(
    bounded_types::bounded_string::BoundedString<0usize, 16_777_216usize, false>,
);

impl BoundedJsonText {
    pub fn compact(&self) -> Result<Self, crate::bounded_json_read_error::BoundedJsonReadError> {
        self.format_json_text(crate::json_text_format::JsonTextFormat::Compact)
    }

    pub fn pretty(&self) -> Result<Self, crate::bounded_json_read_error::BoundedJsonReadError> {
        self.format_json_text(crate::json_text_format::JsonTextFormat::Pretty)
    }

    fn format_json_text(
        &self,
        json_text_format: crate::json_text_format::JsonTextFormat,
    ) -> Result<Self, crate::bounded_json_read_error::BoundedJsonReadError> {
        let value =
            serde_json::from_str::<serde_json::Value>(self.0.as_str()).map_err(|error| {
                crate::bounded_json_read_error::BoundedJsonReadError::SerdeJson(
                    crate::serde_json_error::SerdeJsonError::from(error),
                )
            })?;
        let text = match json_text_format {
            crate::json_text_format::JsonTextFormat::Compact => serde_json::to_string(&value),
            crate::json_text_format::JsonTextFormat::Pretty => serde_json::to_string_pretty(&value),
        }
        .map_err(|error| {
            crate::bounded_json_read_error::BoundedJsonReadError::SerdeJson(
                crate::serde_json_error::SerdeJsonError::from(error),
            )
        })?;
        bounded_types::bounded_string::BoundedString::try_from(text)
            .map(Self)
            .map_err(crate::bounded_json_read_error::BoundedJsonReadError::from_string_bounds)
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
            serde_json::from_str::<crate::validated_json_value::ValidatedJsonValue>(value.as_str())
                .map_err(|error| {
                    crate::bounded_json_read_error::BoundedJsonReadError::SerdeJson(
                        crate::serde_json_error::SerdeJsonError::from(error),
                    )
                })?;
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(crate::bounded_json_read_error::BoundedJsonReadError::from_string_bounds)
    }
}
