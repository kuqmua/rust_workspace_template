#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::FromInner,
)]
pub struct RuntimePathRef<'path_lt>(&'path_lt std::path::Path);

impl<'path_lt> RuntimePathRef<'path_lt> {
    pub(crate) const fn get(self) -> &'path_lt std::path::Path {
        self.0
    }
}
