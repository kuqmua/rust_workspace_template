#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    PartialEq,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
    proc_macro_newtype::Display,
)]
#[accessor(pub(crate))]
pub struct MultipartValueLength(usize);
