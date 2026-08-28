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
    validator = ResourceUtilizationPercent::validate
)]
pub struct ResourceUtilizationPercent(pub(super) u8);

impl From<super::ResourceUtilizationKnownPercent> for ResourceUtilizationPercent {
    fn from(value: super::ResourceUtilizationKnownPercent) -> Self {
        match value {
            super::ResourceUtilizationKnownPercent::Max => Self(100u8),
        }
    }
}

impl ResourceUtilizationPercent {
    // The owner module retains lint-sensitive semantics from the original implementation.
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)]
    const fn validate(value: &u8) -> Result<(), super::ResourceUtilizationPercentTryFromU8Error> {
        if *value <= 100u8 {
            Ok(())
        } else {
            Err(super::ResourceUtilizationPercentTryFromU8Error)
        }
    }
}
