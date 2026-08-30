#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct NotificationAxumRouter(axum::Router);
impl NotificationAxumRouter {
    pub(crate) fn into_inner(self) -> axum::Router {
        self.0
    }
}
