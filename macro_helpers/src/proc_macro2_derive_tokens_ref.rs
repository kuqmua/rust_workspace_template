#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub struct ProcMacro2DeriveTokensRef<'tokens_lt>(
    &'tokens_lt [&'tokens_lt proc_macro2::TokenStream],
);

impl<'tokens_lt> ProcMacro2DeriveTokensRef<'tokens_lt> {
    pub(crate) const fn tokens(self) -> &'tokens_lt [&'tokens_lt proc_macro2::TokenStream] {
        self.0
    }
}
