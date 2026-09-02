#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    serde::Deserialize,
    serde::Serialize,
    utoipa::ToSchema,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
)]
#[serde(
    from = "crate::admin_bounded_vec::AdminBoundedVec<crate::admin_session_view::AdminSessionView>"
)]
#[schema(value_type = crate::admin_open_api_vec::AdminOpenApiVec<crate::admin_session_view::AdminSessionView, 10_000>)]
pub struct AdminSessionViews(
    crate::admin_bounded_vec::AdminBoundedVec<crate::admin_session_view::AdminSessionView>,
);
impl TryFrom<Vec<crate::admin_session_view::AdminSessionView>> for AdminSessionViews {
    type Error = crate::admin_collection_error::AdminCollectionError;
    fn try_from(
        vec: Vec<crate::admin_session_view::AdminSessionView>,
    ) -> Result<Self, Self::Error> {
        crate::admin_bounded_vec::AdminBoundedVec::try_from(vec).map(Self)
    }
}
impl AdminSessionViews {
    pub(crate) const fn as_slice(&self) -> &[crate::admin_session_view::AdminSessionView] {
        self.0.as_slice()
    }
}
