#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_newtype::AsRefStr,
    proc_macro_newtype::FromInner,
)]
pub(super) struct TextRef<'text_lt>(&'text_lt str);
