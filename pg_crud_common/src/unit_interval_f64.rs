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
    type Error = crate::domain_types::UnitIntervalF64Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(crate::domain_types::UnitIntervalF64Error::NotFinite);
        }
        if !(0.0f64..=1.0f64).contains(&value) {
            return Err(crate::domain_types::UnitIntervalF64Error::OutOfRange);
        }
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn unit_interval_includes_both_boundaries() {
        assert!(
            [0.0f64, 0.5f64, 1.0f64].into_iter().all(|value| {
                super::UnitIntervalF64::try_from(value).map(f64::from) == Ok(value)
            })
        );
        assert_eq!(
            super::UnitIntervalF64::try_from(1.1f64),
            Err(crate::domain_types::UnitIntervalF64Error::OutOfRange)
        );
        assert_eq!(
            super::UnitIntervalF64::try_from(f64::INFINITY),
            Err(crate::domain_types::UnitIntervalF64Error::NotFinite)
        );
    }
}
