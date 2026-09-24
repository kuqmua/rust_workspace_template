#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    proc_macro_newtype_from_inner::FromInner,
    utoipa::ToSchema,
)]
#[serde(from = "[crate::admin_read_rule_column::AdminReadRuleColumn; 13]")]
pub struct AdminReadRuleSelection([crate::admin_read_rule_column::AdminReadRuleColumn; 13]);

impl Default for AdminReadRuleSelection {
    fn default() -> Self {
        let empty = crate::admin_no_body::AdminNoBody;
        Self::from([
            crate::admin_read_rule_column::AdminReadRuleColumn::Id(empty),
            crate::admin_read_rule_column::AdminReadRuleColumn::PermissionResourceActionId(empty),
            crate::admin_read_rule_column::AdminReadRuleColumn::BasemapId(empty),
            crate::admin_read_rule_column::AdminReadRuleColumn::LayerGroupId(empty),
            crate::admin_read_rule_column::AdminReadRuleColumn::LayerId(empty),
            crate::admin_read_rule_column::AdminReadRuleColumn::ProjectGroupId(empty),
            crate::admin_read_rule_column::AdminReadRuleColumn::ProjectId(empty),
            crate::admin_read_rule_column::AdminReadRuleColumn::PropertyId(empty),
            crate::admin_read_rule_column::AdminReadRuleColumn::RoleId(empty),
            crate::admin_read_rule_column::AdminReadRuleColumn::UserId(empty),
            crate::admin_read_rule_column::AdminReadRuleColumn::FeatureId(empty),
            crate::admin_read_rule_column::AdminReadRuleColumn::ValueItemId(empty),
            crate::admin_read_rule_column::AdminReadRuleColumn::CreatedAt(empty),
        ])
    }
}
