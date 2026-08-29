#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct RemoteSyncSource {
    payload: synchronization_service_runtime::SynchronizationPayload,
    request_count: super::RemoteSyncRequestCount,
}

impl RemoteSyncSource {
    #[must_use]
    pub fn new(payload: synchronization_service_runtime::SynchronizationPayload) -> Self {
        Self {
            payload,
            request_count: super::RemoteSyncRequestCount::from(constants_usize::ZERO),
        }
    }

    #[must_use]
    pub const fn request_count(&self) -> super::RemoteSyncRequestCount {
        self.request_count
    }
}

impl synchronization_service_runtime::SynchronizationSource for RemoteSyncSource {
    type Error = std::convert::Infallible;

    fn read(
        &mut self,
    ) -> impl Future<
        Output = Result<synchronization_service_runtime::SynchronizationPayload, Self::Error>,
    > + Send {
        self.request_count.0 = self.request_count.0.saturating_add(constants_usize::ONE);
        std::future::ready(Ok(self.payload.clone()))
    }
}
