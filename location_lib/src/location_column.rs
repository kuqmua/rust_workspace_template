use crate::domain_types::{LocationColumnNonZeroU32, LocationCoordinateTryFromU32Error};

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
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::Display,
)]
#[serde(try_from = "u32")]
#[schema(value_type = u32)]
#[schemars(with = "u32")]
pub struct LocationColumn(LocationColumnNonZeroU32);
impl From<std::num::NonZeroU32> for LocationColumn {
    fn from(value: std::num::NonZeroU32) -> Self {
        Self(LocationColumnNonZeroU32::from(value))
    }
}
impl TryFrom<u32> for LocationColumn {
    type Error = LocationCoordinateTryFromU32Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        std::num::NonZeroU32::new(value)
            .map(Self::from)
            .ok_or(LocationCoordinateTryFromU32Error)
    }
}
impl LocationColumn {
    #[must_use]
    pub fn first() -> Self {
        Self::from(std::num::NonZeroU32::MIN)
    }
}
