use super::{SynExpr, SynType};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[allow(clippy::field_scoped_visibility_modifiers)] // the proc-macro entry module consumes this parsed domain model
pub(crate) struct NewtypeTryFromAttrs {
    pub(crate) error: Option<SynType>,
    pub(crate) validator: SynExpr,
}
