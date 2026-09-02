#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
pub(super) struct OpenApiSpecificationPath(&'static str);
