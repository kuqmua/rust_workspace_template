#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    newtype::IntoInnerFrom,
)]
pub struct UnitIntervalF64(f64);

impl TryFrom<f64> for UnitIntervalF64 {
    type Error = crate::unit_interval_f64_error::UnitIntervalF64Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(crate::unit_interval_f64_error::UnitIntervalF64Error::NotFinite);
        }
        if !(0.0f64..=1.0f64).contains(&value) {
            return Err(crate::unit_interval_f64_error::UnitIntervalF64Error::OutOfRange);
        }
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unit_interval_includes_both_boundaries() {
        assert!([0.0f64, 0.5f64, 1.0f64].into_iter().all(|value| {
            crate::unit_interval_f64::UnitIntervalF64::try_from(value).map(f64::from) == Ok(value)
        }));
        assert_eq!(
            crate::unit_interval_f64::UnitIntervalF64::try_from(1.1f64),
            Err(crate::unit_interval_f64_error::UnitIntervalF64Error::OutOfRange)
        );
        assert_eq!(
            crate::unit_interval_f64::UnitIntervalF64::try_from(f64::INFINITY),
            Err(crate::unit_interval_f64_error::UnitIntervalF64Error::NotFinite)
        );
    }
}
