#[path = "resource_utilization/calculate_resource_utilization.rs"]
mod calculate_resource_utilization;
#[path = "resource_utilization/critical_percent.rs"]
mod critical_percent;
#[path = "resource_utilization/reject_non_essential_writes_percent.rs"]
mod reject_non_essential_writes_percent;
#[path = "resource_utilization/resource_amount.rs"]
mod resource_amount;
#[path = "resource_utilization/resource_utilization.rs"]
mod resource_utilization;
#[path = "resource_utilization/resource_utilization_error.rs"]
mod resource_utilization_error;
#[path = "resource_utilization/resource_utilization_known_percent.rs"]
mod resource_utilization_known_percent;
#[path = "resource_utilization/resource_utilization_percent.rs"]
mod resource_utilization_percent;
#[path = "resource_utilization/resource_utilization_percent_try_from_u8_error.rs"]
mod resource_utilization_percent_try_from_u8_error;
#[path = "resource_utilization/resource_utilization_status.rs"]
mod resource_utilization_status;
#[path = "resource_utilization/warning_percent.rs"]
mod warning_percent;

pub use calculate_resource_utilization::calculate_resource_utilization;
use critical_percent::CRITICAL_PERCENT;
use reject_non_essential_writes_percent::REJECT_NON_ESSENTIAL_WRITES_PERCENT;
pub use resource_amount::ResourceAmount;
pub use resource_utilization::ResourceUtilization;
pub use resource_utilization_error::ResourceUtilizationError;
use resource_utilization_known_percent::ResourceUtilizationKnownPercent;
pub use resource_utilization_percent::ResourceUtilizationPercent;
pub use resource_utilization_percent_try_from_u8_error::ResourceUtilizationPercentTryFromU8Error;
pub use resource_utilization_status::ResourceUtilizationStatus;
use warning_percent::WARNING_PERCENT;

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
