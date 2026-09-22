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
#[serde(
    from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_rule_summary::AdminRuleSummary>"
)]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_rule_summary::AdminRuleSummary, 10_000>)]
pub struct AdminRuleSummaries(
    crate::admin_bounded_vec::AdminBoundedVec<crate::admin_rule_summary::AdminRuleSummary>,
);
impl TryFrom<Vec<crate::admin_rule_summary::AdminRuleSummary>> for AdminRuleSummaries {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(
        value: Vec<crate::admin_rule_summary::AdminRuleSummary>,
    ) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(value).map(Self)
    }
}
impl AdminRuleSummaries {
    pub(crate) const fn as_slice(&self) -> &[crate::admin_rule_summary::AdminRuleSummary] {
        self.0.as_slice()
    }
}
