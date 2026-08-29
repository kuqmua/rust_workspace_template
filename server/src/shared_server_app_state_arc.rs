#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, newtype::DerefTarget, newtype::FromInner,
)]
pub(crate) struct SharedServerAppStateArc(
    std::sync::Arc<server_app_state::server_app_state::ServerAppState<'static>>,
);
impl SharedServerAppStateArc {
    #[allow(
        clippy::single_call_fn,
        reason = "shared application-state construction owns the sole Arc allocation boundary"
    )]
    pub(crate) fn from_state(
        state: server_app_state::server_app_state::ServerAppState<'static>,
    ) -> Self {
        Self::from(std::sync::Arc::new(state))
    }

    pub(crate) const fn get(
        &self,
    ) -> &std::sync::Arc<server_app_state::server_app_state::ServerAppState<'static>> {
        &self.0
    }
}
