#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    proc_macro_new::New,
    proc_macro_getters::Getters,
)]
pub struct AdminUnsearchableReadQuery {
    pagination: crate::admin_read_page::AdminReadPage,
    #[getters(copy)]
    order: crate::admin_sort_direction::AdminSortDirection,
}

impl TryFrom<&crate::admin_table_query::AdminTableQuery> for AdminUnsearchableReadQuery {
    type Error =
        crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError;

    fn try_from(value: &crate::admin_table_query::AdminTableQuery) -> Result<Self, Self::Error> {
        if !value.sort().as_ref().is_empty() {
            return Err(
                crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown,
            );
        }
        Ok(Self::new(
            crate::admin_read_page::AdminReadPage::new(value.offset(), value.limit()),
            crate::admin_sort_direction::AdminSortDirection::Descending,
        ))
    }
}
