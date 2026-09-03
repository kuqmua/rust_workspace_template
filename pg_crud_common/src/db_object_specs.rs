#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    proc_macro_newtype_deref_target::DerefTarget,
    proc_macro_newtype_deref_mut_target::DerefMutTarget,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct DbObjectSpecs(Vec<crate::db_object_spec::DbObjectSpec>);
