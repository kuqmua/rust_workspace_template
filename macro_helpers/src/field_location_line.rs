#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct FieldLocationLine(std::num::NonZeroU32);

impl TryFrom<u32> for FieldLocationLine {
    type Error =
        crate::field_location_coordinate_try_from_u32_error::FieldLocationCoordinateTryFromU32Error;

    fn try_from(u32: u32) -> Result<Self, Self::Error> {
        std::num::NonZeroU32::new(u32)
            .map(Self::from)
            .ok_or(crate::field_location_coordinate_try_from_u32_error::FieldLocationCoordinateTryFromU32Error::OutOfRange)
    }
}

impl FieldLocationLine {
    #[must_use]
    pub fn first() -> Self {
        Self::from(std::num::NonZeroU32::MIN)
    }

    pub(crate) const fn value(self) -> u32 {
        self.0.get()
    }
}
