#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::DerefMutTarget,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub struct DbObjectSnapshots(Vec<crate::db_object_snapshot::DbObjectSnapshot>);
