#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub(in super::super) struct TracingHttpClientSpan(tracing::Span);

impl TracingHttpClientSpan {
    pub(super) fn into_inner(self) -> tracing::Span {
        self.0
    }
}
