#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct AxumNotificationJson(
    notification_service_contract::domain_types::CreateNotificationReq,
);
impl AxumNotificationJson {
    pub(crate) fn into_inner(
        self,
    ) -> notification_service_contract::domain_types::CreateNotificationReq {
        self.0
    }
}
impl axum::extract::FromRequest<super::NotificationState> for AxumNotificationJson {
    type Rejection = super::CreateNotificationError;
    async fn from_request(
        req: axum::extract::Request,
        state: &super::NotificationState,
    ) -> Result<Self, Self::Rejection> {
        <axum::Json<notification_service_contract::domain_types::CreateNotificationReq> as axum::extract::FromRequest<super::NotificationState>>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
            .map_err(|_error| super::CreateNotificationError::Validation)
    }
}
