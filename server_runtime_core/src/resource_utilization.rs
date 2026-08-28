#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceUtilization {
    pub(super) maximum: ResourceAmount,
    pub(super) used: ResourceAmount,
    pub(super) percent: ResourceUtilizationPercent,
    pub(super) status: ResourceUtilizationStatus,
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

pub use crate::calculate_resource_utilization::calculate_resource_utilization;
use crate::critical_percent::CRITICAL_PERCENT;
use crate::reject_non_essential_writes_percent::REJECT_NON_ESSENTIAL_WRITES_PERCENT;
pub use crate::resource_amount::ResourceAmount;
pub use crate::resource_utilization_error::ResourceUtilizationError;
use crate::resource_utilization_known_percent::ResourceUtilizationKnownPercent;
pub use crate::resource_utilization_percent::ResourceUtilizationPercent;
pub use crate::resource_utilization_percent_try_from_u8_error::ResourceUtilizationPercentTryFromU8Error;
pub use crate::resource_utilization_status::ResourceUtilizationStatus;
use crate::warning_percent::WARNING_PERCENT;

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
        let zero = calculate(constants_u64::ZERO, u64::MAX);
        assert_eq!(zero.percent().get(), constants_u8::ZERO);
        assert_eq!(zero.status(), super::ResourceUtilizationStatus::Ok);
        let rounded_down = calculate(699u64, 1000u64);
        assert_eq!(rounded_down.percent().get(), 69u8);
        assert_eq!(rounded_down.status(), super::ResourceUtilizationStatus::Ok);
    }

    #[test]
    fn rejects_zero_maximum() {
        assert_eq!(
            super::calculate_resource_utilization(
                super::ResourceAmount::from(constants_u64::ZERO),
                super::ResourceAmount::from(constants_u64::ZERO),
            ),
            Err(super::ResourceUtilizationError::ZeroMaximum)
        );
    }
    #[test]
    fn percentage_rejects_values_above_one_hundred() {
        let _error = super::ResourceUtilizationPercent::try_from(101u8)
            .expect_err(constants_str::VALUE_F7C27C6F);
        assert_eq!(
            super::ResourceUtilizationPercent::try_from(100u8)
                .expect("f17abeab percentage_rejects_values_above_one_hundred invariant must hold")
                .get(),
            100u8
        );
    }
}
