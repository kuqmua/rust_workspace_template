#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, thiserror::Error,
)]
pub enum FiniteF64Error {
    #[error("floating-point value must be finite")]
    NotFinite,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    newtype::IntoInnerFrom,
)]
pub struct FiniteF64(f64);

impl TryFrom<f64> for FiniteF64 {
    type Error = FiniteF64Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(FiniteF64Error::NotFinite);
        }
        Ok(Self(value))
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, thiserror::Error,
)]
pub enum PositiveFiniteF64Error {
    #[error("floating-point value must be finite")]
    NotFinite,
    #[error("floating-point value must be greater than zero")]
    NotPositive,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    newtype::IntoInnerFrom,
)]
pub struct PositiveFiniteF64(f64);

impl TryFrom<f64> for PositiveFiniteF64 {
    type Error = PositiveFiniteF64Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(PositiveFiniteF64Error::NotFinite);
        }
        if value <= 0.0f64 {
            return Err(PositiveFiniteF64Error::NotPositive);
        }
        Ok(Self(value))
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, PartialEq, thiserror::Error,
)]
pub enum UnitIntervalF64Error {
    #[error("floating-point value must be finite")]
    NotFinite,
    #[error("floating-point value must be within the inclusive unit interval")]
    OutOfRange,
}

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
    type Error = UnitIntervalF64Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(UnitIntervalF64Error::NotFinite);
        }
        if !(0.0f64..=1.0f64).contains(&value) {
            return Err(UnitIntervalF64Error::OutOfRange);
        }
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn finite_value_rejects_non_finite_values() {
        assert!(
            [f64::NAN, f64::INFINITY, f64::NEG_INFINITY]
                .into_iter()
                .all(|value| super::FiniteF64::try_from(value)
                    == Err(super::FiniteF64Error::NotFinite))
        );
        assert_eq!(
            super::FiniteF64::try_from(1.5f64).map(f64::from),
            Ok(1.5f64)
        );
    }

    #[test]
    fn positive_value_requires_finite_value_greater_than_zero() {
        assert_eq!(
            super::PositiveFiniteF64::try_from(0.0f64),
            Err(super::PositiveFiniteF64Error::NotPositive)
        );
        assert_eq!(
            super::PositiveFiniteF64::try_from(-1.0f64),
            Err(super::PositiveFiniteF64Error::NotPositive)
        );
        assert_eq!(
            super::PositiveFiniteF64::try_from(f64::NAN),
            Err(super::PositiveFiniteF64Error::NotFinite)
        );
    }

    #[test]
    fn unit_interval_includes_both_boundaries() {
        assert!(
            [0.0f64, 0.5f64, 1.0f64].into_iter().all(|value| {
                super::UnitIntervalF64::try_from(value).map(f64::from) == Ok(value)
            })
        );
        assert_eq!(
            super::UnitIntervalF64::try_from(1.1f64),
            Err(super::UnitIntervalF64Error::OutOfRange)
        );
        assert_eq!(
            super::UnitIntervalF64::try_from(f64::INFINITY),
            Err(super::UnitIntervalF64Error::NotFinite)
        );
    }
}
