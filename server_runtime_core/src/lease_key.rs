#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Hash, PartialEq, newtype::AsRefStr,
)]
pub struct LeaseKey(String);
impl TryFrom<String> for LeaseKey {
    type Error = crate::lease_text_error::LeaseTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > crate::lease_text_maximum_bytes::LEASE_TEXT_MAXIMUM_BYTES {
            return Err(crate::lease_text_error::LeaseTextError::TooLong);
        }
        crate::validate_lease_text::validate_lease_text(crate::lease_text_ref::LeaseTextRef(&value))
            .map(|()| Self(value))
    }
}
