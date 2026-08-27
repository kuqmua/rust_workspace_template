#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
use super::{AdminDefaultPageLimit, AdminPageLimitError, AdminPageLimitVisitor};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::IntoInnerFrom,
    serde::Serialize,
    utoipa::ToSchema,
)]
pub struct AdminPageLimit(pub(super) u16);
impl<'de> serde::Deserialize<'de> for AdminPageLimit {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = deserializer.deserialize_any(AdminPageLimitVisitor)?;
        Self::try_from(u16::from(value)).map_err(serde::de::Error::custom)
    }
}
impl Default for AdminPageLimit {
    fn default() -> Self {
        Self::from(AdminDefaultPageLimit)
    }
}
impl TryFrom<u16> for AdminPageLimit {
    type Error = AdminPageLimitError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(value))
        } else {
            Err(AdminPageLimitError)
        }
    }
}
impl AdminPageLimit {
    pub const DEFAULT: u16 = 20u16;
    pub const MAX: u16 = 100u16;
    pub const MIN: u16 = 1u16;
}
