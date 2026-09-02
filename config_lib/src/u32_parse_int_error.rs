#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DebugTransparent,
    proc_macro_newtype::FromInner,
)]
pub struct U32ParseIntError(std::num::ParseIntError);
