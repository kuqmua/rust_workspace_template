#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct SynParsedGeneratePgTableInput(pub(super) syn::DeriveInput);
