#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner, newtype::GetInner,
)]
pub(crate) struct CargoArgsRef<'args_lt>(&'args_lt [&'args_lt str]);
