#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
pub struct SqlQualifiedIdentifier {
    schema: crate::sql_identifier::SqlIdentifier,
    table: crate::sql_identifier::SqlIdentifier,
}
impl std::fmt::Display for SqlQualifiedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.schema.as_ref())?;
        f.write_str(constants_str::DOT)?;
        f.write_str(self.table.as_ref())
    }
}
