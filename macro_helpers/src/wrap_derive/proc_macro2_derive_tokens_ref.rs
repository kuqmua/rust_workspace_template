#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub struct ProcMacro2DeriveTokensRef<'tokens_lt>(
    pub(super) &'tokens_lt [&'tokens_lt proc_macro2::TokenStream],
);
