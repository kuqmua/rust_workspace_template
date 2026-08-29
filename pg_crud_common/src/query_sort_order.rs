#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySortOrder {
    Ascending,
    Descending,
}

impl QuerySortOrder {
    pub(crate) fn sql(self) -> crate::sql_sort_order_text::SqlSortOrderText {
        crate::sql_sort_order_text::SqlSortOrderText::from(match self {
            Self::Ascending => constants_str::test_fixtures::SORT_ASC,
            Self::Descending => constants_str::test_fixtures::SORT_DESC,
        })
    }
}
