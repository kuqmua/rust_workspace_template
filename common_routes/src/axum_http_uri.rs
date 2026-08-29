#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, newtype::FromInner)]
pub(crate) struct AxumHttpUri(axum::http::Uri);

impl AxumHttpUri {
    pub(crate) const fn get(&self) -> &axum::http::Uri {
        &self.0
    }
}

impl<State> axum::extract::FromRequestParts<State> for AxumHttpUri
where
    State: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &State,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self::from(parts.uri.clone())))
    }
}
