#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInner,
)]
pub struct AxumBody(axum::body::Body);

impl AxumBody {
    pub(crate) fn size_hint(&self) -> http_body::SizeHint {
        axum::body::HttpBody::size_hint(&self.0)
    }
}
