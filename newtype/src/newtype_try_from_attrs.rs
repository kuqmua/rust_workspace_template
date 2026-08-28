use super::{SynExpr, SynType};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[allow(clippy::field_scoped_visibility_modifiers)] // the proc-macro entry module consumes this parsed domain model
#[derive(generate_accessor::Getters, generate_constructor::New)]
pub(crate) struct NewtypeTryFromAttrs {
    error: Option<SynType>,
    validator: SynExpr,
}
