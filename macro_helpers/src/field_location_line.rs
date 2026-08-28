#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub struct FieldLocationLine(pub(super) super::FieldLocationLineNonZeroU32);

impl From<std::num::NonZeroU32> for FieldLocationLine {
    fn from(value: std::num::NonZeroU32) -> Self {
        Self(super::FieldLocationLineNonZeroU32::from(value))
    }
}

impl TryFrom<u32> for FieldLocationLine {
    type Error = super::FieldLocationCoordinateTryFromU32Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        std::num::NonZeroU32::new(value)
            .map(Self::from)
            .ok_or(super::FieldLocationCoordinateTryFromU32Error)
    }
}

impl FieldLocationLine {
    #[must_use]
    pub fn first() -> Self {
        Self::from(std::num::NonZeroU32::MIN)
    }
}
