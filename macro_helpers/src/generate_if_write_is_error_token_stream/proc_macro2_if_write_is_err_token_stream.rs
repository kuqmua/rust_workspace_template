#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, newtype::FromInner, newtype::ToTokens,
)]
pub struct ProcMacro2IfWriteIsErrTokenStream(proc_macro2::TokenStream);
