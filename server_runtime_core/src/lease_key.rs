#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, Hash, PartialEq, newtype::AsRefStr,
)]
pub struct LeaseKey(String);
impl TryFrom<String> for LeaseKey {
    type Error = crate::lease_text_error::LeaseTextError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::validate_lease_text::validate_lease_text(crate::lease_text_ref::LeaseTextRef(&value))
            .map(|()| Self(value))
    }
}
