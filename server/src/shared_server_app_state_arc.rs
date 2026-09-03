#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    proc_macro_newtype_deref_target::DerefTarget,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
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
