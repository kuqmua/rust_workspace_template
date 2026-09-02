#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, proc_macro_newtype::FromInner,
)]
pub(crate) struct TracingHttpClientSpan(tracing::Span);

impl TracingHttpClientSpan {
    pub(super) fn into_inner(self) -> tracing::Span {
        self.0
    }
}
