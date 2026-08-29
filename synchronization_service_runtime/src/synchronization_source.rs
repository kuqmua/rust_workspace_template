pub trait SynchronizationSource {
    type Error: std::error::Error + Send + Sync + 'static;

    fn read(
        &mut self,
    ) -> impl Future<
        Output = Result<crate::synchronization_payload::SynchronizationPayload, Self::Error>,
    > + Send;
}
