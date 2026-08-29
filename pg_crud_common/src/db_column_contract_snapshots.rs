#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    newtype::DerefTarget,
    newtype::DerefMutTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DbColumnContractSnapshots(
    Vec<crate::db_column_contract_snapshot::DbColumnContractSnapshot>,
);
