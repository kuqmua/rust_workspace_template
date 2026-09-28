#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Eq,
    PartialEq,
    proc_macro_new::New,
)]
pub struct SqlSelectBuilder {
    #[constructor(order = 1)]
    columns: crate::sql_identifiers::SqlIdentifiers,
    #[constructor(order = 0)]
    table: crate::sql_qualified_identifier::SqlQualifiedIdentifier,
}

impl SqlSelectBuilder {
    pub fn build(
        &self,
    ) -> Result<
        crate::query_part_fragment::QueryPartFragment,
        crate::pg_crud_string_wrapper_try_from_string_error::PgCrudStringWrapperTryFromStringError,
    > {
        let fixed_len = constants_str::SELECT
            .len()
            .saturating_add(constants_str::FROM.len())
            .saturating_add(self.table.get_schema().as_ref().len())
            .saturating_add(constants_str::DOT.len())
            .saturating_add(self.table.get_table().as_ref().len());
        let columns = self.columns.get_inner().get_inner().as_str();
        let capacity = fixed_len.saturating_add(columns.len());
        let mut query = String::with_capacity(capacity);
        query.push_str(constants_str::SELECT);
        query.push_str(columns);
        query.push_str(constants_str::FROM);
        query.push_str(self.table.get_schema().as_ref());
        query.push('.');
        query.push_str(self.table.get_table().as_ref());
        crate::query_part_fragment::QueryPartFragment::try_from(query)
    }
}
