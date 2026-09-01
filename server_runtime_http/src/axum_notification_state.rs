#[derive(generate_accessor::Getters)]
#[getters(bare)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct AxumNotificationState<Sender> {
    headers: crate::http_notification_header_map::HttpNotificationHeaderMap,
    state: crate::notification_service_state::NotificationServiceState<Sender>,
}

impl<Sender>
    axum::extract::FromRequestParts<
        crate::notification_service_state::NotificationServiceState<Sender>,
    > for AxumNotificationState<Sender>
where
    Sender: Clone + Send + Sync,
{
    type Rejection = std::convert::Infallible;

    fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &crate::notification_service_state::NotificationServiceState<Sender>,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self {
            headers: crate::http_notification_header_map::HttpNotificationHeaderMap::from(
                parts.headers.clone(),
            ),
            state: state.clone(),
        }))
    }
}
