#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
    proc_macro_newtype_from_inner::FromInner,
)]
pub struct SynLocationField(syn::Field);
