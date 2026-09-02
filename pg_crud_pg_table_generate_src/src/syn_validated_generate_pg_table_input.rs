#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, proc_macro_newtype::FromInner,
)]
pub struct SynValidatedGeneratePgTableInput(crate::generate_pg_table_model::GeneratePgTableModel);

impl SynValidatedGeneratePgTableInput {
    pub(crate) fn into_model(self) -> crate::generate_pg_table_model::GeneratePgTableModel {
        self.0
    }
}
