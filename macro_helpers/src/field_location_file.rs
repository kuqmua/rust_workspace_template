#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[must_use]
#[derive(Debug, Clone, Copy, newtype::FromInner)]
pub struct FieldLocationFile(&'static str);

impl FieldLocationFile {
    pub(crate) const fn as_str(self) -> &'static str {
        self.0
    }
}
