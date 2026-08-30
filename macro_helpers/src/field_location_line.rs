#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub struct FieldLocationLine(pub(super) std::num::NonZeroU32);

impl TryFrom<u32> for FieldLocationLine {
    type Error =
        crate::field_location_coordinate_try_from_u32_error::FieldLocationCoordinateTryFromU32Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        std::num::NonZeroU32::new(value)
            .map(Self::from)
            .ok_or(crate::field_location_coordinate_try_from_u32_error::FieldLocationCoordinateTryFromU32Error::OutOfRange)
    }
}

impl FieldLocationLine {
    #[must_use]
    pub fn first() -> Self {
        Self::from(std::num::NonZeroU32::MIN)
    }
}
