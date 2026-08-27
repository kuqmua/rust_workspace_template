#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::AsRefStr, newtype::FromInner,
)]
pub(crate) struct ToSnakeCaseInput<'input_lt>(&'input_lt str);
