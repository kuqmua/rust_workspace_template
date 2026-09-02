#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
)]
pub(crate) struct ProcMacro2VariantMatchingTokensRef<'tokens_lt>(
    &'tokens_lt [proc_macro2::TokenStream],
);
