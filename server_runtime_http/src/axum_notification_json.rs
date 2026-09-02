#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInner,
)]
pub(super) struct AxumNotificationJson(crate::notification_request::NotificationRequest);

impl<State> axum::extract::FromRequest<State> for AxumNotificationJson
where
    State: Send + Sync,
{
    type Rejection = axum::extract::rejection::JsonRejection;

    async fn from_request(
        request: axum::extract::Request,
        state: &State,
    ) -> Result<Self, Self::Rejection> {
        axum::Json::<crate::notification_request::NotificationRequest>::from_request(request, state)
            .await
            .map(|axum::Json(value)| Self::from(value))
    }
}
