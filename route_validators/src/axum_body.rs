#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct AxumBody(axum::body::Body);

impl AxumBody {
    pub(crate) fn into_inner(self) -> axum::body::Body {
        self.0
    }

    pub(crate) fn size_hint(&self) -> http_body::SizeHint {
        axum::body::HttpBody::size_hint(&self.0)
    }
}
