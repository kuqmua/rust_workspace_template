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
    proc_macro_newtype_try_from::TryFrom,
)]
#[try_from(
    error = crate::resource_utilization_percent_try_from_u8_error::ResourceUtilizationPercentTryFromU8Error,
    validator = |value: &u8| {
        if *value <= 100u8 { Ok(()) } else { Err(crate::resource_utilization_percent_try_from_u8_error::ResourceUtilizationPercentTryFromU8Error::OutOfRange) }
    }
)]
pub struct ResourceUtilizationPercent(u8);

impl From<crate::resource_utilization_known_percent::ResourceUtilizationKnownPercent>
    for ResourceUtilizationPercent
{
    fn from(
        resource_utilization_known_percent: crate::resource_utilization_known_percent::ResourceUtilizationKnownPercent,
    ) -> Self {
        match resource_utilization_known_percent {
            crate::resource_utilization_known_percent::ResourceUtilizationKnownPercent::Max => {
                Self(100u8)
            }
        }
    }
}
