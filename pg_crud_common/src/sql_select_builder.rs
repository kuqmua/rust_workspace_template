#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct SqlSelectBuilder {
    columns: crate::domain_types::SqlIdentifiers,
    table: crate::domain_types::SqlQualifiedIdentifier,
}

impl SqlSelectBuilder {
    #[must_use]
    pub fn build(&self) -> crate::domain_types::QueryPartFragment {
        let fixed_len = constants_str::SELECT
            .len()
            .saturating_add(constants_str::FROM.len())
            .saturating_add(self.table.get_schema().as_ref().len())
            .saturating_add(constants_str::DOT.len())
            .saturating_add(self.table.get_table().as_ref().len());
        let columns = self.columns.get_inner().get_inner().as_str();
        let capacity = fixed_len.saturating_add(columns.len());
        let mut query =
            crate::domain_types::SqlQueryText::try_from(String::with_capacity(capacity))
                .unwrap_or_else(crate::domain_types::SqlQueryText::from);
        query.get_inner_mut().push_str(constants_str::SELECT);
        query.get_inner_mut().push_str(columns);
        query.get_inner_mut().push_str(constants_str::FROM);
        query
            .get_inner_mut()
            .push_str(self.table.get_schema().as_ref());
        query.get_inner_mut().push('.');
        query
            .get_inner_mut()
            .push_str(self.table.get_table().as_ref());
        crate::domain_types::QueryPartFragment::try_from(String::from(query))
            .unwrap_or_else(crate::domain_types::QueryPartFragment::from)
    }

    #[must_use]
    pub const fn new(
        table: crate::domain_types::SqlQualifiedIdentifier,
        columns: crate::domain_types::SqlIdentifiers,
    ) -> Self {
        Self { columns, table }
    }
}
