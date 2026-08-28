#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(crate) struct TracingHttpClientSpan(tracing::Span);

impl TracingHttpClientSpan {
    pub(super) fn into_inner(self) -> tracing::Span {
        self.0
    }
}
