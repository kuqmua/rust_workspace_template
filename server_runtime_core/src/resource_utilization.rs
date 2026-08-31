#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    generate_constructor::New,
)]
#[constructor(pub(crate))]
pub struct ResourceUtilization {
    maximum: crate::resource_amount::ResourceAmount,
    used: crate::resource_amount::ResourceAmount,
    percent: crate::resource_utilization_percent::ResourceUtilizationPercent,
    status: crate::resource_utilization_status::ResourceUtilizationStatus,
}

impl ResourceUtilization {
    #[must_use]
    pub const fn maximum(self) -> crate::resource_amount::ResourceAmount {
        self.maximum
    }

    #[must_use]
    pub const fn percent(self) -> crate::resource_utilization_percent::ResourceUtilizationPercent {
        self.percent
    }

    #[must_use]
    pub const fn status(self) -> crate::resource_utilization_status::ResourceUtilizationStatus {
        self.status
    }

    #[must_use]
    pub const fn used(self) -> crate::resource_amount::ResourceAmount {
        self.used
    }
}
#[cfg(test)]
mod tests {
    fn calculate(used: u64, maximum: u64) -> super::ResourceUtilization {
        crate::calculate_resource_utilization::calculate_resource_utilization(
            crate::resource_amount::ResourceAmount::from(used),
            crate::resource_amount::ResourceAmount::from(maximum),
        )
        .expect("8c23bc92 calculate invariant must hold")
    }

    #[test]
    fn classifies_every_threshold_boundary() {
        assert_eq!(
            calculate(69u64, 100u64).status(),
            crate::resource_utilization_status::ResourceUtilizationStatus::Ok
        );
        assert_eq!(
            calculate(70u64, 100u64).status(),
            crate::resource_utilization_status::ResourceUtilizationStatus::Warning
        );
        assert_eq!(
            calculate(85u64, 100u64).status(),
            crate::resource_utilization_status::ResourceUtilizationStatus::Critical
        );
        assert_eq!(
            calculate(95u64, 100u64).status(),
            crate::resource_utilization_status::ResourceUtilizationStatus::RejectNonEssentialWrites
        );
    }

    #[test]
    fn caps_over_capacity_percent_without_overflow() {
        let utilization = calculate(u64::MAX, 1u64);
        assert_eq!(utilization.percent().get(), 100u8);
        assert_eq!(
            utilization.status(),
            crate::resource_utilization_status::ResourceUtilizationStatus::RejectNonEssentialWrites
        );
        assert_eq!(
            utilization.used(),
            crate::resource_amount::ResourceAmount::from(u64::MAX)
        );
        assert_eq!(
            utilization.maximum(),
            crate::resource_amount::ResourceAmount::from(1u64)
        );
    }

    #[test]
    fn percentage_uses_integer_floor_and_zero_usage_is_ok() {
        let zero = calculate(constants_u64::ZERO, u64::MAX);
        assert_eq!(zero.percent().get(), constants_u8::ZERO);
        assert_eq!(
            zero.status(),
            crate::resource_utilization_status::ResourceUtilizationStatus::Ok
        );
        let rounded_down = calculate(699u64, 1000u64);
        assert_eq!(rounded_down.percent().get(), 69u8);
        assert_eq!(
            rounded_down.status(),
            crate::resource_utilization_status::ResourceUtilizationStatus::Ok
        );
    }

    #[test]
    fn rejects_zero_maximum() {
        assert_eq!(
            crate::calculate_resource_utilization::calculate_resource_utilization(
                crate::resource_amount::ResourceAmount::from(constants_u64::ZERO),
                crate::resource_amount::ResourceAmount::from(constants_u64::ZERO),
            ),
            Err(crate::resource_utilization_error::ResourceUtilizationError::ZeroMaximum)
        );
    }
    #[test]
    fn percentage_rejects_values_above_one_hundred() {
        let _error =
            crate::resource_utilization_percent::ResourceUtilizationPercent::try_from(101u8)
                .expect_err(constants_str::VALUE_F7C27C6F);
        assert_eq!(
            crate::resource_utilization_percent::ResourceUtilizationPercent::try_from(100u8)
                .expect("f17abeab percentage_rejects_values_above_one_hundred invariant must hold")
                .get(),
            100u8
        );
    }
}
