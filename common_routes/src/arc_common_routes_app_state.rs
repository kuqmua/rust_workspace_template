#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]

#[derive(Clone, optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub struct ArcCommonRoutesAppState(
    pub(super) std::sync::Arc<dyn crate::common_routes_parameters::CommonRoutesParameters>,
);
impl ArcCommonRoutesAppState {
    pub(crate) fn get(&self) -> &dyn crate::common_routes_parameters::CommonRoutesParameters {
        self.0.as_ref()
    }
}
impl std::fmt::Debug for ArcCommonRoutesAppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(constants_str::catalog::STDARCCOMMONROUTESAPPSTATE)
            .finish()
    }
}
impl axum::extract::FromRequestParts<Self> for ArcCommonRoutesAppState {
    type Rejection = std::convert::Infallible;
    fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &Self,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(state.clone()))
    }
}
impl<AppStateTy> From<std::sync::Arc<AppStateTy>> for ArcCommonRoutesAppState
where
    AppStateTy: crate::common_routes_parameters::CommonRoutesParameters + 'static,
{
    fn from(value: std::sync::Arc<AppStateTy>) -> Self {
        Self(value)
    }
}
