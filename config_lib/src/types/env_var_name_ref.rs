#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(super) struct EnvVarNameRef<'name_lt>(pub(super) &'name_lt str);
