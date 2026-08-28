#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::DerefTarget, newtype::FromInner,
)]
pub(crate) struct SharedServerAppStateArc(
    std::sync::Arc<server_app_state::domain_types::ServerAppState<'static>>,
);
impl SharedServerAppStateArc {
    pub(crate) const fn get(
        &self,
    ) -> &std::sync::Arc<server_app_state::domain_types::ServerAppState<'static>> {
        &self.0
    }
}
