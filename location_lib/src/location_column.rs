#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    schemars::JsonSchema,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_inner::FromInner,
)]
#[serde(try_from = "u32")]
#[schema(value_type = u32)]
#[schemars(with = "u32")]
pub struct LocationColumn(std::num::NonZeroU32);
impl TryFrom<u32> for LocationColumn {
    type Error = crate::location_coordinate_try_from_u32_error::LocationCoordinateTryFromU32Error;

    fn try_from(u32: u32) -> Result<Self, Self::Error> {
        std::num::NonZeroU32::new(u32)
            .map(Self::from)
            .ok_or(crate::location_coordinate_try_from_u32_error::LocationCoordinateTryFromU32Error::OutOfRange)
    }
}
impl LocationColumn {
    #[must_use]
    pub fn first() -> Self {
        Self::from(std::num::NonZeroU32::MIN)
    }
}
