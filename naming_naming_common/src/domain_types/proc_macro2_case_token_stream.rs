#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, newtype::FromInner)]
pub(super) struct ProcMacro2CaseTokenStream(pub(super) proc_macro2::TokenStream);
