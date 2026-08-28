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
    type Error = super::BoundedReadError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > constants_usize::VALUE_16_777_216 {
            return Err(super::BoundedReadError::ExceedsMaximum {
                maximum_bytes: super::BoundedReadMaximumBytes::from(
                    constants_usize::VALUE_16_777_216,
                ),
            });
        }
        Ok(Self(value))
    }
}

impl TryFrom<super::BoundedBytes> for BoundedText {
    type Error = super::BoundedReadError;

    fn try_from(value: super::BoundedBytes) -> Result<Self, Self::Error> {
        let text = String::from_utf8(value.0).map_err(|source| super::BoundedReadError::Utf8 {
            source: super::BoundedReadFromUtf8Error::from(source),
        })?;
        Self::try_from(text)
    }
}
