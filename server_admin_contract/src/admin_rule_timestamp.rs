#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_display::Display,
    utoipa::ToSchema,
)]
#[serde(from = "crate::admin_role_timestamp::AdminRoleTimestamp")]
pub struct AdminRuleTimestamp(crate::admin_role_timestamp::AdminRoleTimestamp);
