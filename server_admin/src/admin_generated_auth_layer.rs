#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, proc_macro_getters::Getters,
)]
pub struct AdminGeneratedAuthLayer {
    state: crate::shared_admin_auth_service_state_arc::SharedAdminAuthServiceStateArc,
}
impl From<crate::shared_admin_auth_service_state_arc::SharedAdminAuthServiceStateArc>
    for AdminGeneratedAuthLayer
{
    fn from(
        value: crate::shared_admin_auth_service_state_arc::SharedAdminAuthServiceStateArc,
    ) -> Self {
        Self { state: value }
    }
}
impl<Service> tower::Layer<Service> for AdminGeneratedAuthLayer {
    type Service = crate::admin_generated_auth_service::AdminGeneratedAuthService<Service>;
    fn layer(&self, service: Service) -> Self::Service {
        crate::admin_generated_auth_service::AdminGeneratedAuthService::new(
            service,
            self.get_state().clone(),
        )
    }
}
