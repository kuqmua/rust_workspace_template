#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub(crate) struct ProcMacro2VariantMatchingTokensRef<'tokens_lt>(
    &'tokens_lt [proc_macro2::TokenStream],
);
