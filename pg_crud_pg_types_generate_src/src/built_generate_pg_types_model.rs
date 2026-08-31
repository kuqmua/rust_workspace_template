#[derive(
    generate_accessor::Getters,
    generate_constructor::New,
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
)]
pub struct BuiltGeneratePgTypesModel {
    config: crate::generate_pg_types_config::GeneratePgTypesConfig,
    entry_count: crate::pg_types_model_entry_count::PgTypesModelEntryCount,
}
impl BuiltGeneratePgTypesModel {
    pub(super) fn into_parts(
        self,
    ) -> (
        crate::generate_pg_types_config::GeneratePgTypesConfig,
        crate::pg_types_model_entry_count::PgTypesModelEntryCount,
    ) {
        (self.config, self.entry_count)
    }
}
