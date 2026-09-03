#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype_as_ref_inner::AsRefInner,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct ProcessProgram(&'static str);
