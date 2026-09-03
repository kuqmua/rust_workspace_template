#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_get_inner::GetInner,
)]
#[borrow]
pub(crate) struct AxumHttpUri(axum::http::Uri);

impl<State> axum::extract::FromRequestParts<State> for AxumHttpUri
where
    State: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    #[allow(
        unused_variables,
        reason = "the extractor trait implementation preserves type-based parameter names"
    )]
    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &State,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> {
        std::future::ready(Ok(Self::from(parts.uri.clone())))
    }
}
