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
    error = crate::resource_utilization_percent_try_from_u8_error::ResourceUtilizationPercentTryFromU8Error,
    validator = |value: &u8| {
        if *value <= 100u8 { Ok(()) } else { Err(crate::resource_utilization_percent_try_from_u8_error::ResourceUtilizationPercentTryFromU8Error) }
    }
)]
pub struct ResourceUtilizationPercent(pub(super) u8);

impl From<crate::resource_utilization_known_percent::ResourceUtilizationKnownPercent>
    for ResourceUtilizationPercent
{
    fn from(
        value: crate::resource_utilization_known_percent::ResourceUtilizationKnownPercent,
    ) -> Self {
        match value {
            crate::resource_utilization_known_percent::ResourceUtilizationKnownPercent::Max => {
                Self(100u8)
            }
        }
    }
}
