pub trait Transport {
    fn send(
        &self,
        request: super::TransportRequest,
    ) -> impl Future<Output = Result<super::TransportResponse, super::TransportError>> + '_;
}
