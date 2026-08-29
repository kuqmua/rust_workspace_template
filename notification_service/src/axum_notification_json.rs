#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumNotificationJson(
    notification_service_contract::create_notification_req::CreateNotificationReq,
);
impl AxumNotificationJson {
    pub(crate) fn into_inner(
        self,
    ) -> notification_service_contract::create_notification_req::CreateNotificationReq {
        self.0
    }
}
impl axum::extract::FromRequest<crate::notification_state::NotificationState>
    for AxumNotificationJson
{
    type Rejection = crate::create_notification_error::CreateNotificationError;
    async fn from_request(
        req: axum::extract::Request,
        state: &crate::notification_state::NotificationState,
    ) -> Result<Self, Self::Rejection> {
        <axum::Json<notification_service_contract::create_notification_req::CreateNotificationReq> as axum::extract::FromRequest<crate::notification_state::NotificationState>>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
            .map_err(|_error| crate::create_notification_error::CreateNotificationError::Validation)
    }
}
