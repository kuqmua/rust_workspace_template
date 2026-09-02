#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::AsRefOwned,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
pub struct ProcessOutput(std::process::Output);
