#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    newtype::GetInner,
    newtype::TryFrom,
)]
#[try_from(
    error = super::ResourceUtilizationPercentTryFromU8Error,
    validator = |value: &u8| {
        if *value <= 100u8 { Ok(()) } else { Err(super::ResourceUtilizationPercentTryFromU8Error) }
    }
)]
pub struct ResourceUtilizationPercent(pub(super) u8);

impl From<super::ResourceUtilizationKnownPercent> for ResourceUtilizationPercent {
    fn from(value: super::ResourceUtilizationKnownPercent) -> Self {
        match value {
            super::ResourceUtilizationKnownPercent::Max => Self(100u8),
        }
    }
}
