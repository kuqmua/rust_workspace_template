#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub struct FormValueRef<'value_lt>(&'value_lt str);
