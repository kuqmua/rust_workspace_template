#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct ProgramArgsRef<'lt>(&'lt [&'lt str]);
impl<'lt, const N: usize> From<&'lt [&'lt str; N]> for ProgramArgsRef<'lt> {
    fn from(value: &'lt [&'lt str; N]) -> Self {
        Self(value.as_slice())
    }
}
impl<'lt> ProgramArgsRef<'lt> {
    pub(crate) const fn get(self) -> &'lt [&'lt str] {
        self.0
    }
}
