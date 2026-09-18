#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_new::New,
    proc_macro_getters::Getters,
)]
pub struct AdminAccessSessionsReadRequest {
    #[constructor(order = 3)]
    where_many: Option<crate::admin_where_many::AdminWhereMany>,
    #[constructor(order = 1)]
    pagination: crate::admin_read_page::AdminReadPage,
    #[constructor(order = 0)]
    select: crate::admin_read_access_session_selection::AdminReadAccessSessionSelection,
    #[constructor(order = 2)]
    order_by: crate::admin_read_access_session_order::AdminReadAccessSessionOrder,
}

impl TryFrom<&crate::admin_table_query::AdminTableQuery> for AdminAccessSessionsReadRequest {
    type Error =
        crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError;

    fn try_from(value: &crate::admin_table_query::AdminTableQuery) -> Result<Self, Self::Error> {
        let admin_unsearchable_read_query =
            crate::admin_unsearchable_read_query::AdminUnsearchableReadQuery::try_from(value)?;
        Ok(Self::new(
            crate::admin_read_access_session_selection::AdminReadAccessSessionSelection::default(),
            *admin_unsearchable_read_query.get_pagination(),
            crate::admin_read_access_session_order::AdminReadAccessSessionOrder::new(
                crate::admin_read_access_session_column::AdminReadAccessSessionColumn::CreatedAt(
                    crate::admin_no_body::AdminNoBody,
                ),
                admin_unsearchable_read_query.get_order(),
            ),
            None,
        ))
    }
}
