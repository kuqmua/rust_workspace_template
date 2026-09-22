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
#[serde(from = "[crate::admin_read_rule_column::AdminReadRuleColumn; 3]")]
pub struct AdminReadRuleSelection([crate::admin_read_rule_column::AdminReadRuleColumn; 3]);
impl Default for AdminReadRuleSelection {
    fn default() -> Self {
        Self::from([
            crate::admin_read_rule_column::AdminReadRuleColumn::Id(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_rule_column::AdminReadRuleColumn::Name(
                crate::admin_no_body::AdminNoBody,
            ),
            crate::admin_read_rule_column::AdminReadRuleColumn::CreatedAt(
                crate::admin_no_body::AdminNoBody,
            ),
        ])
    }
}
