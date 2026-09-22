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
#[serde(from = "[crate::admin_read_role_rule_column::AdminReadRoleRuleColumn; 4]")]
pub struct AdminReadRoleRuleSelection(
    [crate::admin_read_role_rule_column::AdminReadRoleRuleColumn; 4],
);

impl Default for AdminReadRoleRuleSelection {
    fn default() -> Self {
        Self::from(std::array::from_fn(|index| match index {
            0 => crate::admin_read_role_rule_column::AdminReadRoleRuleColumn::Id(
                crate::admin_no_body::AdminNoBody,
            ),
            1 => crate::admin_read_role_rule_column::AdminReadRoleRuleColumn::RoleId(
                crate::admin_no_body::AdminNoBody,
            ),
            2 => crate::admin_read_role_rule_column::AdminReadRoleRuleColumn::RuleId(
                crate::admin_no_body::AdminNoBody,
            ),
            _ => crate::admin_read_role_rule_column::AdminReadRoleRuleColumn::CreatedAt(
                crate::admin_no_body::AdminNoBody,
            ),
        }))
    }
}
