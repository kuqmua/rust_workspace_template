pub trait Transport {
    fn send(
        &self,
        transport_request: crate::transport_request::TransportRequest,
    ) -> impl Future<
        Output = Result<
            crate::transport_response::TransportResponse,
            crate::transport_error::TransportError,
        >,
    > + '_;
}
