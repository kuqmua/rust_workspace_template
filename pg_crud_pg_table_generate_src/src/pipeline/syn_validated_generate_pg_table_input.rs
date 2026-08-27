#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct SynValidatedGeneratePgTableInput(
    pub(super) crate::domain_types::table::GeneratePgTableModel,
);

impl SynValidatedGeneratePgTableInput {
    pub(crate) fn into_model(self) -> crate::domain_types::table::GeneratePgTableModel {
        self.0
    }
}
