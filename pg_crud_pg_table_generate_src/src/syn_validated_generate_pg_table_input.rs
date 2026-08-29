#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct SynValidatedGeneratePgTableInput(
    pub(super) crate::generate_pg_table_model::GeneratePgTableModel,
);

impl SynValidatedGeneratePgTableInput {
    pub(crate) fn into_model(self) -> crate::generate_pg_table_model::GeneratePgTableModel {
        self.0
    }
}
