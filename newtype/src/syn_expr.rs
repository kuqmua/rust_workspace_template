#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype_foundation::AsRefInner,
    newtype_foundation::FromInner,
    newtype_foundation::ToTokens,
)]
pub(crate) struct SynExpr(syn::Expr);
