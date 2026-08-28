use super::{SynExpr, SynType};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[allow(clippy::field_scoped_visibility_modifiers)] // the proc-macro entry module consumes this parsed domain model
#[derive(generate_accessor::Getters)]
pub(crate) struct NewtypeTryFromAttrs {
    error: Option<SynType>,
    validator: SynExpr,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl NewtypeTryFromAttrs {
    #[allow(clippy::single_call_fn)] // construction remains inside the field owner module
    pub(crate) const fn new(error: Option<SynType>, validator: SynExpr) -> Self {
        Self { error, validator }
    }
}
