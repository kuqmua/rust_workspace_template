#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(super) struct AxumNotificationJson(pub(super) super::NotificationRequest);

impl<State> axum::extract::FromRequest<State> for AxumNotificationJson
where
    State: Send + Sync,
{
    type Rejection = axum::extract::rejection::JsonRejection;

    async fn from_request(
        req: axum::extract::Request,
        state: &State,
    ) -> Result<Self, Self::Rejection> {
        axum::Json::<super::NotificationRequest>::from_request(req, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
    }
}
