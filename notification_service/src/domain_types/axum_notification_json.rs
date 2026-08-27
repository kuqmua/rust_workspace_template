use super::{CreateNotificationError, NotificationState};

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
impl axum::extract::FromRequest<NotificationState> for AxumNotificationJson {
    type Rejection = CreateNotificationError;
    async fn from_request(
        req: axum::extract::Request,
        state: &NotificationState,
    ) -> Result<Self, Self::Rejection> {
        <axum::Json<notification_service_contract::domain_types::CreateNotificationReq> as axum::extract::FromRequest<NotificationState>>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
            .map_err(|_error| CreateNotificationError::Validation)
    }
}
