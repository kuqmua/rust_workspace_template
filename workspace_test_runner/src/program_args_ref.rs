#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
pub(crate) struct ProgramArgsRef<'lt>(&'lt [&'lt str]);
impl<'lt, const N: usize> From<&'lt [&'lt str; N]> for ProgramArgsRef<'lt> {
    fn from(value: &'lt [&'lt str; N]) -> Self {
        Self(value.as_slice())
    }
}
