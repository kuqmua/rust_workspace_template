#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
#[borrow]
pub(crate) struct NotificationAxumState(crate::notification_state::NotificationState);
impl axum::extract::FromRequestParts<crate::notification_state::NotificationState>
    for NotificationAxumState
{
    type Rejection = crate::http_notification_status_code::HttpNotificationStatusCode;
    #[allow(
        unused_variables,
        reason = "the extractor trait implementation preserves type-based parameter names"
    )]
    fn from_request_parts(
        parts: &mut http::request::Parts,
        notification_state: &crate::notification_state::NotificationState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self::from(notification_state.clone())))
    }
}
