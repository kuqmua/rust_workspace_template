#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::GetInner,
)]
#[borrow]
pub(crate) struct NotificationAxumState(crate::notification_state::NotificationState);
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
