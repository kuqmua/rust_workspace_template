#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::IntoInnerFrom,
    proc_macro_newtype::FromInner,
)]
pub struct SynLocationField(syn::Field);
