#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
)]
#[constructor(pub(crate))]
pub struct ResourceUtilization {
    #[getters(copy)]
    maximum: crate::resource_amount::ResourceAmount,
    #[getters(copy)]
    used: crate::resource_amount::ResourceAmount,
    #[getters(copy)]
    percent: crate::resource_utilization_percent::ResourceUtilizationPercent,
    #[getters(copy)]
    status: crate::resource_utilization_status::ResourceUtilizationStatus,
}

#[cfg(test)]
mod tests {
    fn calculate(used: u64, maximum: u64) -> super::ResourceUtilization {
        crate::calculate_resource_utilization::calculate_resource_utilization(
            crate::resource_amount::ResourceAmount::from(used),
            crate::resource_amount::ResourceAmount::from(maximum),
        )
        .expect(constants_str::DIAGNOSTIC_8C23BC92)
    }

    #[test]
    fn test_classifies_every_threshold_boundary() {
        assert_eq!(
            calculate(69u64, 100u64).status(),
            crate::resource_utilization_status::ResourceUtilizationStatus::Ok
        );
        assert_eq!(
            calculate(70u64, 100u64).status(),
            crate::resource_utilization_status::ResourceUtilizationStatus::Warning
        );
        assert_eq!(
            calculate(84u64, 100u64).status(),
            crate::resource_utilization_status::ResourceUtilizationStatus::Warning
        );
        assert_eq!(
            calculate(85u64, 100u64).status(),
            crate::resource_utilization_status::ResourceUtilizationStatus::Critical
        );
        assert_eq!(
            calculate(94u64, 100u64).status(),
            crate::resource_utilization_status::ResourceUtilizationStatus::Critical
        );
        assert_eq!(
            calculate(95u64, 100u64).status(),
            crate::resource_utilization_status::ResourceUtilizationStatus::RejectNonEssentialWrites
        );
    }

    #[test]
    fn test_caps_over_capacity_percent_without_overflow() {
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
    fn test_percentage_uses_integer_floor_and_zero_usage_is_ok() {
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
    fn test_rejects_zero_maximum() {
        assert_eq!(
            crate::calculate_resource_utilization::calculate_resource_utilization(
                crate::resource_amount::ResourceAmount::from(constants_u64::ZERO),
                crate::resource_amount::ResourceAmount::from(constants_u64::ZERO),
            ),
            Err(crate::resource_utilization_error::ResourceUtilizationError::ZeroMaximum)
        );
    }
    #[test]
    fn test_percentage_rejects_values_above_one_hundred() {
        let _error =
            crate::resource_utilization_percent::ResourceUtilizationPercent::try_from(101u8)
                .expect_err(constants_str::VALUE_F7C27C6F);
        assert_eq!(
            crate::resource_utilization_percent::ResourceUtilizationPercent::try_from(100u8)
                .expect(constants_str::DIAGNOSTIC_F17ABEAB)
                .get(),
            100u8
        );
    }

    #[test]
    fn test_percentage_conversion_covers_all_byte_values_and_known_maximum() {
        assert!((u8::MIN..=100u8).all(|value| {
            crate::resource_utilization_percent::ResourceUtilizationPercent::try_from(value)
                .is_ok_and(|resource_utilization_percent| {
                    resource_utilization_percent.get() == value
                })
        }));
        assert!((101u8..=u8::MAX).all(|value| {
            crate::resource_utilization_percent::ResourceUtilizationPercent::try_from(value)
                .is_err_and(|error| error == crate::resource_utilization_percent_try_from_u8_error::ResourceUtilizationPercentTryFromU8Error::OutOfRange)
        }));
        assert_eq!(
            crate::resource_utilization_percent::ResourceUtilizationPercent::from(
                crate::resource_utilization_known_percent::ResourceUtilizationKnownPercent::Max
            )
            .get(),
            100u8
        );
    }

    #[test]
    fn test_resource_utilization_large_ratios_preserve_integer_floor_without_overflow() {
        assert!([
            (u64::MAX, u64::MAX, 100u8),
            (u64::MAX - 1u64, u64::MAX, 99u8),
            (9_223_372_036_854_775_807u64, u64::MAX, 49u8),
            (9_223_372_036_854_775_808u64, u64::MAX, 50u8),
            (184_467_440_737_095_516u64, u64::MAX, 0u8),
            (184_467_440_737_095_517u64, u64::MAX, 1u8),
            (u64::MAX, u64::MAX - 1u64, 100u8),
        ].into_iter().all(|(used, maximum, percent)| {
            let utilization = calculate(used, maximum);
            let status = if percent == 99u8 || percent == 100u8 {
                crate::resource_utilization_status::ResourceUtilizationStatus::RejectNonEssentialWrites
            } else {
                crate::resource_utilization_status::ResourceUtilizationStatus::Ok
            };
            utilization.percent().get() == percent
                && utilization.status() == status
                && utilization.used() == crate::resource_amount::ResourceAmount::from(used)
                && utilization.maximum() == crate::resource_amount::ResourceAmount::from(maximum)
        }));
    }
}
