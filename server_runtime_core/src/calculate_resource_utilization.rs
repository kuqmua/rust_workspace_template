pub fn calculate_resource_utilization(
    used: super::ResourceAmount,
    maximum: super::ResourceAmount,
) -> Result<super::ResourceUtilization, super::ResourceUtilizationError> {
    if maximum.0 == constants_u64::ZERO {
        return Err(super::ResourceUtilizationError::ZeroMaximum);
    }
    let percent_u128 = u128::from(used.0)
        .saturating_mul(100u128)
        .div_euclid(u128::from(maximum.0))
        .min(100u128);
    let percent_u8 = u8::try_from(percent_u128).unwrap_or(100u8);
    let percent =
        super::ResourceUtilizationPercent::try_from(percent_u8).unwrap_or_else(|_error| {
            super::ResourceUtilizationPercent::from(super::ResourceUtilizationKnownPercent::Max)
        });
    let status = match percent.0 {
        super::REJECT_NON_ESSENTIAL_WRITES_PERCENT..=u8::MAX => {
            super::ResourceUtilizationStatus::RejectNonEssentialWrites
        }
        super::CRITICAL_PERCENT..super::REJECT_NON_ESSENTIAL_WRITES_PERCENT => {
            super::ResourceUtilizationStatus::Critical
        }
        super::WARNING_PERCENT..super::CRITICAL_PERCENT => {
            super::ResourceUtilizationStatus::Warning
        }
        constants_u8::ZERO..super::WARNING_PERCENT => super::ResourceUtilizationStatus::Ok,
    };
    Ok(super::ResourceUtilization {
        maximum,
        used,
        percent,
        status,
    })
}
