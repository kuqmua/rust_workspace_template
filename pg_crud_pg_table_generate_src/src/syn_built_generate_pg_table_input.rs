#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, proc_macro_newtype::FromInner,
)]
pub struct SynBuiltGeneratePgTableInput(crate::generate_pg_table_model::GeneratePgTableModel);

impl SynBuiltGeneratePgTableInput {
    #[must_use]
    pub const fn model(&self) -> &crate::generate_pg_table_model::GeneratePgTableModel {
        &self.0
    }

    pub(crate) fn into_model(self) -> crate::generate_pg_table_model::GeneratePgTableModel {
        self.0
    }
}
