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

#[cfg(test)]
mod tests {
    #[test]
    fn test_json_formatting_enforces_output_bounds_after_pretty_expansion() {
        let maximum = constants_usize::VALUE_16_777_216;
        assert!([maximum - 4usize, maximum - 3usize, maximum, maximum + 1usize]
            .into_iter()
            .all(|length| {
                let mut text = String::with_capacity(length);
                text.push('[');
                text.push('"');
                text.push_str(&constants_str::X.repeat(length - 4usize));
                text.push('"');
                text.push(']');
                let converted = crate::bounded_json_text::BoundedJsonText::try_from(text);
                if length > maximum {
                    return converted.is_err_and(|error| matches!(error,
                        crate::bounded_json_read_error::BoundedJsonReadError::Read(
                            crate::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes }
                        ) if maximum_bytes.get() == maximum
                    ));
                }
                converted.is_ok_and(|json| {
                    let compact = json.compact();
                    let pretty = json.pretty();
                    compact.is_ok_and(|value| value == json)
                        && if length == maximum - 4usize {
                            pretty.is_ok_and(|value| {
                                value.as_ref().len() == maximum
                                    && value.compact().is_ok_and(|restored| restored == json)
                            })
                        } else {
                            pretty.is_err_and(|error| matches!(error,
                                crate::bounded_json_read_error::BoundedJsonReadError::Read(
                                    crate::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes }
                                ) if maximum_bytes.get() == maximum
                            ))
                        }
                })
            }));
    }
}
