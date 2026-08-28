use super::SynIdent;

#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
pub(crate) struct PageCatalogArgs {
    inventory: SynIdent,
    path_ref: SynIdent,
    spec: SynIdent,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl PageCatalogArgs {
    #[allow(
        clippy::single_call_fn,
        reason = "constructor mirrors the parsed field model"
    )]
    pub(crate) const fn new(inventory: SynIdent, path_ref: SynIdent, spec: SynIdent) -> Self {
        Self {
            inventory,
            path_ref,
            spec,
        }
    }
}
