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
pub struct AdminAuditLogReadRequest {
    #[constructor(order = 4)]
    where_many: Option<crate::admin_where_many::AdminWhereMany>,
    #[constructor(order = 0)]
    search: Option<crate::admin_table_search::AdminTableSearch>,
    #[constructor(order = 2)]
    pagination: crate::admin_read_page::AdminReadPage,
    #[constructor(order = 1)]
    select: crate::admin_read_audit_log_selection::AdminReadAuditLogSelection,
    #[constructor(order = 3)]
    order_by: crate::admin_read_audit_log_order::AdminReadAuditLogOrder,
}

impl TryFrom<&crate::admin_table_query::AdminTableQuery> for AdminAuditLogReadRequest {
    type Error =
        crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError;

    fn try_from(value: &crate::admin_table_query::AdminTableQuery) -> Result<Self, Self::Error> {
        let column = if value.sort().as_ref().is_empty() {
            crate::admin_read_audit_log_column::AdminReadAuditLogColumn::CreatedAt(
                crate::admin_no_body::AdminNoBody,
            )
        } else {
            match crate::admin_table_sort_field::AdminTableSortField::try_from_key(
                &crate::admin_table_sort_field::AdminTableSortField::AUDIT,
                crate::admin_table_sort_key_ref::AdminTableSortKeyRef::from(value.sort().as_ref()),
            )? {
                crate::admin_table_sort_field::AdminTableSortField::AuditAction => {
                    crate::admin_read_audit_log_column::AdminReadAuditLogColumn::Action(
                        crate::admin_no_body::AdminNoBody,
                    )
                }
                crate::admin_table_sort_field::AdminTableSortField::AuditCreatedAt => {
                    crate::admin_read_audit_log_column::AdminReadAuditLogColumn::CreatedAt(
                        crate::admin_no_body::AdminNoBody,
                    )
                }
                crate::admin_table_sort_field::AdminTableSortField::AuditResource => {
                    crate::admin_read_audit_log_column::AdminReadAuditLogColumn::Resource(
                        crate::admin_no_body::AdminNoBody,
                    )
                }
                crate::admin_table_sort_field::AdminTableSortField::AuditSucceeded => {
                    crate::admin_read_audit_log_column::AdminReadAuditLogColumn::Succeeded(
                        crate::admin_no_body::AdminNoBody,
                    )
                }
                crate::admin_table_sort_field::AdminTableSortField::AuditUserId => {
                    crate::admin_read_audit_log_column::AdminReadAuditLogColumn::UserId(
                        crate::admin_no_body::AdminNoBody,
                    )
                }
                crate::admin_table_sort_field::AdminTableSortField::RuleCreatedAt
                | crate::admin_table_sort_field::AdminTableSortField::RuleId
                | crate::admin_table_sort_field::AdminTableSortField::RuleName
                | crate::admin_table_sort_field::AdminTableSortField::RoleId
                | crate::admin_table_sort_field::AdminTableSortField::RoleName
                | crate::admin_table_sort_field::AdminTableSortField::RoleSystem
                | crate::admin_table_sort_field::AdminTableSortField::UserDisplayName
                | crate::admin_table_sort_field::AdminTableSortField::UserId
                | crate::admin_table_sort_field::AdminTableSortField::UserLogin
                | crate::admin_table_sort_field::AdminTableSortField::UserStatus => {
                    return Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown);
                }
            }
        };
        let order = if value.sort().as_ref().is_empty() {
            crate::admin_sort_direction::AdminSortDirection::Descending
        } else {
            value.direction()
        };
        Ok(Self::new(
            Some(value.search().clone()),
            crate::admin_read_audit_log_selection::AdminReadAuditLogSelection::default(),
            crate::admin_read_page::AdminReadPage::new(value.offset(), value.limit()),
            crate::admin_read_audit_log_order::AdminReadAuditLogOrder::new(column, order),
            None,
        ))
    }
}
