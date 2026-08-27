#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub struct StdEnvVarOkRef<'value_lt>(pub(super) &'value_lt str);
