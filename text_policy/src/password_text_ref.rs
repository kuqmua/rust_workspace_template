#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct PasswordTextRef<'value_lt>(&'value_lt str);
impl std::fmt::Debug for PasswordTextRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(constants_str::catalog::REDACTED_ALT_3)
    }
}
