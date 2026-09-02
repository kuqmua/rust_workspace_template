#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, proc_macro_getters::Getters,
)]
pub struct AdminGeneratedAuthLayer {
    state: crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc,
}
impl From<crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc>
    for AdminGeneratedAuthLayer
{
    fn from(value: crate::shared_admin_auth_svc_state_arc::SharedAdminAuthSvcStateArc) -> Self {
        Self { state: value }
    }
}
impl<Service> tower::Layer<Service> for AdminGeneratedAuthLayer {
    type Service = crate::admin_generated_auth_service::AdminGeneratedAuthService<Service>;
    fn layer(&self, inner: Service) -> Self::Service {
        crate::admin_generated_auth_service::AdminGeneratedAuthService::new(
            inner,
            self.get_state().clone(),
        )
    }
}
