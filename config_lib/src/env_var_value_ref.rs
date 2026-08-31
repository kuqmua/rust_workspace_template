#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefInner,
    newtype::FromInner,
)]
pub(super) struct EnvVarValueRef<'value_lt>(&'value_lt str);
