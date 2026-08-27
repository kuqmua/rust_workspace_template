#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct SynBuiltGeneratePgTableInput(
    pub(super) crate::domain_types::table::GeneratePgTableModel,
);

impl SynBuiltGeneratePgTableInput {
    #[must_use]
    pub const fn model(&self) -> &crate::domain_types::table::GeneratePgTableModel {
        &self.0
    }
}
