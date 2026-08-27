use super::{LEASE_TEXT_MAXIMUM_BYTES, LeaseTextError, LeaseTextRef, validate_lease_text};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Hash, PartialEq, newtype::AsRefStr,
)]
pub struct LeaseKey(String);
impl TryFrom<String> for LeaseKey {
    type Error = LeaseTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > LEASE_TEXT_MAXIMUM_BYTES {
            return Err(LeaseTextError::TooLong);
        }
        validate_lease_text(LeaseTextRef(&value)).map(|()| Self(value))
    }
}
