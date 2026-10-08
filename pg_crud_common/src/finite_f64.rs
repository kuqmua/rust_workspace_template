#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct FiniteF64(f64);

impl TryFrom<f64> for FiniteF64 {
    type Error = crate::finite_f64_error::FiniteF64Error;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !value.is_finite() {
            return Err(crate::finite_f64_error::FiniteF64Error::NotFinite);
        }
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_finite_value_rejects_non_finite_values() {
        assert!(
            [f64::NAN, f64::INFINITY, f64::NEG_INFINITY]
                .into_iter()
                .all(|value| crate::finite_f64::FiniteF64::try_from(value)
                    == Err(crate::finite_f64_error::FiniteF64Error::NotFinite))
        );
        assert_eq!(
            crate::finite_f64::FiniteF64::try_from(1.5f64).map(f64::from),
            Ok(1.5f64)
        );
    }

    #[test]
    fn test_finite_values_preserve_extremes_signed_zero_and_subnormal_bits() {
        assert!(
            [
                f64::MIN,
                f64::MAX,
                f64::MIN_POSITIVE,
                -f64::MIN_POSITIVE,
                0.0f64,
                -0.0f64,
                f64::from_bits(1u64),
                -f64::from_bits(1u64),
            ]
            .into_iter()
            .all(|value| {
                crate::finite_f64::FiniteF64::try_from(value)
                    .is_ok_and(|finite_f64| f64::from(finite_f64).to_bits() == value.to_bits())
            })
        );
    }
}
