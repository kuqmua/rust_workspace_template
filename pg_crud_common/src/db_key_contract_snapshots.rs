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
pub struct DbKeyContractSnapshots(Vec<crate::db_key_contract_snapshot::DbKeyContractSnapshot>);
