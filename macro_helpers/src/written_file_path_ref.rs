#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefInner,
    proc_macro_newtype::FromInner,
)]
pub struct WrittenFilePathRef<'path_lt>(&'path_lt std::path::Path);
