#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_foundation::FromInner,
    proc_macro_newtype_foundation::GetInner,
)]
pub struct SynDeriveInputRef<'input_lt>(&'input_lt syn::DeriveInput);
