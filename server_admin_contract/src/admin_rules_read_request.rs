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
pub struct AdminRulesReadRequest {
    #[constructor(order = 3)]
    where_many: Option<crate::admin_where_many::AdminWhereMany>,
    #[constructor(order = 0)]
    pagination: crate::admin_read_page::AdminReadPage,
    #[constructor(order = 1)]
    select: crate::admin_read_rule_selection::AdminReadRuleSelection,
    #[constructor(order = 2)]
    order_by: crate::admin_read_rule_order::AdminReadRuleOrder,
}
impl TryFrom<&crate::admin_table_query::AdminTableQuery> for AdminRulesReadRequest {
    type Error =
        crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError;
    fn try_from(value: &crate::admin_table_query::AdminTableQuery) -> Result<Self, Self::Error> {
        let column = match value.sort().as_ref() {
            constants_str::EMPTY | constants_str::SQL_NAMES_ID => crate::admin_read_rule_column::AdminReadRuleColumn::Id(crate::admin_no_body::AdminNoBody),
            constants_str::PERMISSION_RESOURCE_ACTION_ID => crate::admin_read_rule_column::AdminReadRuleColumn::PermissionResourceActionId(crate::admin_no_body::AdminNoBody),
            constants_str::BASEMAP_ID => crate::admin_read_rule_column::AdminReadRuleColumn::BasemapId(crate::admin_no_body::AdminNoBody),
            constants_str::LAYER_GROUP_ID => crate::admin_read_rule_column::AdminReadRuleColumn::LayerGroupId(crate::admin_no_body::AdminNoBody),
            constants_str::LAYER_ID => crate::admin_read_rule_column::AdminReadRuleColumn::LayerId(crate::admin_no_body::AdminNoBody),
            constants_str::PROJECT_GROUP_ID => crate::admin_read_rule_column::AdminReadRuleColumn::ProjectGroupId(crate::admin_no_body::AdminNoBody),
            constants_str::PROJECT_ID => crate::admin_read_rule_column::AdminReadRuleColumn::ProjectId(crate::admin_no_body::AdminNoBody),
            constants_str::PROPERTY_ID => crate::admin_read_rule_column::AdminReadRuleColumn::PropertyId(crate::admin_no_body::AdminNoBody),
            constants_str::ROLE_ID => crate::admin_read_rule_column::AdminReadRuleColumn::RoleId(crate::admin_no_body::AdminNoBody),
            constants_str::USER_ID => crate::admin_read_rule_column::AdminReadRuleColumn::UserId(crate::admin_no_body::AdminNoBody),
            constants_str::FEATURE_ID => crate::admin_read_rule_column::AdminReadRuleColumn::FeatureId(crate::admin_no_body::AdminNoBody),
            constants_str::VALUE_ITEM_ID => crate::admin_read_rule_column::AdminReadRuleColumn::ValueItemId(crate::admin_no_body::AdminNoBody),
            constants_str::CREATED_AT => crate::admin_read_rule_column::AdminReadRuleColumn::CreatedAt(crate::admin_no_body::AdminNoBody),
            _ => return Err(crate::admin_table_sort_field_try_from_key_error::AdminTableSortFieldTryFromKeyError::Unknown),
        };
        Ok(Self::new(
            crate::admin_read_page::AdminReadPage::new(value.offset(), value.limit()),
            crate::admin_read_rule_selection::AdminReadRuleSelection::default(),
            crate::admin_read_rule_order::AdminReadRuleOrder::new(column, value.direction()),
            None,
        ))
    }
}
