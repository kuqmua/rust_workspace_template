#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct SqlSelectBuilder {
    columns: crate::sql_identifiers::SqlIdentifiers,
    table: crate::sql_qualified_identifier::SqlQualifiedIdentifier,
}

impl SqlSelectBuilder {
    #[must_use]
    pub fn build(&self) -> crate::query_part_fragment::QueryPartFragment {
        let fixed_len = constants_str::catalog::SELECT
            .len()
            .saturating_add(constants_str::catalog::FROM.len())
            .saturating_add(self.table.get_schema().as_ref().len())
            .saturating_add(constants_str::catalog::DOT.len())
            .saturating_add(self.table.get_table().as_ref().len());
        let columns = self.columns.get_inner().get_inner().as_str();
        let capacity = fixed_len.saturating_add(columns.len());
        let mut query =
            crate::sql_query_text::SqlQueryText::try_from(String::with_capacity(capacity))
                .unwrap_or_else(crate::sql_query_text::SqlQueryText::from);
        query
            .get_inner_mut()
            .push_str(constants_str::catalog::SELECT);
        query.get_inner_mut().push_str(columns);
        query.get_inner_mut().push_str(constants_str::catalog::FROM);
        query
            .get_inner_mut()
            .push_str(self.table.get_schema().as_ref());
        query.get_inner_mut().push('.');
        query
            .get_inner_mut()
            .push_str(self.table.get_table().as_ref());
        crate::query_part_fragment::QueryPartFragment::try_from(String::from(query))
            .unwrap_or_else(crate::query_part_fragment::QueryPartFragment::from)
    }

    #[must_use]
    pub const fn new(
        table: crate::sql_qualified_identifier::SqlQualifiedIdentifier,
        columns: crate::sql_identifiers::SqlIdentifiers,
    ) -> Self {
        Self { columns, table }
    }
}
