#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    proc_macro_newtype_as_ref_str::AsRefStr,
)]
pub struct LeaseId(
    bounded_types::bounded_string::BoundedString<
        1usize,
        { crate::lease_text_maximum_bytes::LEASE_TEXT_MAXIMUM_BYTES },
        false,
    >,
);
impl TryFrom<String> for LeaseId {
    type Error = crate::lease_text_error::LeaseTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::validate_lease_text::validate_lease_text(
            crate::lease_text_ref::LeaseTextRef::from(value.as_str()),
        )?;
        bounded_types::bounded_string::BoundedString::try_from(value)
            .map(Self)
            .map_err(|source| match source {
                bounded_types::bounded_string_error::BoundedStringError::AboveMaximum {
                    ..
                } => Self::Error::TooLong,
                bounded_types::bounded_string_error::BoundedStringError::BelowMinimum {
                    ..
                } => Self::Error::Empty,
            })
    }
}
