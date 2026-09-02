#[derive(
    Clone, proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_newtype::FromInner,
)]
pub struct ArcCommonRoutesAppState(
    std::sync::Arc<dyn crate::common_routes_parameters::CommonRoutesParameters>,
);
impl ArcCommonRoutesAppState {
    pub(crate) fn get(&self) -> &dyn crate::common_routes_parameters::CommonRoutesParameters {
        self.0.as_ref()
    }
}
impl std::fmt::Debug for ArcCommonRoutesAppState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple(constants_str::STDARCCOMMONROUTESAPPSTATE)
            .finish()
    }
}
impl axum::extract::FromRequestParts<Self> for ArcCommonRoutesAppState {
    type Rejection = std::convert::Infallible;
    #[allow(
        unused_variables,
        reason = "the extractor trait implementation preserves type-based parameter names"
    )]
    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Self,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(state.clone()))
    }
}
impl<AppStateTy> From<std::sync::Arc<AppStateTy>> for ArcCommonRoutesAppState
where
    AppStateTy: crate::common_routes_parameters::CommonRoutesParameters + 'static,
{
    fn from(arc: std::sync::Arc<AppStateTy>) -> Self {
        Self(arc)
    }
}
