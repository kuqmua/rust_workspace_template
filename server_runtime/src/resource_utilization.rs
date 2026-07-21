const CRITICAL_PERCENT: u8 = 85u8;
const REJECT_NON_ESSENTIAL_WRITES_PERCENT: u8 = 95u8;
const WARNING_PERCENT: u8 = 70u8;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ResourceAmount(u64);

impl From<u64> for ResourceAmount {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ResourceUtilizationPercent(u8);
impl From<u8> for ResourceUtilizationPercent {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl ResourceUtilizationPercent {
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceUtilizationStatus {
    Critical,
    Ok,
    RejectNonEssentialWrites,
    Warning,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ResourceUtilizationError {
    #[error(
        "{}",
        str_constants::RESOURCE_UTILIZATION_MAXIMUM_MUST_BE_GREATER_THAN_ZERO
    )]
    ZeroMaximum,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceUtilization {
    maximum: ResourceAmount,
    percent: ResourceUtilizationPercent,
    status: ResourceUtilizationStatus,
    used: ResourceAmount,
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
    if maximum.0 == 0u64 {
        return Err(ResourceUtilizationError::ZeroMaximum);
    }
    let percent_u128 = u128::from(used.0)
        .saturating_mul(100u128)
        .div_euclid(u128::from(maximum.0))
        .min(100u128);
    let percent = match u8::try_from(percent_u128) {
        Ok(value) => ResourceUtilizationPercent::from(value),
        Err(_error) => ResourceUtilizationPercent::from(100u8),
    };
    let status = match percent.0 {
        REJECT_NON_ESSENTIAL_WRITES_PERCENT..=u8::MAX => {
            ResourceUtilizationStatus::RejectNonEssentialWrites
        }
        CRITICAL_PERCENT..REJECT_NON_ESSENTIAL_WRITES_PERCENT => {
            ResourceUtilizationStatus::Critical
        }
        WARNING_PERCENT..CRITICAL_PERCENT => ResourceUtilizationStatus::Warning,
        0u8..WARNING_PERCENT => ResourceUtilizationStatus::Ok,
    };
    Ok(ResourceUtilization {
        maximum,
        percent,
        status,
        used,
    })
}

#[cfg(test)]
mod tests {
    fn calculate(used: u64, maximum: u64) -> super::ResourceUtilization {
        super::calculate_resource_utilization(
            super::ResourceAmount::from(used),
            super::ResourceAmount::from(maximum),
        )
        .expect("8c23bc92")
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
    }

    #[test]
    fn rejects_zero_maximum() {
        assert_eq!(
            super::calculate_resource_utilization(
                super::ResourceAmount::from(0u64),
                super::ResourceAmount::from(0u64),
            ),
            Err(super::ResourceUtilizationError::ZeroMaximum)
        );
    }
}
