#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(crate) struct AxumNotificationState(super::NotificationState);
impl AxumNotificationState {
    pub(crate) const fn get(&self) -> &super::NotificationState {
        &self.0
    }
}
impl axum::extract::FromRequestParts<super::NotificationState> for AxumNotificationState {
    type Rejection = super::HttpNotificationStatusCode;
    fn from_request_parts(
        _parts: &mut http::request::Parts,
        state: &super::NotificationState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self::from(state.clone())))
    }
}
