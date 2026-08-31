#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::AsRefStr,
    newtype::FromInner,
)]
pub(super) struct TextRef<'text_lt>(&'text_lt str);
