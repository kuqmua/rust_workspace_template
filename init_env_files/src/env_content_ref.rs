#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::AsRefStr, newtype::FromInner,
)]
pub(crate) struct EnvContentRef<'content_lt>(&'content_lt str);
