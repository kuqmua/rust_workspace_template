#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::FromInner,
    newtype::IntoInnerFrom,
    generate_accessor::Getters,
)]
pub(crate) struct AxumAdminStateRouter(
    axum::Router<crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc>,
);
