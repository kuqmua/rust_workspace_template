#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, proc_macro_newtype::FromInner,
)]
pub(crate) struct NotificationAxumJson(
    notification_service_contract::create_notification_request::CreateNotificationRequest,
);
impl NotificationAxumJson {
    pub(crate) fn into_inner(
        self,
    ) -> notification_service_contract::create_notification_request::CreateNotificationRequest {
        self.0
    }
}
impl axum::extract::FromRequest<crate::notification_state::NotificationState>
    for NotificationAxumJson
{
    type Rejection = crate::create_notification_error::CreateNotificationError;
    async fn from_request(
        request: axum::extract::Request,
        notification_state: &crate::notification_state::NotificationState,
    ) -> Result<Self, Self::Rejection> {
        <axum::Json<notification_service_contract::create_notification_request::CreateNotificationRequest> as axum::extract::FromRequest<crate::notification_state::NotificationState>>::from_request(request, notification_state)
            .await
            .map(|axum::Json(value)| Self::from(value))
            .map_err(|_error| crate::create_notification_error::CreateNotificationError::Validation)
    }
}
