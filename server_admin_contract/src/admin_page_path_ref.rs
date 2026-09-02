#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype::FromInner,
)]
pub struct AdminPagePathRef<'path_lt>(&'path_lt str);
impl<'path_lt> AdminPagePathRef<'path_lt> {
    pub(crate) const fn get(self) -> &'path_lt str {
        self.0
    }
}
