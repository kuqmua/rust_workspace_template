#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct SqlLikeInputRef<'value_lt>(&'value_lt str);

impl<'value_lt> SqlLikeInputRef<'value_lt> {
    pub(crate) const fn get(self) -> &'value_lt str {
        self.0
    }
}
