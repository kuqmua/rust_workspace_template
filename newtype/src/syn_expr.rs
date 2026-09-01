#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype_foundation::FromInner)]
pub(crate) struct SynExpr(syn::Expr);
impl AsRef<syn::Expr> for SynExpr {
    fn as_ref(&self) -> &syn::Expr {
        &self.0
    }
}
impl quote::ToTokens for SynExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
