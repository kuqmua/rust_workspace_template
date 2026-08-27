#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    std::hash::Hash,
    newtype::AsRefInner,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct StdAdminStrRef<'value_lt>(&'value_lt str);
