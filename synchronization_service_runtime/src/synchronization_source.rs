pub trait SynchronizationSource {
    type Error: std::error::Error + Send + Sync + 'static;

    fn read(
        &mut self,
    ) -> impl Future<Output = Result<super::SynchronizationPayload, Self::Error>> + Send;
}
