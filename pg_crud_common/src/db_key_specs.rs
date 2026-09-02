#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::DerefMutTarget,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub struct DbKeySpecs(Vec<crate::db_key_spec::DbKeySpec>);
