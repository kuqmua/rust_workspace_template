#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "split owner modules expose representation only within the crate"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySortOrder {
    Ascending,
    Descending,
}

impl QuerySortOrder {
    pub(crate) fn sql(self) -> crate::domain_types::SqlSortOrderText {
        crate::domain_types::SqlSortOrderText::from(match self {
            Self::Ascending => constants_str::SORT_ASC,
            Self::Descending => constants_str::SORT_DESC,
        })
    }
}
