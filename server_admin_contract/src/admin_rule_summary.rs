#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    proc_macro_new::New,
)]
pub struct AdminRuleSummary {
    #[getters(copy)]
    id: crate::admin_rule_id::AdminRuleId,
    name: crate::admin_rule_value::AdminRuleValue,
    created_at: crate::admin_rule_timestamp::AdminRuleTimestamp,
}
