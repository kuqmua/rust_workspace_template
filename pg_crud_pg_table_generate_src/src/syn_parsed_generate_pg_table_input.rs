#[derive(
    generate_accessor::Getters,
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct SynParsedGeneratePgTableInput(syn::DeriveInput);
