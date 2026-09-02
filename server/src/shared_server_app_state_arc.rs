#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    proc_macro_newtype::DerefTarget,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
#[borrow]
pub(crate) struct SharedServerAppStateArc(
    std::sync::Arc<server_app_state::server_app_state::ServerAppState<'static>>,
);
impl SharedServerAppStateArc {
    #[allow(
        clippy::single_call_fn,
        reason = "shared application-state construction owns the sole Arc allocation boundary"
    )]
    pub(crate) fn from_state(
        server_app_state: server_app_state::server_app_state::ServerAppState<'static>,
    ) -> Self {
        Self::from(std::sync::Arc::new(server_app_state))
    }
}
