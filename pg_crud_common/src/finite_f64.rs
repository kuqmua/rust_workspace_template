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

    fn try_from(f64: f64) -> Result<Self, Self::Error> {
        if !f64.is_finite() {
            return Err(crate::finite_f64_error::FiniteF64Error::NotFinite);
        }
        Ok(Self(f64))
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
}
