#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype_foundation::FromInner,
    newtype_foundation::GetInner,
)]
pub struct SynDeriveInputRef<'input_lt>(&'input_lt syn::DeriveInput);
