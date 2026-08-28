use super::{SynExpr, SynIdent};

#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct RouteCatalogArgs {
    body_limit: SynExpr,
    family: SynIdent,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl RouteCatalogArgs {
    #[allow(
        clippy::single_call_fn,
        reason = "constructor mirrors the parsed field model"
    )]
    pub(crate) const fn new(body_limit: SynExpr, family: SynIdent) -> Self {
        Self { body_limit, family }
    }
}
