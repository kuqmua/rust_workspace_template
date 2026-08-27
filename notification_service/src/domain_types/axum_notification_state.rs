use super::{HttpNotificationStatusCode, NotificationState};

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(crate) struct AxumNotificationState(NotificationState);
impl AxumNotificationState {
    pub(crate) const fn get(&self) -> &NotificationState {
        &self.0
    }
}
impl axum::extract::FromRequestParts<NotificationState> for AxumNotificationState {
    type Rejection = HttpNotificationStatusCode;
    fn from_request_parts(
        _parts: &mut http::request::Parts,
        state: &NotificationState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self::from(state.clone())))
    }
}
