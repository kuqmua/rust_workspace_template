#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct EnvVarValueRef<'value_lt>(pub(super) &'value_lt str);
