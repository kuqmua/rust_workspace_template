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
    type Error = crate::positive_finite_f64_error::PositiveFiniteF64Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(crate::positive_finite_f64_error::PositiveFiniteF64Error::NotFinite);
        }
        if value <= 0.0f64 {
            return Err(crate::positive_finite_f64_error::PositiveFiniteF64Error::NotPositive);
        }
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_positive_value_requires_finite_value_greater_than_zero() {
        assert_eq!(
            crate::positive_finite_f64::PositiveFiniteF64::try_from(0.0f64),
            Err(crate::positive_finite_f64_error::PositiveFiniteF64Error::NotPositive)
        );
        assert_eq!(
            crate::positive_finite_f64::PositiveFiniteF64::try_from(-1.0f64),
            Err(crate::positive_finite_f64_error::PositiveFiniteF64Error::NotPositive)
        );
        assert_eq!(
            crate::positive_finite_f64::PositiveFiniteF64::try_from(f64::NAN),
            Err(crate::positive_finite_f64_error::PositiveFiniteF64Error::NotFinite)
        );
    }
}
