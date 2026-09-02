#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
pub struct AdminPermissionStrRef<'value_lt>(&'value_lt str);
