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
