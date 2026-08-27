#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner, newtype::AsRefOwned)]
pub(crate) struct SynExpr(syn::Expr);
impl SynExpr {
    pub(crate) fn into_inner(self) -> syn::Expr {
        self.0
    }
}
