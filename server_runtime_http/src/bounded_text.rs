#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
    proc_macro_newtype_into_inner::IntoInner,
)]
pub struct BoundedText(
    bounded_types::bounded_string::BoundedString<0usize, 16_777_216usize, false>,
);

impl TryFrom<String> for BoundedText {
    type Error = crate::bounded_read_error::BoundedReadError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_16_777_216 {
            return Err(
                crate::bounded_read_error::BoundedReadError::ExceedsMaximum {
                    maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
                        constants_usize::VALUE_16_777_216,
                    ),
                },
            );
        }
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    maximum_length,
                    ..
                }
                | bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    minimum_length: maximum_length,
                    ..
                } => Self::Error::ExceedsMaximum {
                    maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
                        maximum_length.get(),
                    ),
                },
            })
    }
}

impl TryFrom<crate::bounded_bytes::BoundedBytes> for BoundedText {
    type Error = crate::bounded_read_error::BoundedReadError;

    fn try_from(value: crate::bounded_bytes::BoundedBytes) -> Result<Self, Self::Error> {
        let text = String::from_utf8(value.into_inner()).map_err(|source| {
            crate::bounded_read_error::BoundedReadError::Utf8 {
                source: crate::bounded_read_from_utf8_error::BoundedReadFromUtf8Error::from(source),
            }
        })?;
        Self::try_from(text)
    }
}
