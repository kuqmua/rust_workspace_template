#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(super) struct AxumNotificationState<Sender> {
    pub(super) headers: super::HttpNotificationHeaderMap,
    pub(super) state: super::NotificationServiceState<Sender>,
}

impl<Sender> axum::extract::FromRequestParts<super::NotificationServiceState<Sender>>
    for AxumNotificationState<Sender>
where
    Sender: Clone + Send + Sync,
{
    type Rejection = std::convert::Infallible;

    fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &super::NotificationServiceState<Sender>,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self {
            headers: super::HttpNotificationHeaderMap::from(parts.headers.clone()),
            state: state.clone(),
        }))
    }
}
