#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    PartialEq,
    Eq,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
)]
pub(crate) struct RsFilePathBuf(std::path::PathBuf);
