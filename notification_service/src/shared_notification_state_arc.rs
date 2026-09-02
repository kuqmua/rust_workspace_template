#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, proc_macro_newtype::FromInner,
)]
pub(crate) struct SharedNotificationStateArc(
    std::sync::Arc<crate::notification_state::NotificationState>,
);

impl SharedNotificationStateArc {
    #[must_use]
    #[allow(
        clippy::single_call_fn,
        reason = "shared-state construction owns the sole Arc allocation boundary"
    )]
    pub(crate) fn from_state(state: crate::notification_state::NotificationState) -> Self {
        Self::from(std::sync::Arc::new(state))
    }

    #[must_use]
    pub(crate) fn into_common_routes_app_state(
        self,
    ) -> common_routes::arc_common_routes_app_state::ArcCommonRoutesAppState {
        let state: std::sync::Arc<
            dyn common_routes::common_routes_parameters::CommonRoutesParameters,
        > = self.0;
        common_routes::arc_common_routes_app_state::ArcCommonRoutesAppState::from(state)
    }
}
