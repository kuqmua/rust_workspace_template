#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_display::Display,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
pub struct QueryPartIncrement(u64);
impl crate::query_part_increment_mut::QueryPartIncrementMut for QueryPartIncrement {
    fn checked_add_one(&mut self) -> Option<QueryPartIncrement> {
        self.get().checked_add(1).map(|value| {
            *self = Self::from(value);
            Self::from(value)
        })
    }
}
