#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
pub(crate) struct PanicFile<'file_lt>(&'file_lt str);

impl<'file_lt> PanicFile<'file_lt> {
    pub(crate) const fn get(self) -> &'file_lt str {
        self.0
    }
}
