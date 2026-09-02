#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
)]
pub struct ProcessStaticArgument(&'static str);
impl ProcessStaticArgument {
    pub(super) const fn get(&self) -> &'static str {
        self.0
    }
}
