#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(crate) struct NotificationAxumState(crate::notification_state::NotificationState);
impl NotificationAxumState {
    pub(crate) const fn get(&self) -> &crate::notification_state::NotificationState {
        &self.0
    }
}
impl axum::extract::FromRequestParts<crate::notification_state::NotificationState>
    for NotificationAxumState
{
    type Rejection = crate::http_notification_status_code::HttpNotificationStatusCode;
    fn from_request_parts(
        _parts: &mut http::request::Parts,
        state: &crate::notification_state::NotificationState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self::from(state.clone())))
    }
}
