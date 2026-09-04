#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    proc_macro_newtype_get_inner::GetInner,
)]
pub struct ResourceUtilizationPercent(u8);
impl TryFrom<u8> for ResourceUtilizationPercent {
    type Error = crate::resource_utilization_percent_try_from_u8_error::ResourceUtilizationPercentTryFromU8Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 100u8 {
            Ok(Self(value))
        } else {
            Err(Self::Error::OutOfRange)
        }
    }
}

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
