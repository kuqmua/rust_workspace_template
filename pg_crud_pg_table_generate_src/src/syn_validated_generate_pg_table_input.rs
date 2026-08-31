#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct SynValidatedGeneratePgTableInput(crate::generate_pg_table_model::GeneratePgTableModel);

impl SynValidatedGeneratePgTableInput {
    pub(crate) fn into_model(self) -> crate::generate_pg_table_model::GeneratePgTableModel {
        self.0
    }
}
