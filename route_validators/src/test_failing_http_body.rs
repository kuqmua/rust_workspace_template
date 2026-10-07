#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct TestFailingHttpBody;

impl http_body::Body for TestFailingHttpBody {
    type Data = bytes::Bytes;
    type Error = std::io::Error;

    fn poll_frame(
        self: std::pin::Pin<&mut Self>,
        _context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<http_body::Frame<Self::Data>, Self::Error>>> {
        std::task::Poll::Ready(Some(Err(std::io::Error::other(constants_str::X))))
    }
}
