#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    proc_macro_newtype_from_inner::FromInner,
)]
#[serde(from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_rule_id::AdminRuleId>")]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_rule_id::AdminRuleId, 10_000>)]
pub struct AdminRuleIds(
    crate::admin_bounded_vec::AdminBoundedVec<crate::admin_rule_id::AdminRuleId>,
);
impl TryFrom<Vec<crate::admin_rule_id::AdminRuleId>> for AdminRuleIds {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(value: Vec<crate::admin_rule_id::AdminRuleId>) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(value).map(Self)
    }
}
#[allow(
    clippy::derivable_impls,
    reason = "only identifier request collections intentionally expose Default"
)]
impl Default for AdminRuleIds {
    fn default() -> Self {
        Self::from(crate::admin_empty_collection::AdminEmptyCollection)
    }
}
impl From<crate::admin_empty_collection::AdminEmptyCollection> for AdminRuleIds {
    fn from(value: crate::admin_empty_collection::AdminEmptyCollection) -> Self {
        let _: crate::admin_empty_collection::AdminEmptyCollection = value;
        Self(crate::admin_bounded_vec::AdminBoundedVec::from([]))
    }
}
