pub fn calculate_resource_utilization(
    used: crate::resource_amount::ResourceAmount,
    maximum: crate::resource_amount::ResourceAmount,
) -> Result<
    crate::resource_utilization::ResourceUtilization,
    crate::resource_utilization_error::ResourceUtilizationError,
> {
    if maximum.0 == constants_u64::ZERO {
        return Err(crate::resource_utilization_error::ResourceUtilizationError::ZeroMaximum);
    }
    let percent_u128 = u128::from(used.0)
        .saturating_mul(100u128)
        .div_euclid(u128::from(maximum.0))
        .min(100u128);
    let percent_u8 = u8::try_from(percent_u128).unwrap_or(100u8);
    let percent =
        crate::resource_utilization_percent::ResourceUtilizationPercent::try_from(percent_u8)
            .unwrap_or_else(|_error| {
                crate::resource_utilization_percent::ResourceUtilizationPercent::from(
                    crate::resource_utilization_known_percent::ResourceUtilizationKnownPercent::Max,
                )
            });
    let status = match percent.0 {
        crate::reject_non_essential_writes_percent::REJECT_NON_ESSENTIAL_WRITES_PERCENT
            ..=u8::MAX => {
            crate::resource_utilization_status::ResourceUtilizationStatus::RejectNonEssentialWrites
        }
        crate::critical_percent::CRITICAL_PERCENT
            ..crate::reject_non_essential_writes_percent::REJECT_NON_ESSENTIAL_WRITES_PERCENT => {
            crate::resource_utilization_status::ResourceUtilizationStatus::Critical
        }
        crate::warning_percent::WARNING_PERCENT..crate::critical_percent::CRITICAL_PERCENT => {
            crate::resource_utilization_status::ResourceUtilizationStatus::Warning
        }
        constants_u8::ZERO..crate::warning_percent::WARNING_PERCENT => {
            crate::resource_utilization_status::ResourceUtilizationStatus::Ok
        }
    };
    Ok(crate::resource_utilization::ResourceUtilization {
        maximum,
        used,
        percent,
        status,
    })
}
