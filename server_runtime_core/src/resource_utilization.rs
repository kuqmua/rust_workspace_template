const CRITICAL_PERCENT: u8 = 85u8;
const REJECT_NON_ESSENTIAL_WRITES_PERCENT: u8 = 95u8;
const WARNING_PERCENT: u8 = 70u8;

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    newtype::FromInner,
)]
pub struct ResourceAmount(u64);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    newtype::TryFrom,
)]
#[try_from(
    error = ResourceUtilizationPercentTryFromU8Error,
    validator = ResourceUtilizationPercent::validate
)]
pub struct ResourceUtilizationPercent(u8);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
enum ResourceUtilizationKnownPercent {
    Max,
}
impl From<ResourceUtilizationKnownPercent> for ResourceUtilizationPercent {
    fn from(value: ResourceUtilizationKnownPercent) -> Self {
        match value {
            ResourceUtilizationKnownPercent::Max => Self(100u8),
        }
    }
}
#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{self:?}")]
pub struct ResourceUtilizationPercentTryFromU8Error;

impl ResourceUtilizationPercent {
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
    #[allow(clippy::single_call_fn, clippy::trivially_copy_pass_by_ref)] // derive-generated TryFrom owns the single call and borrows the inner value
    const fn validate(value: &u8) -> Result<(), ResourceUtilizationPercentTryFromU8Error> {
        if *value <= 100u8 {
            Ok(())
        } else {
            Err(ResourceUtilizationPercentTryFromU8Error)
        }
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceUtilizationStatus {
    Critical,
    Ok,
    RejectNonEssentialWrites,
    Warning,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum ResourceUtilizationError {
    #[error(
        "{}",
        str_constants::RESOURCE_UTILIZATION_MAXIMUM_MUST_BE_GREATER_THAN_ZERO
    )]
    ZeroMaximum,
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::arbitrary_source_item_ordering)] // alignment order required by optimal_memory_layout takes precedence over alphabetical field order
pub struct ResourceUtilization {
    maximum: ResourceAmount,
    used: ResourceAmount,
    percent: ResourceUtilizationPercent,
    status: ResourceUtilizationStatus,
}

impl ResourceUtilization {
    #[must_use]
    pub const fn maximum(self) -> ResourceAmount {
        self.maximum
    }

    #[must_use]
    pub const fn percent(self) -> ResourceUtilizationPercent {
        self.percent
    }

    #[must_use]
    pub const fn status(self) -> ResourceUtilizationStatus {
        self.status
    }

    #[must_use]
    pub const fn used(self) -> ResourceAmount {
        self.used
    }
}

pub fn calculate_resource_utilization(
    used: ResourceAmount,
    maximum: ResourceAmount,
) -> Result<ResourceUtilization, ResourceUtilizationError> {
    if maximum.0 == u64_constants::ZERO {
        return Err(ResourceUtilizationError::ZeroMaximum);
    }
    let percent_u128 = u128::from(used.0)
        .saturating_mul(100u128)
        .div_euclid(u128::from(maximum.0))
        .min(100u128);
    let percent_u8 = u8::try_from(percent_u128).unwrap_or(100u8);
    let percent = ResourceUtilizationPercent::try_from(percent_u8).unwrap_or_else(|_error| {
        ResourceUtilizationPercent::from(ResourceUtilizationKnownPercent::Max)
    });
    let status = match percent.0 {
        REJECT_NON_ESSENTIAL_WRITES_PERCENT..=u8::MAX => {
            ResourceUtilizationStatus::RejectNonEssentialWrites
        }
        CRITICAL_PERCENT..REJECT_NON_ESSENTIAL_WRITES_PERCENT => {
            ResourceUtilizationStatus::Critical
        }
        WARNING_PERCENT..CRITICAL_PERCENT => ResourceUtilizationStatus::Warning,
        u8_constants::ZERO..WARNING_PERCENT => ResourceUtilizationStatus::Ok,
    };
    Ok(ResourceUtilization {
        maximum,
        used,
        percent,
        status,
    })
}

#[cfg(test)]
mod tests {
    fn calculate(used: u64, maximum: u64) -> super::ResourceUtilization {
        super::calculate_resource_utilization(
            super::ResourceAmount::from(used),
            super::ResourceAmount::from(maximum),
        )
        .expect("8c23bc92 calculate invariant must hold")
    }

    #[test]
    fn classifies_every_threshold_boundary() {
        assert_eq!(
            calculate(69u64, 100u64).status(),
            super::ResourceUtilizationStatus::Ok
        );
        assert_eq!(
            calculate(70u64, 100u64).status(),
            super::ResourceUtilizationStatus::Warning
        );
        assert_eq!(
            calculate(85u64, 100u64).status(),
            super::ResourceUtilizationStatus::Critical
        );
        assert_eq!(
            calculate(95u64, 100u64).status(),
            super::ResourceUtilizationStatus::RejectNonEssentialWrites
        );
    }

    #[test]
    fn caps_over_capacity_percent_without_overflow() {
        let utilization = calculate(u64::MAX, 1u64);
        assert_eq!(utilization.percent().get(), 100u8);
        assert_eq!(
            utilization.status(),
            super::ResourceUtilizationStatus::RejectNonEssentialWrites
        );
        assert_eq!(utilization.used(), super::ResourceAmount::from(u64::MAX));
        assert_eq!(utilization.maximum(), super::ResourceAmount::from(1u64));
    }

    #[test]
    fn percentage_uses_integer_floor_and_zero_usage_is_ok() {
        let zero = calculate(u64_constants::ZERO, u64::MAX);
        assert_eq!(zero.percent().get(), u8_constants::ZERO);
        assert_eq!(zero.status(), super::ResourceUtilizationStatus::Ok);
        let rounded_down = calculate(699u64, 1000u64);
        assert_eq!(rounded_down.percent().get(), 69u8);
        assert_eq!(rounded_down.status(), super::ResourceUtilizationStatus::Ok);
    }

    #[test]
    fn rejects_zero_maximum() {
        assert_eq!(
            super::calculate_resource_utilization(
                super::ResourceAmount::from(u64_constants::ZERO),
                super::ResourceAmount::from(u64_constants::ZERO),
            ),
            Err(super::ResourceUtilizationError::ZeroMaximum)
        );
    }
    #[test]
    fn percentage_rejects_values_above_one_hundred() {
        let _error = super::ResourceUtilizationPercent::try_from(101u8).expect_err("7ba1d197");
        assert_eq!(
            super::ResourceUtilizationPercent::try_from(100u8)
                .expect("f17abeab percentage_rejects_values_above_one_hundred invariant must hold")
                .get(),
            100u8
        );
    }
}
