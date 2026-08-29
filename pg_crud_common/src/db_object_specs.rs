#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    newtype::DerefTarget,
    newtype::DerefMutTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct DbObjectSpecs(Vec<crate::db_object_spec::DbObjectSpec>);
