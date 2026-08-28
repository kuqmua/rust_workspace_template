#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::AsRefInner,
    newtype::FromInner,
    newtype::GetInner,
)]
pub struct AdminDataColumnsCsvRef<'value_lt>(&'value_lt str);
