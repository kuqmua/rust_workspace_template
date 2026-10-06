#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
pub(crate) struct CargoArgs(&'static [&'static str]);
impl<const N: usize> From<&'static [&'static str; N]> for CargoArgs {
    fn from(value: &'static [&'static str; N]) -> Self {
        Self(value.as_slice())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cargo_argument_array_conversion_preserves_order_duplicates_and_borrowing() {
        let value = &[
            constants_str::CHECK,
            constants_str::CHECK,
            constants_str::STATIC,
        ];
        let cargo_args = crate::cargo_args::CargoArgs::from(value);
        assert_eq!(cargo_args.get(), value.as_slice());
        assert!(std::ptr::eq(cargo_args.get(), value.as_slice()));
        assert!(crate::cargo_args::CargoArgs::from(&[]).get().is_empty());
    }
}
