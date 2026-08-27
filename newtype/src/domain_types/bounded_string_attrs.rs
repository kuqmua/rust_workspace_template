use super::{BoundedStringOption, SynExpr};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
#[allow(clippy::field_scoped_visibility_modifiers)] // the proc-macro entry module incrementally builds this parsed domain model
pub(crate) struct BoundedStringAttrs {
    pub(crate) description: Option<SynExpr>,
    pub(crate) max: Option<SynExpr>,
    pub(crate) min: Option<SynExpr>,
    pub(crate) options:
        workspace_macro_helpers::domain_types::UniqueOptionBTreeSet<BoundedStringOption>,
    pub(crate) validator: Option<SynExpr>,
}
