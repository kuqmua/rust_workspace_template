#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    generate_accessor::Getters,
    generate_constructor::New,
)]
pub struct SqlQualifiedIdentifier {
    schema: crate::sql_identifier::SqlIdentifier,
    table: crate::sql_identifier::SqlIdentifier,
}
impl std::fmt::Display for SqlQualifiedIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.schema.as_ref())?;
        f.write_str(constants_str::catalog::DOT)?;
        f.write_str(self.table.as_ref())
    }
}
