#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_as_ref_target::AsRefTarget,
)]
pub(crate) struct StdFrontendOsString(std::ffi::OsString);
