#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumNotificationRouter(axum::Router);
impl AxumNotificationRouter {
    pub(crate) fn into_inner(self) -> axum::Router {
        self.0
    }
}
