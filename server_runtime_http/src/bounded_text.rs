#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    newtype::AsRefStr,
    newtype::IntoInner,
)]
pub struct BoundedText(String);

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
        Ok(Self(value))
    }
}

impl TryFrom<crate::bounded_bytes::BoundedBytes> for BoundedText {
    type Error = crate::bounded_read_error::BoundedReadError;

    fn try_from(value: crate::bounded_bytes::BoundedBytes) -> Result<Self, Self::Error> {
        let text = String::from_utf8(value.0).map_err(|source| {
            crate::bounded_read_error::BoundedReadError::Utf8 {
                source: crate::bounded_read_from_utf8_error::BoundedReadFromUtf8Error::from(source),
            }
        })?;
        Self::try_from(text)
    }
}
