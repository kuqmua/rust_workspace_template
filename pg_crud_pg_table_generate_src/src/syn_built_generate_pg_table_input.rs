#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct SynBuiltGeneratePgTableInput(
    pub(super) crate::generate_pg_table_model::GeneratePgTableModel,
);

impl SynBuiltGeneratePgTableInput {
    #[must_use]
    pub const fn model(&self) -> &crate::generate_pg_table_model::GeneratePgTableModel {
        &self.0
    }
}
