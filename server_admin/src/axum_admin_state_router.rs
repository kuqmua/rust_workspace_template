#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
    proc_macro_getters::Getters,
)]
pub(crate) struct AxumAdminStateRouter(
    axum::Router<crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc>,
);
