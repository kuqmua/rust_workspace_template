#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
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
    fn test_positive_finite_float_preserves_extreme_values_and_classifies_rejections() {
        assert!(
            [
                f64::from_bits(1u64),
                f64::MIN_POSITIVE,
                f64::EPSILON,
                1.0f64,
                f64::MAX
            ]
            .into_iter()
            .all(|value| {
                crate::positive_finite_f64::PositiveFiniteF64::try_from(value).is_ok_and(
                    |positive_finite_f64| {
                        f64::from(positive_finite_f64).to_bits() == value.to_bits()
                    },
                )
            })
        );
        assert!(
            [
                0.0f64,
                -0.0f64,
                -f64::from_bits(1u64),
                -f64::MIN_POSITIVE,
                f64::MIN
            ]
            .into_iter()
            .all(|value| {
                crate::positive_finite_f64::PositiveFiniteF64::try_from(value)
                    == Err(crate::positive_finite_f64_error::PositiveFiniteF64Error::NotPositive)
            })
        );
        assert!(
            [
                f64::INFINITY,
                f64::NEG_INFINITY,
                f64::NAN,
                f64::from_bits(0x7ff0_0000_0000_0001u64),
                f64::from_bits(0xfff8_0000_0000_0001u64)
            ]
            .into_iter()
            .all(|value| {
                crate::positive_finite_f64::PositiveFiniteF64::try_from(value)
                    == Err(crate::positive_finite_f64_error::PositiveFiniteF64Error::NotFinite)
            })
        );
    }

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
