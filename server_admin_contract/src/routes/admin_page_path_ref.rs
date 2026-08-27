#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub struct AdminPagePathRef<'path_lt>(pub(super) &'path_lt str);
impl<'path_lt> AdminPagePathRef<'path_lt> {
    pub(crate) const fn get(self) -> &'path_lt str {
        self.0
    }
}
