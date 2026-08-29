#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner, newtype::GetInner,
)]
pub(crate) struct CargoArgs(&'static [&'static str]);
impl<const N: usize> From<&'static [&'static str; N]> for CargoArgs {
    fn from(value: &'static [&'static str; N]) -> Self {
        Self(value.as_slice())
    }
}
