#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct AxumNotificationState<Sender> {
    pub(super) headers: crate::http_notification_header_map::HttpNotificationHeaderMap,
    pub(super) state: crate::notification_service_state::NotificationServiceState<Sender>,
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
