#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct AdminTableSortKeyRef<'value_lt>(&'value_lt str);
