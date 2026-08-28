#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct AdminGeneratedAuthLayer {
    state: crate::domain_types::auth::SharedAdminAuthSvcStateArc,
}
impl From<crate::domain_types::auth::SharedAdminAuthSvcStateArc> for AdminGeneratedAuthLayer {
    fn from(value: crate::domain_types::auth::SharedAdminAuthSvcStateArc) -> Self {
        Self { state: value }
    }
}
impl<Service> tower::Layer<Service> for AdminGeneratedAuthLayer {
    type Service = crate::admin_generated_auth_service::AdminGeneratedAuthService<Service>;
    fn layer(&self, inner: Service) -> Self::Service {
        crate::admin_generated_auth_service::AdminGeneratedAuthService {
            inner,
            state: self.state.clone(),
        }
    }
}
