#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub struct AdminTableSortKeyRef<'value_lt>(&'value_lt str);
