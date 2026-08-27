#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy)]
pub struct FieldLocationColumn(pub(super) super::FieldLocationColumnNonZeroU32);

impl From<std::num::NonZeroU32> for FieldLocationColumn {
    fn from(value: std::num::NonZeroU32) -> Self {
        Self(super::FieldLocationColumnNonZeroU32::from(value))
    }
}

impl TryFrom<u32> for FieldLocationColumn {
    type Error = super::FieldLocationCoordinateTryFromU32Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        std::num::NonZeroU32::new(value)
            .map(Self::from)
            .ok_or(super::FieldLocationCoordinateTryFromU32Error)
    }
}

impl FieldLocationColumn {
    #[must_use]
    pub fn first() -> Self {
        Self::from(std::num::NonZeroU32::MIN)
    }
}
