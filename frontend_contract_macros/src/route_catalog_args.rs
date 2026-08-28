use super::{SynExpr, SynIdent};

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    generate_accessor::Getters,
    generate_constructor::New,
)]
#[getters(get_mut)]
pub(crate) struct RouteCatalogArgs {
    body_limit: SynExpr,
    family: SynIdent,
}
