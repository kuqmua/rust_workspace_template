#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    proc_macro_newtype_deref_target::DerefTarget,
    proc_macro_newtype_deref_mut_target::DerefMutTarget,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct DbColumnContractSnapshots(
    Vec<crate::db_column_contract_snapshot::DbColumnContractSnapshot>,
);
