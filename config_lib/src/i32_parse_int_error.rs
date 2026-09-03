#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_debug_transparent::DebugTransparent,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct I32ParseIntError(std::num::ParseIntError);
